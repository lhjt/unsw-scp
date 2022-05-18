use std::borrow::Cow;

use actix_tls::accept::rustls::TlsStream;
use actix_web::dev::Extensions;
use tokio::net::TcpStream;
use tracing::{debug, instrument, trace};

use crate::tls::get_emails_from_cert;

pub use redirect::CheckCertificate;

mod redirect;

#[derive(Debug, Clone)]
pub(crate) struct Email(pub(crate) String);

#[instrument]
/// Middlware that intercepts the client's TLS certificate and attempts to extract the stored emails.
pub(crate) fn handle_client_cert(connection: &dyn core::any::Any, data: &mut Extensions) {
    if let Some(tls_socket) = connection.downcast_ref::<TlsStream<TcpStream>>() {
        trace!("TLS on_connect");

        let (_, tls_session) = tls_socket.get_ref();

        if let Some(certs) = tls_session.peer_certificates() {
            debug!("client certificate found");
            // Only care about the first certificate in the chain
            let cert = match certs.first() {
                Some(cert) => cert,
                None => unreachable!("must be at least one certificate"),
            };

            let emails: Vec<Cow<str>> = get_emails_from_cert(&cert.0);

            // Find the first email that starts with `_scpU`
            if let Some(e) = emails.iter().find(|e| e.starts_with("_scpU")) {
                data.insert(Email(e.to_string()));
            }
        }
    }
}
