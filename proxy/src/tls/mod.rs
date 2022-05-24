use std::{borrow::Cow, fs::File, io::BufReader, vec};

use once_cell::sync::OnceCell;
use rustls::{
    server::AllowAnyAnonymousOrAuthenticatedClient,
    Certificate,
    PrivateKey,
    RootCertStore,
    ServerConfig,
};
use tracing::{debug, instrument, warn};
use x509_parser::extensions::GeneralName;

pub static EDDSA_KEY_PEM: OnceCell<Cow<str>> = OnceCell::new();

pub fn initialise_key_pem() {
    let string_data = std::fs::read_to_string(
        std::env::var("JWT_PEM").unwrap_or_else(|_| "certs/jwt-key.pem".to_string()),
    )
    .expect("could not read JWT keys");

    EDDSA_KEY_PEM
        .set(string_data.into())
        .expect("could not read EDDSA private keys");
}

/// Get the emails from a certificate. The emails are taken from the `subjectAlternateNames`
/// component of the certificate.
#[instrument]
pub fn get_emails_from_cert(certificate_data: &[u8]) -> Vec<Cow<str>> {
    debug!("fetching emails from certificate");
    let cert = match x509_parser::parse_x509_certificate(certificate_data) {
        Ok((_, cert)) => cert,
        Err(e) => {
            warn!("failed to parse certificate: {:?}", e);
            return vec![];
        },
    };

    // Get the SAN entry from the certificate
    let entry = match cert
        .iter_extensions()
        .find_map(|e| match e.parsed_extension() {
            x509_parser::extensions::ParsedExtension::SubjectAlternativeName(san_data) => {
                Some(san_data)
            },
            _ => None,
        }) {
        Some(e) => e,
        None => return vec![],
    };

    // Get emails from the SAN entry
    entry
        .general_names
        .iter()
        .filter_map(|name| match name {
            GeneralName::RFC822Name(email) => Some(email.to_owned().into()),
            _ => None,
        })
        .collect()
}

const CA_CERT: &str = "certs/rootCA.pem";
const SERVER_CERT: &str = "certs/server-cert.pem";
const SERVER_KEY: &str = "certs/server-key.pem";

/// Create the configuration for the TLS server.
pub fn create_tls_server_config() -> Result<ServerConfig, std::io::Error> {
    let mut cert_store = RootCertStore::empty();
    let ca_cert = &mut BufReader::new(File::open(CA_CERT)?);
    let ca_cert = Certificate(rustls_pemfile::certs(ca_cert).unwrap()[0].clone());
    cert_store
        .add(&ca_cert)
        .expect("root CA not added to store");
    let client_auth = AllowAnyAnonymousOrAuthenticatedClient::new(cert_store);
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_client_cert_verifier(client_auth);
    let cert_file = &mut BufReader::new(File::open(SERVER_CERT)?);
    let key_file = &mut BufReader::new(File::open(SERVER_KEY)?);
    let cert_chain = rustls_pemfile::certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = rustls_pemfile::pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();
    Ok(config.with_single_cert(cert_chain, keys.remove(0)).unwrap())
}

#[cfg(test)]
mod tests;
