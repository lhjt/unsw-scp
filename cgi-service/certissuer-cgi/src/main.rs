use http::{Method, StatusCode};
use serde::{Deserialize, Serialize};

mod handlers;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BodyPayload {
    pub op: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RegisterPayload {
    pub email: Option<String>,
    pub zid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SetCAPayload {
    pub cert_pem: String,
    pub key_pem: String,
}

mod errors;
mod tokens;

macro_rules! invalid_operation {
    () => {
        cgi::text_response(StatusCode::BAD_REQUEST, "Invalid operation")
    };
}

fn main() {
    cgi::handle(|request: cgi::Request| -> cgi::Response {
        let (head, body) = request.into_parts();
        // Method should be post
        if head.method != Method::POST {
            return cgi::text_response(StatusCode::METHOD_NOT_ALLOWED, "Method not allowed");
        }

        // Differentiate operation based on query string
        // We will support either a CA change, email submission, or cert generation
        let query = match head.uri.query() {
            Some(v) => v,
            None => return invalid_operation!(),
        };

        let operation = match querystring::querify(query)
            .iter()
            .find(|(key, _)| key == &"op")
        {
            Some(op) => op.1,
            None => return invalid_operation!(),
        };

        match operation {
            // Generate certificate
            "gc" => handlers::generate_certificate_handler(query),
            // Send email
            "se" => todo!(),
            // Update ca certs
            "uc" => todo!(),
            _ => invalid_operation!(),
        }
    })
}
