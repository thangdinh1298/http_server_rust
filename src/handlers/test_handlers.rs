use crate::http;

use http::request;
use http::response;
use std::error::Error;

pub fn handler_42(
    request: &request::HTTPRequest,
    response: &mut response::HTTPResponse,
) -> Result<(), Box<dyn Error>> {
    response.set_header("Content-Type", "text");
    response.set_header("Content-Length", "2");
    println!("Writing\n");
    response.write_to_body("42".as_bytes())?;
    Ok(())
}
