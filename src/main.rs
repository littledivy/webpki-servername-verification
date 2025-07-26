use rustls_pki_types::{CertificateDer, ServerName};
use webpki::EndEntityCert;

fn main() {
    let cert = include_bytes!("../server.der");

    let ee_der = CertificateDer::from(&cert[..]);
    let cert = EndEntityCert::try_from(&ee_der).unwrap();
    let dns_name = ServerName::try_from("example.local.").unwrap();
    cert.verify_is_valid_for_subject_name(&dns_name)
        .expect("Certificate verification failed");
}
