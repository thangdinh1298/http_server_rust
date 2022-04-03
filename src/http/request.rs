use std::collections::HashMap;

#[allow(dead_code)]
pub enum Method {
    GET,
    POST,
    INVALID,
}

impl Method {
    pub fn to_string(&self) -> &str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::INVALID => "INVALID",
        }
    }

    pub fn new(name: &str) -> Method {
        match name.to_uppercase().as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::INVALID,
        }
    }
}

pub struct HTTPRequest {
    method: Method,
    uri: String,
    version: String,
    path: String,
    headers: HashMap<String, String>,
    params: HashMap<String, String>,
}

impl HTTPRequest {
    fn new(method: Method, uri: String, version: String) -> Self {
        HTTPRequest {
            method,
            uri,
            version,
            path: String::from(""),
            headers: HashMap::new(),
            params: HashMap::new(),
        }
    }

//    pub fn make_request(int sockfd) {
//
//    }
}
