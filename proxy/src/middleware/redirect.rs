use std::future::{ready, Ready};

use actix_web::body::EitherBody;
use actix_web::dev::{self, ServiceRequest, ServiceResponse};
use actix_web::dev::{Service, Transform};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{http, Error, HttpResponse};
use futures_util::future::LocalBoxFuture;

use super::Email;
// TODO: move to env
const DOMAIN: &str = "localhost:8443";

pub struct CheckCertificate;

impl<S, B> Transform<S, ServiceRequest> for CheckCertificate
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckCertificateMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckCertificateMiddleware { service }))
    }
}
pub struct CheckCertificateMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CheckCertificateMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, mut request: ServiceRequest) -> Self::Future {
        // http -> https redirection
        if request.connection_info().scheme() != "https" {
            let host = request.connection_info().host().to_owned();
            let uri = request.uri().clone();
            let new_uri = format!("https://{}{}", host, uri);
            let (request, _) = request.into_parts();

            let response = HttpResponse::Found()
                .insert_header((http::header::LOCATION, new_uri))
                .finish()
                // constructed responses map to "right" body
                .map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        match request.conn_data::<Email>() {
            Some(Email(e)) => {
                // TODO: Construct authentication JWT
                let header_response =
                    HeaderValue::from_str(e).expect("cert email contains illegal characters");
                request
                    .headers_mut()
                    .insert(HeaderName::from_static("x-auth"), header_response);
            }
            // There is no client cert available
            // Display a warning and a link to collect the certs
            None => {
                if request.connection_info().host() != "" && request.path() != "/login" {
                    // Redirect
                    let (request, _pl) = request.into_parts();

                    let response = HttpResponse::Found()
                        .insert_header((
                            http::header::LOCATION,
                            format!("https://{}/login", DOMAIN),
                        ))
                        .finish()
                        // constructed responses map to "right" body
                        .map_into_right_body();

                    return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
                }
            }
        };

        let res = self.service.call(request);

        Box::pin(async move {
            // forwarded responses map to "left" body
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}
