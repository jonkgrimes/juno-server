use futures::future::Future;
use actix::*;
use actix_web::{
    error, http::Method, middleware, pred, ws, fs,
    ws::{WebsocketContext, Message, ProtocolError},
    App, Error, HttpRequest, HttpResponse, AsyncResponder, FutureResponse, Result,
};

use super::agent_server;
use agent_server::AgentServer;
use db::DbExecutor;

pub struct AppState {
    pub template: tera::Tera,
    pub addr: Addr<AgentServer>,
    pub db: Addr<DbExecutor>,
}

impl AppState {
    pub fn db(&self) -> &Addr<DbExecutor> {
        &self.db
    }

    pub fn addr(&self) -> &Addr<AgentServer> {
        &self.addr
    }
}

pub struct Ws;

impl Actor for Ws {
    type Context = WebsocketContext<Self, AppState>;
    
    fn started(&mut self, ctx: &mut Self::Context) {
        use agent_server::Connect; 

        let addr = ctx.address();
        ctx.state()
            .addr
            .send(Connect { addr: addr.recipient() })
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
    let msg = super::db::ListAgents();

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

fn four_oh_four(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let state = req.state();
    let ctx = tera::Context::new();
    let s = state
        .template
        .render("not_found.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::NotFound().content_type("text/html").body(s))
}

pub fn create_app(addr: Addr<AgentServer>, db: Addr<DbExecutor>) -> App<AppState> {
    use agent_routes::*;

    let tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));

    App::with_state(AppState { template: tera, addr: addr.clone(), db: db.clone() })
        .middleware(middleware::Logger::default())
        .resource("/", |r| r.method(Method::GET).with(index))
        .resource("/agents/data", |r| r.method(Method::POST).with(data))
        .resource("/agents/register", |r| r.method(Method::POST).with(register))
        .resource("/agents/stream", |r| r.f(stream))
        .handler("/static", fs::StaticFiles::new("./static").unwrap().show_files_listing())
        .default_resource(|r| {
            // 404 for GET request
            r.method(Method::GET).f(four_oh_four);
            // all requests that are not `GET`
            r.route()
                .filter(pred::Not(pred::Get()))
                .f(|_req| HttpResponse::MethodNotAllowed());
        })
}