#[macro_use]
extern crate log;

mod routes;

// src/main.rs
use actix_files as fs;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use listenfd::ListenFd;

use std::env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new().configure(routes::init_routes).service(
            fs::Files::new("/", "static").index_file("index.html")
            .show_files_listing()
            .use_last_modified(true),
            )
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting server");
    server.run().await
}
