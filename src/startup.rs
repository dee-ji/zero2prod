//! src/routes/startup.rs
use std::net::TcpListener;
use actix_web::dev::Server;
use crate::routes::subscriptions::subscribe;
use crate::routes::health_check::health_check;
use actix_web::{web, App, HttpServer};
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new( || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            // A new entry in our routing table for POST /subscriptions requests
            .route("/subscriptions", web::post().to(subscribe))
    })
        .listen(listener)?
        .run();
    Ok(server)
}