use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HookIn {
    /// Expected Content-Type header value
    pub content_type: String,
    /// Secret for HMAC x-hub-signature-256
    pub secret_256: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HookOut {
    /// Where to forward the webhook to
    pub href: String,
    /// Secret for HMAC x-hub-signature-256
    pub secret_256: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Hook {
    /// Incoming webhook
    pub r#in: HookIn,
    /// Outgoing webhooks
    pub out: Vec<HookOut>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// Hostname to listen on
    pub host: String,
    /// Port to listen on
    pub port: u16,
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
