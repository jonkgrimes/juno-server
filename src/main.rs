extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate bytes;
#[macro_use]
extern crate tera;

use bytes::Bytes;
use futures::future::Future;
use actix::*;
use actix_web::{
    error, http::Method, middleware, pred, server, ws, fs,
    ws::{WebsocketContext, Message, ProtocolError},
    App, Error, HttpRequest, HttpResponse, AsyncResponder, FutureResponse, HttpMessage, Result,
};

struct AppState {
    template: tera::Tera,
}

struct Ws;

impl Actor for Ws {
    type Context = WebsocketContext<Self, AppState>;
}

impl StreamHandler<Message, ProtocolError> for Ws {
    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        match msg {
            Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (), 
        }
    }
}

fn index(req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let state = req.state();
    let ctx = tera::Context::new();
    let s = state
        .template
        .render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn data(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.body()
        .from_err()
        .and_then(|bytes: Bytes| {
            println!("===== Body =====\n{:?}", bytes);
            Ok(HttpResponse::Ok().content_type("text/html").body("OK"))
        })
        .responder()
}

fn four_oh_four(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let state = req.state();
    let ctx = tera::Context::new();
    let s = state
        .template
        .render("not_found.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::NotFound().content_type("text/html").body(s))
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    server::new(|| {
        let tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));

        App::with_state(AppState { template: tera })
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.method(Method::GET).with(index))
            .resource("/data", |r| r.method(Method::POST).with(data))
            .resource("/stream", |r| r.f(|req| ws::start(req, Ws)))
            .handler("/static", fs::StaticFiles::new("./static").unwrap().show_files_listing())
            .default_resource(|r| {
                // 404 for GET request
                r.method(Method::GET).f(four_oh_four);
                // all requests that are not `GET`
                r.route()
                    .filter(pred::Not(pred::Get()))
                    .f(|_req| HttpResponse::MethodNotAllowed());
            })
    }).bind("127.0.0.1:8080")
        .expect("Cannot bind to 127.0.0.1:8080")
        .run();
}
