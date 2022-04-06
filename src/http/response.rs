use std::net::TcpStream;
use std::collections::HashMap;

pub struct HTTPResponse {
    message: String,
    code: u16,
    version: String,
    body: TcpStream,
    headers: HashMap<String, String>,
    status_and_headers_sent: bool
}

impl HTTPResponse {
    pub fn new(stream: TcpStream) -> Self {
        HTTPResponse {
            message: "OK".to_owned(),
            code: 200,
            version: "HTTP/1.0".to_owned(),
            body: stream,
            headers: HashMap::new(),
            status_and_headers_sent: false,
        }
    }
}
