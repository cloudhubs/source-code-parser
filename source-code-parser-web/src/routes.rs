use multipart::server::Multipart;
use rocket::{http::ContentType, Data};
use rocket_contrib::json::JsonValue;
use rust_code_analysis::*;
use source_code_parser::*;
use std::io::{BufReader, Read};

fn error_response(reason: &str) -> JsonValue {
    return json!({
        "status": "error",
        "reason": reason
    });
}

/// Endpoint for retrieving the AST of a source code file supported by the `rust_code_analysis` crate.
/// The POST request's Content-Type header should be multipart/form-data and should contain two
/// entries: one with key "file" with the file data, and another with the key "ext" with the
/// extension type of the source code files (e.g. "cpp", "go", "rs", "py")
#[post("/ast", data = "<file>")]
pub fn ast(cont_type: &ContentType, file: Data) -> JsonValue {
    if !cont_type.is_form_data() {
        return error_response("Content-Type header must be multipart/form-data");
    }

    let (_, boundary) = match cont_type.params().find(|&(k, _)| k == "boundary") {
        Some(x) => x,
        None => {
            return error_response("Error multipart/form-data: boundary parameter not provided");
        }
    };
    let mut code = String::new();
    let mut ext = String::new();
    let mut multipart = Multipart::with_body(file.open(), boundary);

    for _ in (0..2).enumerate() {
        match multipart.read_entry() {
            Ok(entry) => {
                if let Some(entry) = entry {
                    let mut reader = BufReader::new(entry.data);

                    let entry_header = entry.headers.name.to_string();
                    let buf = match entry_header.as_str() {
                        "file" => &mut code,
                        "ext" => &mut ext,
                        header => {
                            let reason = format!("Invalid entry header {} provided", header);
                            return error_response(&reason);
                        }
                    };

                    if let Err(err) = reader.read_to_string(buf) {
                        let reason = format!("Error reading form data {:?}", err);
                        return error_response(&reason);
                    }
                }
            }
            Err(err) => {
                let reason = format!("Error multipart/form-data: Could not read field {}", err);
                return error_response(&reason);
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
        None => error_response("Could not parse source file."),
    }
}

#[catch(404)]
pub fn not_found() -> JsonValue {
    error_response("Resource was not found.")
}
