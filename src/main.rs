extern crate env_logger;
use actix_web::{middleware::Logger, get, App, HttpServer, Responder};

#[get("/")]
async fn root() -> impl Responder {
    return "hello world"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(root)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
