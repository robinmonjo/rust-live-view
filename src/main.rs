extern crate env_logger;

use actix::prelude::*;
use actix_web::{middleware::Logger, web, get, App, HttpRequest, HttpServer, Responder};
use actix_files as fs;
use actix_web_actors::ws;

#[get("/lol")]
async fn root() -> impl Responder {
    return "hello world"
}

async fn ws(r: HttpRequest, stream: web::Payload) -> impl Responder {
    let res = ws::start(MyWs {}, &r, stream);
    res
}

struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(root)
            .service(web::resource("/ws").route(web::get().to(ws)))
            .service(
                // static files
                fs::Files::new("/", "./static/").index_file("index.html"),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
