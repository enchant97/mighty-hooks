use futures::future::join_all;
use mighty_hooks_core::Body;
use std::collections::HashMap;

use mighty_hooks_config::HookOut;
use reqwest::header::{HeaderMap, HeaderName};


fn headers_convert(headers: &HashMap<String, String>) -> HeaderMap {
    let mut header_map = HeaderMap::new();
    for (key, value) in headers {
        header_map.insert::<HeaderName>(key.parse::<HeaderName>().unwrap(), value.parse().unwrap());
    }
    header_map
}

struct ToDispatch {
    pub href: String,
    pub body: Body,
    pub headers: HashMap<String, String>,
}

pub struct Dispatcher {
    client: reqwest::Client,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    async fn dispatch(&self, to_dispatch: ToDispatch) {
        // FIXME unwrap usage
        match self
            .client
            .post(to_dispatch.href.clone())
            .body(to_dispatch.body.content)
            .headers(headers_convert(&to_dispatch.headers))
            .header("Content-Type", to_dispatch.body.content_type)
            .send()
            .await
        {
            Ok(_) => log::info!("dispatched webhook to {}", to_dispatch.href),
            Err(_) => log::error!("failed to dispatch webhook to {}", to_dispatch.href),
        }
    }

    pub async fn dispatch_hook(
        &self,
        hook: &HookOut,
        body: Body,
        extra_headers: HashMap<String, String>,
    ) {
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.extend(extra_headers);
        // sign the body if a secret is set
        if let Some(secret) = &hook.secret_256 {
            let signature = mighty_hooks_core::signing::sign_hmac_sha256(secret, &body.content);
            headers.insert(
                "X-Hub-Signature-256".to_string(),
                format!("sha256={}", signature),
            );
        }
        let to_dispatch = ToDispatch {
            href: hook.href.clone(),
            body,
            headers,
        };
        // send the actual request
        self.dispatch(to_dispatch).await;
    }

    pub async fn dispatch_hooks(
        &self,
        hooks: &[HookOut],
        body: Body,
        headers: HashMap<String, String>,
    ) {
        // TODO switch to std::futures when it's out of experimental
        let to_dispatch = hooks.iter().map(|hook| {
            let mut hook_headers = HashMap::new();
            for key in &hook.keep_headers {
                if let Some(value) = headers.get(&key.to_lowercase()) {
                    hook_headers.insert(key.clone(), value.clone());
                }
            }
            self.dispatch_hook(hook, body.clone(), hook_headers)
        });
        join_all(to_dispatch).await;
    }
}