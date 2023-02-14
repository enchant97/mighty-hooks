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

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use super::*;

    #[test]
    fn test_sign_hmac_sha256() {
        let expected =
            String::from("ffad4a016b7e758451f02a86a9c9b504be8e70c0df26afbab7dccc8ddeb90a71");
        let result = super::sign_hmac_sha256("my-secret", &Bytes::from("my-data"));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_verify_hmac_sha256() {
        let signature =
            String::from("ffad4a016b7e758451f02a86a9c9b504be8e70c0df26afbab7dccc8ddeb90a71");
        let result_true = verify_hmac_sha256("my-secret", &Bytes::from("my-data"), &signature);
        assert_eq!(true, result_true);
        let result_false = verify_hmac_sha256("not-my-secret", &Bytes::from("my-data"), &signature);
        assert_eq!(false, result_false);
        let result_false_2 =
            verify_hmac_sha256("my-secret", &Bytes::from("not-my-data"), &signature);
        assert_eq!(false, result_false_2);
    }
}
