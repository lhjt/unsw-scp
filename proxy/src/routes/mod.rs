#![allow(clippy::unused_async)]

use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use awc::Client;
use tracing::instrument;
use url::Url;

use crate::{middleware::Email, BASE_DOMAIN};

#[instrument(skip(payload, client))]
pub(crate) async fn route_whoami(
    req: HttpRequest,
    payload: web::Payload,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    // Middleware should automatically redirect to login if there is no cert
    let mut new_url: Url;
    if (req.path() == "/enrol"
        || req.path().starts_with("/api")
        || req.path().starts_with("/_next"))
        && req.conn_data::<Email>().is_none()
    {
        // return Ok(HttpResponse::Unauthorized().body("You are missing your certificate."));
        // Make a more elegant page
        if req.path() == "/enrol" || req.path().starts_with("/_next") {
            new_url = Url::parse("http://gaia-frontend:80").unwrap();
        } else {
            new_url = Url::parse("http://gaia-backend:8081").unwrap();
        }
    } else {
        // TODO: grab the subdomain
        let domain = match req.uri().host() {
            Some(s) => s,
            None => {
                // This should not be possible
                eprintln!("req.uri() = {:#?}", req.uri());
                return Ok(HttpResponse::InternalServerError().body("Internal server error: EC.SM"));
            },
        };

        // remove the last 2 elements
        let mut subdomain = domain.split('.').rev().skip(2);
        match subdomain.next() {
            Some("ctf") => match subdomain.next() {
                Some(s) => {
                    // TODO: Check with the service registry to see if this should be proxied
                    // somewhere Return placeholder for the time being
                    new_url = Url::parse("http://gaia-backend:8081/roles").unwrap();
                },
                None => {
                    // TODO: Show the dashboard
                    new_url = Url::parse("https://httpbin.org").unwrap();
                },
            },
            Some(_) | None => {
                // Redirect to the ctf page
                return Ok(HttpResponse::Found()
                    .insert_header(("Location", format!("https://ctf.{}", BASE_DOMAIN.as_str())))
                    .finish());
            },
        }
    }

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
