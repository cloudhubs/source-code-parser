use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use source_code_parser::*;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct AstRequest {
    root: String,
}

#[post("/ast")]
pub fn ast(payload: web::Json<AstRequest>) -> HttpResponse {
    match parse_project_context(&PathBuf::from(&payload.root)) {
        Ok(ctx) => {
            let resp = json!({
                "status": 200,
                "data": ctx
            });
            HttpResponse::Ok().json(resp)
        }
        Err(err) => {
            let msg = format!("{:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": 500,
                "message": msg,
            }))
        }
    }
}
