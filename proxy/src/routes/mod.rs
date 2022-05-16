#![allow(clippy::unused_async)]

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::middleware::Email;

pub(crate) async fn route_whoami(
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
