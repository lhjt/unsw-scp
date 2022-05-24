#![allow(clippy::unused_async)]

use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use awc::Client;
use tracing::instrument;
use url::Url;

use crate::middleware::Email;

#[instrument(skip(payload, client))]
pub(crate) async fn route_whoami(
    req: HttpRequest,
    payload: web::Payload,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    if let Some(Email(email)) = req.conn_data::<Email>() {
        HttpResponse::Ok()
            .insert_header(("x-jwt", req.headers().get("x-auth").unwrap()))
            .body(format!("Hello, {:?}", &email))
    } else if req.path() == "/login" {
        return Ok(HttpResponse::Unauthorized().body(
            "You do not have your certificate installed. Please install it to continue.".to_owned(),
        ));
    } else {
        HttpResponse::TemporaryRedirect()
            .append_header(("Location", "/login"))
            .body("Unauthorized: your certificate could not be validated.".to_owned())
    };

    // TODO: grab the subdomain
    let domain = match req.uri().host() {
        Some(s) => s,
        None => {
            // This should not be possible
            return Ok(HttpResponse::InternalServerError().body("Internal server error: EC.SM"));
        },
    };

    // TODO: Create service registry system
    let url = Url::parse("https://csesoc.app/").unwrap();
    let mut new_url = url;
    new_url.set_path(req.uri().path());
    new_url.set_query(req.uri().query());

    // TODO: This forwarded implementation is incomplete as it only handles the unofficial
    // X-Forwarded-For header but not the official Forwarded one.
    let forwarded_req = client
        .request_from(new_url.as_str(), req.head())
        .no_decompress();
    let forwarded_req = match req.head().peer_addr {
        Some(addr) => forwarded_req.insert_header(("x-forwarded-for", format!("{}", addr.ip()))),
        None => forwarded_req,
    };

    let res = forwarded_req
        .send_stream(payload)
        .await
        .map_err(error::ErrorInternalServerError)?;

    let mut client_resp = HttpResponse::build(res.status());
    // Remove `Connection` as per
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Connection#Directives
    for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
        client_resp.insert_header((header_name.clone(), header_value.clone()));
    }

    Ok(client_resp.streaming(res))
}
