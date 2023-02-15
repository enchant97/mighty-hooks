use std::collections::HashMap;

use mighty_hooks_config::{HookReword, HookRewordDeserializeAs};

#[derive(Debug)]
pub enum RewordErrors {
    BodyMustBeText,
    DeserializeBodyError,
    TemplateError,
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
    // add the raw body to the context for access in template
    tera_context.insert(
        "raw",
        std::str::from_utf8(body).map_err(|_| RewordErrors::BodyMustBeText)?,
    );
    // add the headers to the context for access in template
    tera_context.insert("headers", &headers);
    // deserialize the body if needed for access in template
    match reword.deserialize_as {
        HookRewordDeserializeAs::JsonObject => {
            let v = deserialize_json(body)?;
            match &v.as_object() {
                Some(v) => tera_context.insert("json", &v),
                None => return Err(RewordErrors::DeserializeBodyError),
            };
        }
        HookRewordDeserializeAs::JsonArray => {
            let v = deserialize_json(body)?;
            match &v.as_array() {
                Some(v) => tera_context.insert("json", &v),
                None => return Err(RewordErrors::DeserializeBodyError),
            };
        }
        _ => {}
    };
    // render the template
    tera::Tera::one_off(&reword.content, &tera_context, false)
        .map_err(|_| RewordErrors::TemplateError)
}
