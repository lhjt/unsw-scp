#![warn(clippy::pedantic)]
#![allow(clippy::unused_async)]

use std::env;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use middleware::{handle_client_cert, Email};
use tracing::info;

mod middleware;
mod tls;

async fn route_whoami(
    client_email: Option<web::ReqData<Email>>,
    req: HttpRequest,
) -> impl Responder {
    eprintln!("req = {:#?}", req);
    if let Some(email) = client_email {
        HttpResponse::Ok().body(format!("Hello, {:?}", &email))
    } else if req.path() == "/login" {
        HttpResponse::Unauthorized().body(
            "You do not have your certificate installed. Please install it to continue.".to_owned(),
        )
    } else {
        HttpResponse::TemporaryRedirect()
            .append_header(("Location", "/login"))
            .body("Unauthorized: your certificate could not be validated.".to_owned())
        // HttpResponse::Unauthorized().body("No client certificate provided.")
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();

    info!("Launching server on port 8080");

    HttpServer::new(|| App::new().default_service(web::route().to(route_whoami)))
        .on_connect(handle_client_cert)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
