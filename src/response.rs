use std::collections::HashMap;
use std::time::SystemTime;
use crate::StatusCode;

pub struct Response {
    pub status_code: StatusCode,
    pub headers: HashMap<String, String>,
    pub body : String
}
impl Response {
    pub fn new(status_code: StatusCode, body: impl Into<String>) -> Self {
        let body = body.into();
        let mut headers = HashMap::new();

        let content_type = if body.starts_with("{") || body.starts_with("[") {
            "application/json"
        } else {
            "text/plain"
        };

        headers.insert("Date".to_string(), SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string());
        headers.insert("Content-Length".to_string(), body.len().to_string());
        headers.insert("Content-Type".to_string(), content_type.to_string());

        Response{
            status_code,
            headers,
            body
        }
    }
    pub fn to_http_string(&self) -> String{
        let mut response = format!(
            "HTTP/1.1 {} {}\r\n", self.status_code.code(), self.status_code.reason_phrase()
        );
        
        for (k, v) in self.headers.iter() {
            response.push_str(&format!("{}: {}\r\n", k, v))
        }
        
        response.push_str("\r\n");
        response.push_str(&self.body);
        response
    }
}