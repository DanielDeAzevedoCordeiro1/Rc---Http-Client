use super::Method;
pub struct HttpRequest {
    pub method: Method,
    pub path: String,
    pub body: Option<String>,
}

impl HttpRequest {
    pub fn new(method: Method, path: String, body: Option<String>) -> Self {
        Self { method, path, body }
    }
}