use actix_web::{post, web, HttpResponse};
use rust_code_analysis::AstPayload;
use serde::{Deserialize, Serialize};
use serde_json::json;
use source_code_parser::{
    self, parse_ast, parse_project_context, parse_project_context_compat,
    ressa::{run_ressa_parse, NodePattern},
    Directory,
};
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct AstRequest {
    file_path: String,
}

#[post("/ctx")]
pub async fn ctx(payload: web::Json<Directory>) -> HttpResponse {
    match parse_project_context_compat(&payload) {
        Ok(ctx) => ok(ctx),
        Err(err) => internal_server_error(err),
    }
}

#[derive(Deserialize)]
pub struct RessaInput {
    project_dir: Directory,
    patterns: Vec<NodePattern>,
}

#[post("/ressa")]
pub async fn ressa(payload: web::Json<RessaInput>) -> HttpResponse {
    match parse_project_context(&payload.project_dir) {
        Ok(mut context) => ok(run_ressa_parse(
            &mut context.modules,
            payload.patterns.clone(),
        )),
        Err(err) => internal_server_error(err),
    }
}

#[post("/ast")]
pub async fn ast(payload: web::Json<AstRequest>) -> HttpResponse {
    let mut code = String::new();
    let path = PathBuf::from(&payload.file_path);
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => return internal_server_error(err),
    };
    if let Err(err) = file.read_to_string(&mut code) {
        return internal_server_error(err);
    }

    let result = parse_ast(AstPayload {
        id: "".to_owned(),
        file_name: payload.file_path.clone(),
        code,
        comment: false,
        span: true,
    });

    match result {
        Some((ast, _lang)) => ok(ast),
        None => internal_server_error("Invalid language"),
    }
}

fn ok<T: Serialize>(data: T) -> HttpResponse {
    let resp = json!(data);
    HttpResponse::Ok().json(resp)
}

fn internal_server_error<E: Debug>(err: E) -> HttpResponse {
    let resp = json!({
        "status": 500,
        "message": format!("{:?}", err),
    });
    HttpResponse::InternalServerError().json(resp)
}
