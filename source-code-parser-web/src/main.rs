use env_logger;

use actix_web::middleware::Logger;
use actix_web::App;
use actix_web::HttpServer;
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
    
    println!("Hello, world!");

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(ast)
            .service(ctx)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(addr)?
    .run()
    .await
}
