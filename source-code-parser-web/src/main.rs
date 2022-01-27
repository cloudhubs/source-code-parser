use actix_web::{middleware::Logger, web, App, HttpServer};
use clap::Parser;

mod routes;
use routes::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Opt {
    #[clap(long, short, default_value = "127.0.0.1")]
    host: String,
    #[clap(long, short, default_value = "8080")]
    port: i32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let opt = Opt::parse();
    let addr = format!("{}:{}", opt.host, opt.port);
    HttpServer::new(|| {
        App::new()
            .service(ast)
            .service(ctx)
            .service(ressa)
            .wrap(Logger::default())
            .app_data(web::JsonConfig::default().limit(1024 * 1024 * 4))
    })
    .bind(addr)?
    .run()
    .await
}
