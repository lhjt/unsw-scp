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
mod tests {
    use super::get_emails_from_cert;

    #[test]
    fn finds_3_emails_from_cert() {
        let cert_data = r"
-----BEGIN CERTIFICATE-----
MIIB5zCCAY2gAwIBAgIULOehWQnqjm80FO/9Riu+k0DgPSAwCgYIKoZIzj0EAwQw
FTETMBEGA1UEAwwKY3RmLmRvbWFpbjAgFw0yMjA1MTYxNTEyNTJaGA8zMDIxMDkx
NjE1MTI1MlowFTETMBEGA1UEAwwKY3RmLmRvbWFpbjBZMBMGByqGSM49AgEGCCqG
SM49AwEHA0IABFVhIN67yZqjHin7J/xeCTPkbq+bwPlDT3uslWgdCY6C7yZVJoZU
nS3J3SCTX3cUB9nTmOs/m3MOLGLMDyQkhqyjgbgwgbUwHQYDVR0OBBYEFPAqvUOs
cX51E1kEoRkezyBMTgrrMB8GA1UdIwQYMBaAFPAqvUOscX51E1kEoRkezyBMTgrr
MA4GA1UdDwEB/wQEAwIFoDAgBgNVHSUBAf8EFjAUBggrBgEFBQcDAQYIKwYBBQUH
AwIwQQYDVR0RBDowOIEQZW1haWxAZW1haWwuaG9zdIERZW1haWwyQGVtYWlsLmhv
c3SBEWVtYWlsM0BlbWFpbC5ob3N0MAoGCCqGSM49BAMEA0gAMEUCIA0el03Jm4vl
m6TdzikHqCoimLEKkwE2JrrjkaW/MDbjAiEA7fP1PT0ulRvONjGkf4hF/TPsdhcU
g87GfZqig0TyS84=
-----END CERTIFICATE-----
        ";

        match x509_parser::pem::parse_x509_pem(cert_data.as_bytes()) {
            Ok((_, pem)) => {
                let emails = get_emails_from_cert(&pem.contents);
                assert_eq!(emails.len(), 3);
            }
            Err(e) => {
                panic!("{:#?}", e)
            }
        }
    }

    #[test]
    fn finds_1_email_from_cert() {
        let cert_data = r"
-----BEGIN CERTIFICATE-----
MIIBwTCCAWegAwIBAgIUekY6Dg92/WhJcZJTxq6y4YmC8bcwCgYIKoZIzj0EAwQw
FTETMBEGA1UEAwwKY3RmLmRvbWFpbjAgFw0yMjA1MTYxNTE2NDJaGA8zMDIxMDkx
NjE1MTY0MlowFTETMBEGA1UEAwwKY3RmLmRvbWFpbjBZMBMGByqGSM49AgEGCCqG
SM49AwEHA0IABAtdurdGwI3WwwbTiu2r3K3Mkq8p3kGQZis7Mc+MolKj+uexMckx
xMe0Ka2Av7YviOtCeRydjySNpqg41yKuYNejgZIwgY8wHQYDVR0OBBYEFM1P081Z
2O1DLzLekQL8AxcJNYukMB8GA1UdIwQYMBaAFM1P081Z2O1DLzLekQL8AxcJNYuk
MA4GA1UdDwEB/wQEAwIFoDAgBgNVHSUBAf8EFjAUBggrBgEFBQcDAQYIKwYBBQUH
AwIwGwYDVR0RBBQwEoEQZW1haWxAZW1haWwuaG9zdDAKBggqhkjOPQQDBANIADBF
AiA/GYrJfu+TYyOvfUEYJDAmRNughG6bqCTow2isteSb5QIhALTuiM/gEyG3eGua
gNIwej+FX/uaCiy/m8CatWoc72+W
-----END CERTIFICATE-----
        ";

        match x509_parser::pem::parse_x509_pem(cert_data.as_bytes()) {
            Ok((_, pem)) => {
                let emails = get_emails_from_cert(&pem.contents);
                assert_eq!(emails.len(), 1);
            }
            Err(e) => {
                panic!("{:#?}", e)
            }
        }
    }

    #[test]
    fn finds_no_emails_from_cert() {
        let cert_data = r"
-----BEGIN CERTIFICATE-----
MIIBojCCAUigAwIBAgIUHynzRR+6apMMmzLNpzasyDEeziowCgYIKoZIzj0EAwQw
FTETMBEGA1UEAwwKY3RmLmRvbWFpbjAgFw0yMjA1MTYxNTE5MDdaGA8zMDIxMDkx
NjE1MTkwN1owFTETMBEGA1UEAwwKY3RmLmRvbWFpbjBZMBMGByqGSM49AgEGCCqG
SM49AwEHA0IABCBaLypuwnB0EhqKbWfMVPxW/7qS3a2PnE1K2VaRP3SlUvmeNTbO
U7oR6BzjLbz95yZ/+IzJ+1S7gLJCb5SJKr+jdDByMB0GA1UdDgQWBBTFLipPXpnR
2yIVqJ02NnnLr8DCMjAfBgNVHSMEGDAWgBTFLipPXpnR2yIVqJ02NnnLr8DCMjAO
BgNVHQ8BAf8EBAMCBaAwIAYDVR0lAQH/BBYwFAYIKwYBBQUHAwEGCCsGAQUFBwMC
MAoGCCqGSM49BAMEA0gAMEUCIQDNTsS3QmMOsODpKN5MQzReOV6PbVN5rnq6SJha
97wSbAIgVQvdGOpf6xUs8oSv5wZ/J93obeaCkjjb5urQFRf2bWk=
-----END CERTIFICATE-----
        ";

        match x509_parser::pem::parse_x509_pem(cert_data.as_bytes()) {
            Ok((_, pem)) => {
                let emails = get_emails_from_cert(&pem.contents);
                assert_eq!(emails.len(), 0);
            }
            Err(e) => {
                panic!("{:#?}", e)
            }
        }
    }
}
