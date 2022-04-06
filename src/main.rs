mod http;
use http::request;
use http::response;

use std::error::Error;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpListener;
use std::net::TcpStream;

const REQUEST_LINE_MAX_SIZE: usize = 8096;
const HEADER_LINE_MAX_SIZE: usize = 8096;

fn main() {
    let method = request::Method::new("get");
    //    let rq = request::HTTPRequest::new(method, "xyz".to_owned(), "1.0".to_owned());
    println!("Method is {}", method.to_string());
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    loop {
        match listener.accept() {
            Ok((socket, _addr)) => {
                if let Err(err) = handle_client(socket) {
                    println!("Error handling connection {:?}", err);
                }
            }
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}
/*
 * Read from stream until a newline character is encountered
 * or byte limit `max_to_read` is encountered
 * or EOF is reached. Which ever comes first
 */
fn readline_or_max(
    stream: &mut TcpStream,
    max_to_read: usize,
) -> Result<(String, usize), Box<dyn Error>> {
    let mut bytes_read: usize = 0;
    let mut byte = [0u8; 1];
    let mut line = String::new();

    while bytes_read < max_to_read {
        let n = match stream.read(&mut byte) {
            Ok(n) => n,
            Err(err) => return Err(Box::new(err)),
        };
        if n == 0 {
            break;
        }
        line.push(byte[0] as char);

        if byte[0] == 0xA {
            break;
        }
    }
    Ok((line, bytes_read))
}

fn handler_42(
    request: &request::HTTPRequest,
    response: &mut response::HTTPResponse,
) -> Result<(), Box<dyn Error>> {
    response.set_header("Content-Type", "text");
    response.set_header("Content-Length", "2");
    println!("Writing\n");
    response.write_to_body("42".as_bytes())?;
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let request_line = readline_or_max(&mut stream, REQUEST_LINE_MAX_SIZE)?.0;

    if !request_line.ends_with("\r\n") {
        return Err("Request line must end with \\r\\n".into());
    }
    println!("Request line {}", request_line);
    let mut request = request::HTTPRequest::new_from_str(&request_line)?;

    let mut bytes_read = 0;
    while bytes_read < HEADER_LINE_MAX_SIZE {
        let (header_line, n) = readline_or_max(&mut stream, HEADER_LINE_MAX_SIZE - bytes_read)?;

        bytes_read += n;
        if !header_line.ends_with('\n') {
            return Err("Header limit of 8096 bytes reached".into());
        } else {
            if header_line == "\r\n" {
                break;
            } else {
                request.parse_header(&header_line)?;
            }
        }
    }

    let mut response = response::HTTPResponse::new(stream);
    Ok(handler_42(&request, &mut response)?)
}
