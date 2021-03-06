#![allow(proc_macro_derive_resolution_fallback)]
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate bytes;
#[macro_use]
extern crate tera;
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate uuid;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;

mod schema;
mod db;
mod models;
mod agent_server;

use dotenv::dotenv;
use std::str;
use bytes::Bytes;
use futures::future::Future;
use actix::*;
use actix_web::{
    error, http::Method, middleware, pred, server, ws, fs,
    ws::{WebsocketContext, Message, ProtocolError},
    App, Error, HttpRequest, HttpResponse, AsyncResponder, FutureResponse, HttpMessage, Result,
};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

use db::{DbExecutor};

#[derive(Debug, Serialize, Deserialize)]
struct RegisterAgentPayload {
    hostname: String,
    ip: String
}

struct AppState {
    template: tera::Tera,
    addr: Addr<agent_server::AgentServer>,
    db: Addr<DbExecutor>,
}

struct Ws;

impl Actor for Ws {
    type Context = WebsocketContext<Self, AppState>;
    
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        ctx.state()
            .addr
            .send(agent_server::Connect { addr: addr.recipient() })
            .into_actor(self)
            .then(|_res, _act, _ctx| {
                fut::ok(())
            })
            .wait(ctx);
    }
}

impl Handler<agent_server::Message> for Ws {
    type Result = ();

    fn handle(&mut self, msg: agent_server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
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

fn index(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let msg = db::ListAgents();

    req.state()
        .db
        .send(msg)
        .from_err()
        .and_then(move |res| {
            match res {
                Ok(agents) => {
                    let mut ctx = tera::Context::new();
                    ctx.add("agents", &agents);
                    let html = req.state()
                        .template
                        .render("index.html", &ctx)
                        .map_err(|e| error::ErrorInternalServerError(e.description().to_owned()))?;
                    Ok(HttpResponse::Ok().content_type("text/html").body(html))
                },
                Err(_) =>  {
                    Ok(HttpResponse::InternalServerError().body("An error occurred"))
                }
            }
        })
        .responder()
}

fn data(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.body()
        .from_err()
        .and_then(move |bytes: Bytes| {
            println!("===== Body =====\n{:?}", bytes);
            let msg = agent_server::ClientMessage {
                msg: str::from_utf8(&bytes).unwrap().to_owned()
            };
            req.state()
                .addr
                .do_send(msg);
            Ok(HttpResponse::Ok().content_type("text/html").body("OK"))
        })
        .responder()
}

fn register(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.json()
        .from_err()
        .and_then(move |val: RegisterAgentPayload| {
            let msg = db::RegisterAgent {
                hostname: val.hostname,
                ip: val.ip
            };
            req.state()
                .db
                .send(msg)
                .from_err()
                .and_then(|res| match res {
                    Ok(agent) => Ok(HttpResponse::Ok().json(agent)),
                    Err(_) => Ok(HttpResponse::InternalServerError().json(r#"{ "error": "An internal server error occurred." }"#))
                })
        })
        .responder()
}

fn stream(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    ws::start(req, Ws)
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
    let sys = actix::System::new("juno-server");

    let agent_server = Arbiter::start(|_| agent_server::AgentServer::default());

    // database setup
    dotenv().ok();

    let database_url = ::std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let db = SyncArbiter::start(3, move || DbExecutor(pool.clone()));

    server::new(move || {
        let tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));

        App::with_state(AppState { template: tera, addr: agent_server.clone(), db: db.clone() })
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.method(Method::GET).with(index))
            .resource("/data", |r| r.method(Method::POST).with(data))
            .resource("/register", |r| r.method(Method::POST).with(register))
            .resource("/stream", |r| r.f(stream))
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
        .start();
    
    let _ = sys.run();
}
