pub mod signing;
pub mod tls;

use bytes::Bytes;

/// A body of a http request,
/// including the raw content and it's type
#[derive(Debug, Clone)]
pub struct Body {
    pub content: Bytes,
    pub content_type: String,
}
