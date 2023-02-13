use std::{fs::File, io::BufReader};

use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

/// Load key/cert files and convert them to rustls objects for use with the web server
/// TODO add error handling
pub fn load_rustls_config(cert_path: &str, key_path: &str) -> rustls::ServerConfig {
    // code modified from: github.com/actix/examples
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open(cert_path).expect("could not load cert file"));
    let key_file = &mut BufReader::new(File::open(key_path).expect("could not load key file"));

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // check that keys have been loaded
    if keys.is_empty() {
        panic!("No PKCS 8 private keys found in key file");
    }

    config
        .with_single_cert(cert_chain, keys.remove(0))
        .expect("could not load key/cert data")
}
