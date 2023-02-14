use bytes::Bytes;
use ring::hmac;

/// Sign a string with a secret using HMAC SHA256 and return the signature as a hex string
pub fn sign_hmac_sha256(secret: &str, data: &Bytes) -> String {
    // Sign the data with the secret
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
    let signature = hmac::sign(&key, data);
    let signature = signature.as_ref();
    hex::encode(signature)
}

/// Verify a signed string with a secret using HMAC SHA256 and return true if it is valid
pub fn verify_hmac_sha256(secret: &str, data: &Bytes, signature_hex: &str) -> bool {
    match hex::decode(signature_hex) {
        Ok(signature) => {
            let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
            hmac::verify(&key, data, &signature).is_ok()
        }
        Err(_) => return false,
    }
}
