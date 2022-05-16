use std::{borrow::Cow, vec};

use x509_parser::extensions::GeneralName;

/// Get the emails from a certificate. The emails are taken from the `subjectAlternateNames` component of the certificate.
pub fn get_emails_from_cert(certificate_data: &[u8]) -> Vec<Cow<str>> {
    let cert = match x509_parser::parse_x509_certificate(certificate_data) {
        Ok((_, cert)) => cert,
        Err(_) => return vec![],
    };

    // Get the SAN entry from the certificate
    let entry = match cert
        .iter_extensions()
        .find_map(|e| match e.parsed_extension() {
            x509_parser::extensions::ParsedExtension::SubjectAlternativeName(san_data) => {
                Some(san_data)
            }
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

#[cfg(test)]
mod tests;
