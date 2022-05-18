#![warn(clippy::pedantic)]

use rcgen::{
    BasicConstraints,
    Certificate,
    CertificateParams,
    DnType,
    DnValue,
    IsCa,
    KeyPair,
    SanType,
};

#[derive(Debug, Clone)]
pub struct ClientCertificatePair {
    pub key:  String,
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

    Certificate::from_params(params).map_err(anyhow::Error::msg)
}

/// Create an x509 certificate client certificate, signed by a given CA cert.
///
/// # Errors
///
/// May error when the supplied CA certificate details, or supplied user parameters are invalid.
pub fn create_client_cert(
    ca_cert: &Certificate,
    ca_keypair: KeyPair,
    name: String,
    user_id: String,
) -> anyhow::Result<ClientCertificatePair> {
    // Create a new key for this certificate
    let mut params = CertificateParams::from_ca_cert_pem(&ca_cert.serialize_pem()?, ca_keypair)?;
    params
        .distinguished_name
        .push(DnType::CommonName, DnValue::Utf8String(name));
    params.subject_alt_names.push(SanType::Rfc822Name(user_id));

    let cert = Certificate::from_params(params)?;
    Ok(ClientCertificatePair {
        key:  cert.serialize_private_key_pem(),
        cert: cert.serialize_pem_with_signer(ca_cert)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_ca_cert() {
        let cert = create_ca_certificate().unwrap();
        std::fs::write("rootCA.pem", cert.serialize_pem().unwrap()).ok();
        std::fs::write("rootCA-key.pem", cert.serialize_private_key_pem()).ok();

        let kp = KeyPair::from_pem(&cert.get_key_pair().serialize_pem()).unwrap();
        let client_cert = create_client_cert(
            &cert,
            kp,
            "Test User 1".to_owned(),
            "_scpUz182381+hs@student.host.domain".to_owned(),
        )
        .unwrap();

        std::fs::write("client-cert.pem", client_cert.cert).ok();
        std::fs::write("client-key.pem", client_cert.key).ok();
    }
}
