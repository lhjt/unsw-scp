#![allow(clippy::unused_async)]

use actix_web::{HttpRequest, HttpResponse, Responder};
use tracing::instrument;

use crate::middleware::Email;

#[instrument]
pub(crate) async fn route_whoami(req: HttpRequest) -> impl Responder {
    if let Some(Email(email)) = req.conn_data::<Email>() {
        HttpResponse::Ok().body(format!("Hello, {:?}", &email))
    } else if req.path() == "/login" {
        HttpResponse::Unauthorized().body(
            "You do not have your certificate installed. Please install it to continue.".to_owned(),
        )
    } else {
        HttpResponse::TemporaryRedirect()
            .append_header(("Location", "/login"))
            .body("Unauthorized: your certificate could not be validated.".to_owned())
    }
}
