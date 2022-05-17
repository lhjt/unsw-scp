#![warn(clippy::pedantic)]

use std::{env, fs::File, io::BufReader};

use actix_web::{middleware::Logger, web, App, HttpServer};
use middleware::handle_client_cert;
use rustls::{
    server::AllowAnyAnonymousOrAuthenticatedClient, Certificate, PrivateKey, RootCertStore,
    ServerConfig,
};
use tracing::info;

mod middleware;
mod routes;
mod tls;

const PORT: u16 = 8080;
const CA_CERT: &str = "certs/rootCA.pem";
const SERVER_CERT: &str = "certs/server-cert.pem";
const SERVER_KEY: &str = "certs/server-key.pem";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();

    info!("Launching SCP proxy version {}", env!("CARGO_PKG_VERSION"));

    let mut cert_store = RootCertStore::empty();

    // import CA cert
    let ca_cert = &mut BufReader::new(File::open(CA_CERT)?);
    let ca_cert = Certificate(rustls_pemfile::certs(ca_cert).unwrap()[0].clone());

    cert_store
        .add(&ca_cert)
        .expect("root CA not added to store");

    // set up client authentication requirements
    let client_auth = AllowAnyAnonymousOrAuthenticatedClient::new(cert_store);
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_client_cert_verifier(client_auth);

    // import server cert and key
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
    let config = config.with_single_cert(cert_chain, keys.remove(0)).unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%a %{Host}i %r %s %t (%T)"))
            .default_service(web::route().to(routes::route_whoami))
    })
    .on_connect(handle_client_cert)
    .bind(("0.0.0.0", PORT))?
    .bind_rustls(("0.0.0.0", 8443), config)?
    .run()
    .await
}
