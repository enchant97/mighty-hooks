use std::collections::HashMap;

use mighty_hooks_config::{HookReword, HookRewordDeserializeAs};
use serde::Serialize;

#[derive(Debug)]
pub enum RewordErrors {
    BodyMustBeText,
    DeserializeBodyError,
    TemplateError,
}

#[derive(Debug, Serialize)]
struct ContentContext<'a> {
    pub headers: &'a HashMap<String, String>,
    pub raw: String,
    pub json: Option<serde_json::Value>,
}

fn deserialize_json(content: &[u8]) -> Result<serde_json::Value, RewordErrors> {
    Ok(serde_json::from_slice(content).map_err(|_| RewordErrors::DeserializeBodyError)?)
}

pub fn reword_body(
    reword: &HookReword,
    body: &[u8],
    headers: &HashMap<String, String>,
) -> Result<String, RewordErrors> {
    let mut tera_context = tera::Context::new();
    // add content to the context for access in template
    let content_context = ContentContext {
        headers: &headers,
        raw: String::from_utf8(body.to_vec()).map_err(|_| RewordErrors::BodyMustBeText)?,
        json: match reword.deserialize_as {
            HookRewordDeserializeAs::Json => Some(deserialize_json(body)?),
            _ => None,
        },
    };
    tera_context.insert("content", &content_context);
    // render the template
    tera::Tera::one_off(&reword.content, &tera_context, false)
        .map_err(|_| RewordErrors::TemplateError)
}
