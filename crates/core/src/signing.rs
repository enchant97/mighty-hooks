use ring::hmac;

/// Sign a string with a secret using HMAC SHA256 and return the signature as a hex string
pub fn sign_hmac_sha256(secret: &str, data: &[u8]) -> String {
    // Sign the data with the secret
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
    let signature = hmac::sign(&key, data);
    let signature = signature.as_ref();
    // Convert the signature to a hex string
    signature
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
}

/// Verify a signed string with a secret using HMAC SHA256 and return true if it is valid
pub fn verify_hmac_sha256(secret: &str, data: &[u8], signature: &str) -> bool {
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
    hmac::verify(&key, data, &signature.as_bytes()).is_ok()
}
