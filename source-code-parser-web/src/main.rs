use actix_web::{middleware::Logger, App, HttpServer};
use structopt::StructOpt;

mod routes;
use routes::*;

#[derive(StructOpt)]
struct Opt {
    #[structopt(long, short, default_value = "127.0.0.1")]
    host: String,
    #[structopt(long, short, default_value = "8080")]
    port: i32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let addr = format!("{}:{}", opt.host, opt.port);
    HttpServer::new(|| App::new().service(ast).service(ctx).wrap(Logger::default()))
        .bind(addr)?
        .run()
        .await
}
