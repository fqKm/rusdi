#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    TRACE,
    CONNECT,
    UNKNOWN
}

impl HttpMethod {
    pub fn as_str(&self) -> String {
        match self { 
            HttpMethod::GET => String::from("GET"),
            HttpMethod::POST => String::from("POST"),
            HttpMethod::PUT => String::from("PUT"),
            HttpMethod::DELETE => String::from("DELETE"),
            HttpMethod::PATCH => String::from("PATCH"),
            HttpMethod::HEAD => String::from("HEAD"),
            HttpMethod::OPTIONS => String::from("OPTIONS"),
            HttpMethod::TRACE => String::from("TRACE"),
            HttpMethod::CONNECT => String::from("CONNECT"),
            HttpMethod::UNKNOWN => String::from("UNKNOWN")
        }
    }
    
}

impl PartialEq for HttpMethod {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}