use multipart::server::Multipart;
use rocket::{http::ContentType, Data};
use rocket_contrib::json::JsonValue;
use rust_code_analysis::*;
use source_code_parser::*;
use std::io::{BufReader, Read};

/// Endpoint for retrieving the AST of a source code file supported by the `rust_code_analysis` crate.
/// The POST request's Content-Type header should be multipart/form-data and should contain two
/// entries: one with key "file" with the file data, and another with the key "ext" with the
/// extension type of the source code files (e.g. "cpp", "go", "rs", "py")
#[post("/ast", data = "<file>")]
pub fn ast(cont_type: &ContentType, file: Data) -> JsonValue {
    if !cont_type.is_form_data() {
        return json!({
            "status": "error",
            "reason": "Content-Type header must be multipart/form-data"
        });
    }

    let (_, boundary) = match cont_type.params().find(|&(k, _)| k == "boundary") {
        Some(x) => x,
        None => {
            return json!({
                "status": "error",
                "reason": "Error multipart/form-data: boundary parameter not provided"
            });
        }
    };
    let mut code = String::new();
    let mut ext = String::new();
    let mut multipart = Multipart::with_body(file.open(), boundary);

    for _ in (0..2).enumerate() {
        match multipart.read_entry() {
            Ok(entry) => {
                if let Some(entry) = entry {
                    println!("{}", entry.headers.name);
                    let mut reader = BufReader::new(entry.data);

                    if &entry.headers.name.to_string() == "file" {
                        if let Err(err) = reader.read_to_string(&mut code) {
                            return json!({
                                "status": "error",
                                "reason": format!("Error reading form data {:?}", err)
                            });
                        }
                    } else if &entry.headers.name.to_string() == "ext" {
                        if let Err(err) = reader.read_to_string(&mut ext) {
                            return json!({
                                "status": "error",
                                "reason": format!("Error reading form data {:?}", err)
                            });
                        }
                    }
                }
            }
            Err(err) => {
                return json!({
                    "status": "error",
                    "reason": format!("Error multipart/form-data: Could not read field {}", err)
                });
            }
        }
    }

    let comment = false;
    let span = true;

    match parse_ast(AstPayload {
        id: "".to_owned(),
        file_name: format!("tmp.{}", ext),
        code,
        comment,
        span,
    }) {
        Some(ast) => json!(ast),
        None => json!({
            "status": "error",
            "reason": "Could not parse source file."
        }),
    }
}

#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
