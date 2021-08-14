use actix_web::{middleware::Logger, web, App, FromRequest, HttpServer};
use source_code_parser::Directory;
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
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let opt = Opt::from_args();
    let addr = format!("{}:{}", opt.host, opt.port);
    HttpServer::new(|| {
        App::new()
            .service(ast)
            .service(ctx)
            .service(ressa)
            .wrap(Logger::default())
            .app_data(web::Json::<Directory>::configure(|cfg| {
                cfg.limit(1024 * 1024 * 4)
            }))
    })
    .bind(addr)?
    .run()
    .await
}
