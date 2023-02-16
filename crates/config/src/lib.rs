use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvVarConfig {
    pub log_level: Option<String>,
    pub config_path: Option<String>,
}

impl EnvVarConfig {
    pub fn from_env() -> Result<Self, envy::Error> {
        envy::prefixed("MIGHTY_HOOKS_").from_env()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HookIn {
    /// Expected Content-Type header value
    pub content_type: String,
    /// Secret for HMAC x-hub-signature-256
    pub secret_256: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HookReword {
    /// New body content type
    pub content_type: String,
    /// New body content, with tera templating
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HookOut {
    /// Where to forward the webhook to
    pub href: String,
    /// Secret for HMAC x-hub-signature-256
    pub secret_256: Option<String>,
    /// Headers to keep from the incoming request
    /// - all others will be dropped
    /// - case-insensitive
    /// - `x-hub-signature` and `x-hub-signature-256` will always be removed
    #[serde(default)]
    pub keep_headers: Vec<String>,
    /// Optionally reword (alter hook output) the body
    pub reword: Option<HookReword>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hook {
    /// Incoming webhook
    pub r#in: HookIn,
    /// Outgoing webhooks
    pub out: Vec<HookOut>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HttpsConfig {
    /// Path to the certificate file
    pub cert: String,
    /// Path to the private key file (in PKCS8 format)
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// Hostname to listen on
    pub host: String,
    /// Port to listen on
    pub port: u16,
    // Setup HTTPS for the server
    pub https: Option<HttpsConfig>,
    /// Whether the server is behind a reverse proxy
    #[serde(default)]
    pub behind_proxy: bool,
    /// FQDN+PATH -> Hook
    pub hooks: HashMap<String, Hook>,
}

impl Config {
    /// Load config from file
    pub fn from_yaml_file(path: &str) -> Result<Self, ()> {
        match std::fs::File::open(path) {
            Ok(file) => match serde_yaml::from_reader(file) {
                Ok(conf) => Ok(conf),
                Err(_) => {
                    eprintln!("Error: Could not parse config file: {}", path);
                    Err(())
                }
            },
            Err(_) => {
                eprintln!("Error: Could not open config file: {}", path);
                Err(())
            }
        }
    }
}
