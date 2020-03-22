extern crate env_logger;

mod message;
mod responder;
mod view;

use actix::prelude::*;
use actix_web::{middleware::Logger, web, get, App, HttpRequest, HttpServer, HttpResponse, Responder};
use actix_files as fs;
use actix_web_actors::ws;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(view::index_component(0))
}

async fn ws(r: HttpRequest, stream: web::Payload) -> impl Responder {
    let res = ws::start(Channel {}, &r, stream);
    res
}

struct Channel;

impl Actor for Channel {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Channel {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                match message::decode(&text) {
                    Ok(msg) => {
                        println!("incoming {:?}", msg);
                        match responder::response_for(&msg) {
                            Some(a) => {
                                println!("sending {:?}", a);
                                ctx.text(a)
                            },
                            None => println!("unknown event")
                        }
                    },
                    Err(err) => println!("Error parsing incoming: {:?}", err)
                }
            },
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
            .service(web::resource("/socket/websocket").route(web::get().to(ws)))
            .service(fs::Files::new("/assets", "./assets/"))
            .service(root)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
