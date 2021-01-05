use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use rust_code_analysis::{
    action, guess_language, AstCallback, AstCfg, AstPayload, AstResponse, Span, LANG,
};

use crate::{lang::*, *};

/// Information on an `AST` node.
/// Taken directly from the `rust_code_analysis` crate with additional serde macros for deserialization.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct AST {
    /// The type of node
    pub r#type: String,
    /// The code associated to a node
    #[serde(rename(deserialize = "TextValue"))]
    pub value: String,
    /// The start and end positions of a node in a code
    pub span: Span,
    /// The children of a node
    pub children: Vec<AST>,
}

impl AST {
    pub fn find_child_by_type(&self, types: &[&str]) -> Option<&AST> {
        self.children
            .iter()
            .find(|child| types.iter().find(|t| &*child.r#type == **t).is_some())
    }

    pub fn find_child_by_value(&self, value: &str) -> Option<&AST> {
        self.children.iter().find(|child| child.value == value)
    }
}

/// Parse the given source code from the `AstPayload`
pub fn parse_ast(payload: AstPayload) -> Option<(AST, LANG)> {
    let file = payload.file_name;
    let buf = payload.code.into_bytes();
    let (language, _ext) = guess_language(&buf, &file);
    let cfg = AstCfg {
        id: payload.id,
        comment: payload.comment,
        span: payload.span,
    };
    Some((
        action::<AstCallback>(&language?, buf, &PathBuf::from(""), None, cfg).into(),
        language?,
    ))
}

impl From<AstResponse> for AST {
    fn from(ast_response: AstResponse) -> Self {
        let value = serde_json::to_value(ast_response).unwrap();
        let root = value
            .as_object()
            .unwrap()
            .get("root")
            .expect(r#"AstResponse was missing "root" field"#)
            .to_owned();
        serde_json::from_value::<AST>(root).expect("could not deserialize AstResponse into AST")
    }
}

impl AST {
    /// Transform the language-specific AST into generic components.
    pub fn transform(self, lang: LANG, path: &str) -> Vec<ComponentType> {
        // Do language specific AST parsing
        match lang {
            LANG::Cpp => cpp::find_components(self, path, path),
            LANG::Java => {
                todo!();
            }
            LANG::Python => {
                todo!();
            }
            LANG::Go => {
                todo!();
            }
            lang => {
                println!("unsupported lang: {:?}", lang);
                todo!();
            }
        }
    }
}

pub fn parse_project_context(root_path: &Path) -> std::io::Result<JSSAContext> {
    let path_str = root_path.to_str().unwrap_or("");
    let modules = parse_directory(&root_path)?;
    let ctx = JSSAContext {
        component: ComponentInfo {
            path: path_str.to_string(),
            package_name: "".to_string(),
            instance_name: "context".to_string(),
            instance_type: InstanceType::AnalysisComponent,
        },
        succeeded: true,
        root_path: path_str,
        modules,
    };
    Ok(ctx)
}

fn flatten_dirs(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    if dir.is_dir() {
        let read_dir = std::fs::read_dir(dir.clone())?;
        let mut dirs: Vec<PathBuf> = read_dir
            .into_iter()
            .filter(|entry| entry.is_ok())
            .map(|entry| entry.unwrap())
            .filter(|entry| entry.path().is_dir())
            .map(|entry| entry.path().as_path().to_owned())
            .collect();

        for path in dirs.clone() {
            let sub_dirs = flatten_dirs(path.as_path())?;
            sub_dirs.into_iter().for_each(|dir| dirs.push(dir));
        }

        dirs.push(dir.to_path_buf());
        Ok(dirs)
    } else {
        Ok(vec![])
    }
}

pub fn parse_directory(dir: &Path) -> std::io::Result<Vec<ModuleComponent>> {
    let mut modules = vec![];

    if dir.is_dir() {
        let dirs = flatten_dirs(dir)?;

        for dir in dirs {
            let path = dir.as_path().to_str().unwrap_or("").to_string();
            let path_clone = path.clone();
            let module_name: Vec<&str> = path_clone.split("/").into_iter().collect();
            let module_name = module_name.get(module_name.len() - 1).unwrap_or(&"");

            let read_dir = std::fs::read_dir(dir.clone())?;
            let mut module = ModuleComponent::new(module_name.to_string(), path);

            for entry in read_dir {
                let entry = entry?;
                if !entry.path().is_dir() {
                    let mut file = File::open(entry.path())?;
                    let components = parse_file(&mut file, &entry.path())?;

                    for component in components.into_iter() {
                        match component {
                            ComponentType::ClassOrInterfaceComponent(component) => {
                                match component.declaration_type {
                                    ContainerType::Class => {
                                        module.classes.push(component);
                                    }
                                    ContainerType::Interface => {
                                        module.interfaces.push(component);
                                    }
                                    r#type => {
                                        println!("got other label when it should have been class/ifc: {:#?}", r#type);
                                    }
                                }
                            }
                            ComponentType::MethodComponent(method) => {
                                module.component.methods.push(method);
                            }
                            ComponentType::ModuleComponent(module) => {
                                modules.push(module);
                            }
                            _ => {}
                        }
                    }
                }
            }

            modules.push(module);
        }
    }

    Ok(modules)
}

pub fn parse_file(file: &mut File, path: &Path) -> std::io::Result<Vec<ComponentType>> {
    let mut code = String::new();
    file.read_to_string(&mut code)?;

    let result = parse_ast(AstPayload {
        id: "".to_owned(),
        file_name: path.to_str().unwrap_or("").to_owned(),
        code,
        comment: false,
        span: true,
    });

    let (ast, lang) = match result {
        Some((ast, lang)) => (ast, lang),
        None => return Ok(vec![]),
    };

    Ok(ast.transform(lang, path.to_str().unwrap_or_default()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_util(code: &str, path: &str) -> Option<(AST, LANG)> {
        let payload = AstPayload {
            id: "".to_owned(),
            file_name: path.to_owned(),
            code: code.to_owned(),
            comment: false,
            span: true,
        };
        parse_ast(payload)
    }

    #[test]
    fn parse_go_ast() {
        let code = r#"package main

        import (
            "sourcecrawler/app"
            "sourcecrawler/config"
        )
        
        func main() {
        
            config := config.GetConfig()
        
            app := &app.App{}
            app.Initialize(config)
            app.Run(":3000")
        
        }
        "#;
        let path = "/path/to/go/file/main.go";

        let root_node = parse_util(code, path);

        assert!(root_node.is_some());
    }

    #[test]
    fn parse_rust_ast() {
        let code = r#"
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct MyStruct {
            id: i32,
            r#type: &'static str,
            point: Point
        }

        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Point(i32, i32);

        fn main() {
            println!("{:#?}", MyStruct {
                id: 10,
                type: "hello world",
                point: Point(10, 20)
            });
        }
        "#;
        let path = "/path/to/rust/file/main.rs";

        let root_node = parse_util(code, path);

        assert!(root_node.is_some());
    }

    #[test]
    fn parse_cpp_ast() {
        let code = r#"
        #include <iostream>
        #include <string>

        class MyClass {
        private:
            int id;
            std::string str;
        public:
            MyClass(int id, const char *str) : id(id), str(str) {}
            int getId() { return id; }
            std::string getStr() { return str; }
        };

        int main(int argc, char **argv) {
            MyClass m(10, argv[1]);
            std::cout << argc << std::endl;
            std::cout << m.getId() << " " << m.getStr() << std::endl;
        }
        "#;
        let path = "/path/to/cpp/file/main.cpp";

        let root_node = parse_util(code, path);

        assert!(root_node.is_some());
    }

    #[test]
    fn parse_python_ast() {
        let code = r#"
        from flask import Flask
        
        app = Flask(__name__)
        cors = CORS(app, resources={"/*": {"origins": os.environ.get("CROSS_ORIGIN")}})

        @app.route("/", methods=["GET"])
        def endpoint():
            """
            Documentation...
            """
            
            data = request.json
            count = data["count"]

            if count is None:
                abort(400)
        
            for i in range(count):
                print("hello")
        
            # Return the resulting similarity ratings
            return {"response": "hello"}

        "#;
        let path = "/path/to/py/file/app.py";

        let root_node = parse_util(code, path);

        assert!(root_node.is_some());
    }
}
