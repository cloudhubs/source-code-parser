use serde::{Deserialize, Serialize};
use std::path::{PathBuf, Path};
use std::fs::{File, DirEntry};

use rust_code_analysis::{
    action, guess_language, AstCallback, AstCfg, AstPayload, AstResponse, Span,
};

use crate::*;

/// Information on an `AST` node.
/// Taken directly from the `rust_code_analysis` crate with additional serde macros for deserialization.
#[derive(Debug, Serialize, Deserialize)]
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

/// Parse the given source code from the `AstPayload`
pub fn parse_ast(payload: AstPayload) -> Option<AST> {
    let file = payload.file_name;
    let buf = payload.code.into_bytes();
    let (language, _ext) = guess_language(&buf, &file);
    let cfg = AstCfg {
        id: payload.id,
        comment: payload.comment,
        span: payload.span,
    };
    Some(action::<AstCallback>(
        &language?,
        buf,
        &PathBuf::from(""),
        None,
        cfg,
    ).into())
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

pub fn parse_project_context(root_path: &Path, payload: AstPayload) -> std::io::Result<JSSAContext> {
    let path_str = root_path.to_str().unwrap_or("");
    let mut ctx = JSSAContext {
        component: ComponentInfo {
            path: path_str,
            package_name: "",
            instance_name: "context",
            instance_type: InstanceType::AnalysisComponent,
        },
        succeeded: true,
        root_path: path_str,
        modules: vec![],
    };
    if root_path.is_dir() {
        if let Some(ast) = parse_ast(payload) {
            let dir = std::fs::read_dir(root_path)?;
            for entry in dir {
                let entry = entry?;
                if entry.path().is_dir() {
                    let module = parse_directory(&entry);
                } else {
                    let file = File::open(entry.path())?;
                    // put these in a root module?
                    let component = parse_file(&file);
                }
            }
        }
    }
    Ok(ctx)
}

pub fn parse_directory(entry: &DirEntry) -> Option<ModuleComponent> {
    None
}

pub fn parse_file(file: &File) -> Vec<ComponentType> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_util(code: &str, path: &str) -> Option<AST> {
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
