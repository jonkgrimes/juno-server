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

mod app;
mod schema;
mod db;
mod models;
mod agent_routes;
mod agent_server;

use dotenv::dotenv;
use actix::*;
use actix_web::{server};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

use db::{DbExecutor};


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

    server::new(move || app::create_app(agent_server.clone(), db.clone()))
        .bind("127.0.0.1:8080")
        .expect("Cannot bind to 127.0.0.1:8080")
        .start();
    
    let _ = sys.run();
}
