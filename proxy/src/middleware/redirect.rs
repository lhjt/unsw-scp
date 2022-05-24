use std::future::{ready, Ready};

use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http,
    http::header::{HeaderName, HeaderValue},
    Error,
    HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use tracing::error;

use super::Email;
// TODO: move to env
const DOMAIN: &str = "login.local.host:8443";

pub struct CheckCertificate;

impl<S, B> Transform<S, ServiceRequest> for CheckCertificate
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Error = Error;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type InitError = ();
    type Response = ServiceResponse<EitherBody<B>>;
    type Transform = CheckCertificateMiddleware<S>;

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
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = ServiceResponse<EitherBody<B>>;

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
                let email = e.clone();
                let jwt = match intra_jwt::create_jwt(
                    email,
                    "placeholder-username".to_string(),
                    crate::tls::EDDSA_KEY_PEM.get().unwrap(),
                ) {
                    Ok(token) => token,
                    Err(e) => {
                        error!("unable to create jwt: {}", e);
                        panic!()
                    },
                };
                request.headers_mut().insert(
                    HeaderName::from_static("x-auth"),
                    HeaderValue::from_str(&jwt).unwrap(),
                );
            },
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
            },
        };

        let res = self.service.call(request);

        Box::pin(async move {
            // forwarded responses map to "left" body
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}
