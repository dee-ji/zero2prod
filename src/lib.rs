//! src/lib.rs
use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

// We were returning `impl Responder` at the very beginning.
// We are now spelling out the type explicitly given that we have
// become more familiar with `actix-web`.
// There is no performance difference! Just a stylistic choice :)
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}

// pub trait FormRequest: Sized {
//     type Error = Into<actix_web::Error>;
//
//     async fn from_request(
//        req: &HttpRequest,
//         payload: &mut Payload
//     ) -> Result<Self, Self::Error>;
// }

impl<T> FormRequest for Form<T>
where
    T: DeserializeOwned + 'static,
{
    type Error = actix_web::Error;

    async fn from_request(
        req: &HttpRequest,
        payload: &mut Payload
) -> Result<Self, Self::Error> {
    // Omitted stuff around extractor configuration (e.g. payload size limits)
    match UrlEncoded::new(req, payload).await {
        Ok(item) -> Ok(Form(item)),
        // The error handler can be customised
        // The default one will return a 400, which is what we want.
        Err(e) => Err(error_handler(e))
        }
    }
}

// Let's start simple: we always return a 200 OK
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

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