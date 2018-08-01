extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate tera;

use actix_web::{error, http, middleware, server, App, Error, HttpRequest, HttpResponse};

struct AppState {
    template: tera::Tera,
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

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    server::new(|| {
        let tera =
            compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));

        App::with_state(AppState{template: tera})
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.method(http::Method::GET).with(index))
         
    }).bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
