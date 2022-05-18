#![warn(clippy::pedantic)]

use anyhow::Context;
use rcgen::{
    BasicConstraints, Certificate, CertificateParams, DnType, DnValue, IsCa, KeyPair, SanType,
};

#[derive(Debug, Clone)]
pub struct ClientCertificatePair {
    pub key: String,
    pub cert: String,
}

/// Creates a new CA certificate and key pair.
///
/// # Errors
///
/// May error when the certificate cannot be generated due to invalid parameters.
pub fn create_ca_certificate() -> anyhow::Result<Certificate> {
    let mut params = CertificateParams::new(vec![]);

    params.distinguished_name.push(
        DnType::CommonName,
        DnValue::Utf8String("Security Challenges Platform".to_string()),
    );
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params.not_before = std::time::SystemTime::now().into();
    params.not_after = std::time::SystemTime::now()
        .checked_add(std::time::Duration::from_secs(60 * 60 * 24 * 365))
        .context("time addition failed")?
        .into();

    Certificate::from_params(params).map_err(anyhow::Error::msg)
}

/// Create an x509 certificate client certificate, signed by a given CA cert.
///
/// # Errors
///
/// May error when the supplied CA certificate details, or supplied user parameters are invalid.
pub fn create_client_cert(name: String, user_id: String) -> anyhow::Result<Certificate> {
    // Create a new key for this certificate
    let mut params = CertificateParams::new(vec![]);
    params.not_before = std::time::SystemTime::now().into();
    params.not_after = std::time::SystemTime::now()
        .checked_add(std::time::Duration::from_secs(60 * 60 * 24 * 90))
        .context("time addition failed")?
        .into();
    params
        .distinguished_name
        .push(DnType::CommonName, DnValue::Utf8String(name));
    params.subject_alt_names.push(SanType::Rfc822Name(user_id));

    Certificate::from_params(params).map_err(anyhow::Error::msg)
}

/// Gets a CA cert for signing purposes from a PEM file.
///
/// # Errors
///
/// The `ca_pem` or `ca_key_pem` are invalid.
pub fn get_ca_cert(ca_pem: &str, ca_key_pem: &str) -> anyhow::Result<Certificate> {
    let ca_keypair = KeyPair::from_pem(ca_key_pem)?;
    let params = CertificateParams::from_ca_cert_pem(ca_pem, ca_keypair)?;

    Certificate::from_params(params).map_err(anyhow::Error::msg)
}

/// Generate a pfx file containing the client cert and key.
///
/// # Errors
///
/// May error if invalid parameters are passed.
pub fn generate_pfx(
    client_cert: &Certificate,
    ca_cert: &Certificate,
    name: &str,
    password: &str,
) -> anyhow::Result<Vec<u8>> {
    let pfx = p12::PFX::new(
        &client_cert.serialize_der_with_signer(ca_cert)?,
        &client_cert.serialize_private_key_der(),
        None,
        password,
        name,
    );

    Ok(pfx.context("failed to crate pfx archive")?.to_der())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_ca_cert() {
        let ca_cert = create_ca_certificate().unwrap();
        std::fs::write("rootCA.pem", ca_cert.serialize_pem().unwrap()).ok();
        std::fs::write("rootCA-key.pem", ca_cert.serialize_private_key_pem()).ok();

        let client_cert = create_client_cert(
            "Test User 1".to_owned(),
            "_scpUz182381+hs@student.host.domain".to_owned(),
        )
        .unwrap();

        std::fs::write(
            "client-cert.pem",
            client_cert.serialize_pem_with_signer(&ca_cert).unwrap(),
        )
        .ok();
        std::fs::write("client-key.pem", client_cert.serialize_private_key_pem()).ok();

        std::fs::write(
            "certs.pfx",
            generate_pfx(&client_cert, &ca_cert, "Test User 1", "password").unwrap(),
        )
        .ok();
    }

    #[test]
    fn it_creates_a_client_cert() {
        let ca_pem = std::fs::read_to_string("rootCA.pem").unwrap();
        let ca_key_pem = std::fs::read_to_string("rootCA-key.pem").unwrap();
        let ca_cert = get_ca_cert(&ca_pem, &ca_key_pem).unwrap();

        let client_cert = create_client_cert(
            "Test User 2".to_owned(),
            "_scpUz000000+hs@student.host.domain".to_owned(),
        )
        .unwrap();

        std::fs::write(
            "client-cert.pem",
            client_cert.serialize_pem_with_signer(&ca_cert).unwrap(),
        )
        .ok();
        std::fs::write("client-key.pem", client_cert.serialize_private_key_pem()).ok();

        std::fs::write(
            "certs.pfx",
            generate_pfx(&client_cert, &ca_cert, "Test User 2", "password").unwrap(),
        )
        .ok();
    }
}
