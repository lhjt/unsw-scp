#![warn(clippy::pedantic)]
#![allow(clippy::unused_async)]

use actix_tls::accept::rustls::TlsStream;
use actix_web::{dev::Extensions, web, App, HttpResponse, HttpServer, Responder};
use rustls::Certificate;
use tokio::net::TcpStream;

mod tls;

#[derive(Debug, Clone)]
struct ConnectionInfo(String);

async fn route_whoami(
    conn_info: web::ReqData<ConnectionInfo>,
    client_cert: Option<web::ReqData<Certificate>>,
) -> impl Responder {
    if let Some(cert) = client_cert {
        HttpResponse::Ok().body(format!("{:?}\n\n{:?}", &conn_info, &cert))
    } else {
        HttpResponse::Unauthorized().body("No client certificate provided.")
    }
}

fn get_client_cert(connection: &dyn core::any::Any, data: &mut Extensions) {
    if let Some(tls_socket) = connection.downcast_ref::<TlsStream<TcpStream>>() {
        // info!("TLS on_connect");

        let (socket, tls_session) = tls_socket.get_ref();

        let msg = format!(
            "local_addr={:?}; peer_addr={:?}",
            socket.local_addr(),
            socket.peer_addr()
        );

        data.insert(ConnectionInfo(msg));

        if let Some(certs) = tls_session.peer_certificates() {
            // info!("client certificate found");
            for cert in certs {
                if let Ok((_, cert)) = x509_parser::parse_x509_certificate(&cert.0) {
                    cert.extensions();
                }
            }

            // insert a `rustls::Certificate` into request data
            data.insert(certs.last().unwrap().clone());
        }
    } else if let Some(socket) = connection.downcast_ref::<TcpStream>() {
        // info!("plaintext on_connect");

        let msg = format!(
            "local_addr={:?}; peer_addr={:?}",
            socket.local_addr(),
            socket.peer_addr()
        );

        data.insert(ConnectionInfo(msg));
    } else {
        unreachable!("socket should be TLS or plaintext");
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("*", web::get().to(route_whoami)))
        .on_connect(get_client_cert)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
