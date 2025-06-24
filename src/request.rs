use std::collections::HashMap;
use crate::HttpMethod;
pub struct Request{
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String
}

impl Request{
    pub fn new(raw_request :&str) -> Request{
        let (headers, body) = match raw_request.split_once("\r\n\r\n") {
            Some((headers, body)) => (headers, body),
            None => (raw_request, "")
        };

        let mut lines = headers.lines();
        let first_line = lines.next().unwrap_or(" ");
        let mut parts = first_line.split(" ");
        let method = match parts.next().unwrap_or(""){
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "PATCH" => HttpMethod::PATCH,
            "HEAD" => HttpMethod::HEAD,
            "OPTIONS" => HttpMethod::OPTIONS,
            "TRACE" => HttpMethod::TRACE,
            "CONNECT" => HttpMethod::CONNECT,
            _ => HttpMethod::UNKNOWN
        };
        let path = parts.next().unwrap().to_string();
        let headers: HashMap<String, String> = lines.filter_map(
            |line| { 
                let trimmed = line.trim();
                match trimmed.split_once(":") { 
                    Some((key, value)) => {
                        let key = key.trim().to_string();
                        let value = value.trim().to_string();
                        Some((key, value))
                    }
                    None => None
                }
            }
        ).collect();
        let body = body.trim().to_string();

        Request{
            method,
            path,
            headers,
            body
        }
    }
}