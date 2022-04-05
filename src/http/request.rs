use std::collections::HashMap;
use std::error::Error;

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

    pub fn new_from_str(request_str: &str) -> Result<Self, Box<dyn Error>> {
        let tokens: Vec<&str> = request_str.split(' ').collect();
        if tokens.len() != 3 {
            return Err("Incorrect number of tokens for request line".into());
        }
        Ok(HTTPRequest::new(
            Method::new(&tokens[0]),
            tokens[1].to_owned(),
            tokens[2].to_owned(),
        ))
    }

    pub fn parse_header(&mut self, header_str: &str) -> Result<(), Box<dyn Error>> {
        let tokens: Vec<&str> = header_str.split(": ").collect();
        if tokens.len() != 2 {
            return Err("Incorrect number of tokens for header line".into());
        }
        self.headers.insert(tokens[0].to_owned(), tokens[1].to_owned());
        Ok(())
    }
}
