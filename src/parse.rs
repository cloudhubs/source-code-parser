use derive_new::new;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use rust_code_analysis::{
    action, guess_language, AstCallback, AstCfg, AstPayload, AstResponse, Span, LANG,
};

use crate::{communication::*, lang::*, *};

/// Information on an `AST` node.
/// Taken directly from the `rust_code_analysis` crate with additional serde macros for deserialization.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
            .find(|child| types.iter().any(|t| child.r#type == *t))
    }

    pub fn find_child_by_value(&self, value: &str) -> Option<&AST> {
        self.children.iter().find(|child| child.value == value)
    }

    pub fn find_all_children_by_type(&self, types: &[&str]) -> Option<Vec<&AST>> {
        let children: Vec<&AST> = self
            .children
            .iter()
            .filter(|child| types.iter().any(|t| child.r#type == *t))
            .collect();
        if !children.is_empty() {
            Some(children)
        } else {
            None
        }
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
    pub fn transform(self, lang: LANG, path: &str) -> (Vec<ComponentType>, Language) {
        // Do language specific AST parsing
        match lang {
            LANG::Cpp => (cpp::find_components(self, path, path), lang.into()),
            LANG::Java => (java::find_components(self, path), lang.into()),
            LANG::Python => (vec![], Language::Python),
            LANG::Go => (vec![], Language::Go),
            lang => {
                tracing::info!("unsupported lang: {:?}", lang);
                (vec![], Language::Unknown)
                // todo!();
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone, new)]
#[serde(rename_all = "camelCase")]
pub struct Directory {
    files: Vec<PathBuf>,
    sub_directories: Vec<Directory>,
    path: PathBuf,
}

pub fn parse_project_context_compat(directory: &Directory) -> std::io::Result<compat::JSSAContext> {
    Ok(parse_project_context(directory)?.into())
}

pub fn parse_project_context(directory: &Directory) -> std::io::Result<JSSAContext> {
    let modules = parse_directory_into_laast(directory)?;
    let ctx = JSSAContext {
        component: ComponentInfo {
            path: "".into(),
            package_name: "".into(),
            instance_name: "context".into(),
            instance_type: InstanceType::AnalysisComponent,
            language: Language::Unknown,
        },
        succeeded: true,
        root_path: "",
        modules,
    };
    Ok(ctx)
}

fn flatten_dirs(dir: &Directory) -> Vec<Directory> {
    let mut dirs: Vec<Directory> = dir.sub_directories.iter().flat_map(flatten_dirs).collect();

    dirs.push(dir.clone());
    dirs
}

#[derive(Debug, Clone, new)]
pub struct ParsedTree {
    ast: AST,
    lang: LANG,
    module_name: String,
    path: PathBuf,
}

pub fn parse_directory_into_laast(dir: &Directory) -> std::io::Result<Vec<ModuleComponent>> {
    convert_trees_to_laast(parse_directory_trees(dir)?)
}

pub fn parse_directory_trees(dir: &Directory) -> std::io::Result<Vec<ParsedTree>> {
    let mut parsed_trees = vec![];

    let dirs = flatten_dirs(dir);

    for dir in dirs {
        // Generate module constants
        let path = dir.path.as_path().to_str().unwrap_or("").to_owned();
        if path.is_empty() {
            continue;
        }

        // Generate module identifier
        let p = dir.path.clone();
        let mod_path;
        if p.is_file() {
            let p: PathBuf = p.iter().dropping_back(1).collect();
            mod_path = p.into_os_string().into_string().unwrap()
        } else {
            mod_path = path.clone();
        }
        let module_name = mod_path.clone();

        // Get directory
        tracing::info!("trying to read {:?}", dir.path);
        let read_dir = match std::fs::read_dir(dir.path) {
            Ok(dir) => dir,
            Err(err) => {
                tracing::warn!("Could not read directory: {:?}", err);
                continue;
            }
        };

        // Generate the sorted directory list
        let mut unwrapped_dirs = vec![];
        for entry in read_dir {
            let entry = entry?;
            if !entry.path().is_dir() {
                unwrapped_dirs.push(entry);
            }
        }
        unwrapped_dirs.sort_by(|a, b| a.path().to_str().cmp(&b.path().to_str()));

        // Handle directories
        for entry in unwrapped_dirs {
            if !dir.files.iter().any(|path_buf| path_buf == &entry.path()) {
                continue;
            }
            let mut file = File::open(entry.path())?;
            let mut code = String::new();
            file.read_to_string(&mut code)?;

            tracing::info!(
                "Parsing file: {:?}",
                entry.path().to_str().unwrap_or_default()
            );
            let result = parse_ast(AstPayload {
                id: "".to_owned(),
                file_name: entry.path().to_str().unwrap_or("").to_owned(),
                code,
                comment: false,
                span: true,
            });

            let (ast, lang) = match result {
                Some((ast, lang)) => (ast, lang),
                None => {
                    tracing::warn!("Could not parse AST {:?}", entry.path());
                    continue;
                }
            };

            parsed_trees.push(ParsedTree::new(
                ast,
                lang,
                module_name.clone(),
                path.clone().into(),
            ));
        }
    }

    tracing::info!("Finished parsing files!");
    Ok(parsed_trees)
}

pub fn convert_trees_to_laast(
    parsed_trees: Vec<ParsedTree>,
) -> std::io::Result<Vec<ModuleComponent>> {
    let get_path_string = |path_buf: &PathBuf| path_buf.as_path().to_str().unwrap_or("").to_owned();
    let mut modules: Vec<ModuleComponent> = vec![];
    let mut language = Language::Unknown;

    for parsed_tree in parsed_trees {
        let path = get_path_string(&parsed_tree.path);
        let mut module = ModuleComponent::new(
            parsed_tree.module_name.clone(),
            path.clone(),
            parsed_tree.lang.into(),
        );

        let (components, lang) = parsed_tree.ast.transform(
            parsed_tree.lang,
            parsed_tree.path.to_str().unwrap_or_default(),
        );

        match lang {
            Language::Unknown => {}
            lang => language = lang,
        }

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
                            tracing::info!(
                                "got other label when it should have been class/ifc: {:#?}",
                                r#type
                            );
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

        modules.push(module);
    }

    Ok(merge_modules(modules, language))
}

fn merge_modules(modules: Vec<ModuleComponent>, lang: Language) -> Vec<ModuleComponent> {
    let modules = match lang {
        Language::Cpp => cpp::merge_modules(modules),
        Language::Java => java::merge_modules(modules),
        _ => modules,
    };

    convert_rpc_and_rest_calls(modules)
}

fn convert_rpc_and_rest_calls(mut modules: Vec<ModuleComponent>) -> Vec<ModuleComponent> {
    let modules_view = modules.clone();
    for module in modules.iter_mut() {
        let module_view = module.clone();
        for class in module.classes.iter_mut() {
            let class_view = class.clone();
            for method in class.component.methods.iter_mut() {
                let method_view = method.clone();
                if let Some(body) = method.body.as_mut() {
                    body.replace_communication_call(
                        &modules_view,
                        &module_view,
                        Some(&class_view),
                        &method_view,
                    );
                }
            }
        }
        for method in module.component.methods.iter_mut() {
            let method_view = method.clone();
            if let Some(body) = method.body.as_mut() {
                body.replace_communication_call(&modules_view, &module_view, None, &method_view);
            }
        }
    }

    modules
}

pub fn parse_file(file: &mut File, path: &Path) -> std::io::Result<(Vec<ComponentType>, Language)> {
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
        None => return Ok((vec![], Language::Unknown)),
    };

    tracing::info!("Parsing file: {:?}", path.to_str().unwrap_or_default());

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
            tracing::info!("{:#?}", MyStruct {
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
