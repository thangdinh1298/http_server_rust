use std::collections::HashMap;
use std::io::Result;
use std::io::Write;
use std::net::TcpStream;

#[allow(dead_code)]
pub struct HTTPResponse {
    message: String,
    code: u16,
    version: String,
    body: TcpStream,
    headers: HashMap<String, String>,
    status_and_headers_sent: bool,
}

#[allow(dead_code)]
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
        if !self.status_and_headers_sent {
            self.write_status_and_header()?;
        }
        Ok(self.body.write(buf)?)
    }

    fn write_status_and_header(&mut self) -> std::result::Result<(), std::io::Error> {
        let status_line = format!("{} {} {}\r\n", &self.version, self.code, &self.message);
        self.body.write(status_line.as_bytes())?;
        self.status_and_headers_sent = true;

        for (key, value) in &self.headers {
            self.body
                .write(format!("{}: {}", key.to_owned(), value.to_owned()).as_bytes())?;
            self.body.write("\r\n".as_bytes())?;
        }

        self.body.write("\r\n".as_bytes())?;
        Ok(())
    }
}

impl Drop for HTTPResponse {
    fn drop(&mut self) {
        if !self.status_and_headers_sent {
            self.write_status_and_header();
            //TODO: What happens if write status and headers fail here ?
            //Write a separate cleanup method for response before dropping ?
        }
    }
}
