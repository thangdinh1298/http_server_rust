use std::io::Read;
use std::net::TcpStream;
use std::error::Error;

/*
 * Read from stream until a newline character is encountered
 * or byte limit `max_to_read` is encountered
 * or EOF is reached. Which ever comes first
 */
pub fn readline_or_max(
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
        bytes_read += 1;

        if byte[0] == 0xA {
            break;
        }
    }
    Ok((line, bytes_read))
}
