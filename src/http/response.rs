use std::io::Result;
use std::net::TcpStream;
use std::collections::HashMap;
use std::io::Write;

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

    pub fn set_message(&mut self, message: &str) {
        self.message = message.to_owned();
    }

    pub fn set_code(&mut self, code: u16) {
        self.code = code;
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_owned(), value.to_owned());
    }

    pub fn write_to_body(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(self.body.write(buf)?)
    }

    fn write_meta_data(&mut self) -> Result<usize> {
        let status_line = format!("{} {} {}\r\n", &self.version, self.code, &self.message);
        Ok(self.body.write(status_line.as_bytes())?)
    }
}
