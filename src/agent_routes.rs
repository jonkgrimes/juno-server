use std::str;
use bytes::Bytes;
use futures::future::Future;
use actix_web::{
    ws, fs,
    Error, HttpRequest, HttpResponse, AsyncResponder, FutureResponse, HttpMessage, Result,
};

use super::{AppState, Ws};
use agent_server::ClientMessage;
use db::{RegisterAgent};

#[derive(Debug, Serialize, Deserialize)]
struct RegisterAgentPayload {
    hostname: String,
    ip: String
}

pub fn stream(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    ws::start(req, Ws)
}

pub fn register(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.json()
        .from_err()
        .and_then(move |val: RegisterAgentPayload| {
            let msg = RegisterAgent {
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

pub fn data(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.body()
        .from_err()
        .and_then(move |bytes: Bytes| {
            println!("===== Body =====\n{:?}", bytes);
            let msg = ClientMessage {
                msg: str::from_utf8(&bytes).unwrap().to_owned()
            };
            req.state()
                .addr
                .do_send(msg);
            Ok(HttpResponse::Ok().content_type("text/html").body("OK"))
        })
        .responder()
}
