mod http;
use http::request;

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{Read, BufReader, BufRead};
use std::error::Error;

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
            },
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}

fn readline_or_max(stream: &mut TcpStream, max_to_read: usize) -> Result<(String, usize), Box<dyn Error>> {
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

fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
//    let mut buf_reader = BufReader::new(stream.take(8096));
//    let mut buffer = Vec::new();
//    buf_reader.read_until('\n' as u8, &mut buffer);
//    println!("Header line is: {}", String::from_utf8(buffer).unwrap());
//
//    let mut buf_reader = BufReader::new(buf_reader.into_inner().into_inner().take(8096));
//    let mut buffer = Vec::new();
//    buf_reader.read_until('\n' as u8, &mut buffer);
//    println!("Header line is: {}", String::from_utf8(buffer).unwrap());

    let request_line = readline_or_max(&mut stream, 8096)?;
    println!("{}", request_line.0);
    
    let headers_line = readline_or_max(&mut stream, 8096)?;
    println!("{}", headers_line.0);

    let headers_line = readline_or_max(&mut stream, 8096)?;
    println!("{}", headers_line.0);
    Ok(())
}
