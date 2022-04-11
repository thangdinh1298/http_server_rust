use super::request;
use super::response;
use crate::utils;

use std::thread;
use std::error::Error;
use std::net::TcpListener;
use std::net::TcpStream;
use std::collections::VecDeque;
use std::sync::{Mutex, MutexGuard, Condvar, Arc};

use crate::utils::utils::readline_or_max;
use crate::handlers::test_handlers::handler_42;

const REQUEST_LINE_MAX_SIZE: usize = 8095;
const HEADER_LINE_MAX_SIZE: usize = 8096;

pub struct HTTPServer {
    threads: Vec<thread::JoinHandle<()>>,
    thread_num: usize,
    con_buffer: Arc<(Mutex<VecDeque<TcpStream>>, Condvar)>,
    con_buffer_len: usize,
}

fn work(con_buffer: Arc<(Mutex<VecDeque<TcpStream>>, Condvar)>) {
    loop {
        let con = get_con(&con_buffer);
        handle_client(con);
    }
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

fn push_con(con: TcpStream,
            con_buffer: &Arc<(Mutex<VecDeque<TcpStream>>, Condvar)>,
            con_buffer_len: usize) {
    let (lock, cvar) = &**con_buffer;
    let mut guard = lock.lock().unwrap();

    while (*guard).len() == con_buffer_len {
        guard = cvar.wait(guard).unwrap();
    }
    (*guard).push_back(con);
    cvar.notify_one();
}

fn get_con(con_buffer: &Arc<(Mutex<VecDeque<TcpStream>>, Condvar)>) -> TcpStream {
    let (lock, cvar) = &**con_buffer;
    let mut guard = lock.lock().unwrap();

    while (*guard).is_empty() {
        guard = cvar.wait(guard).unwrap();
    }
    let front = guard.pop_front();
    front.unwrap()
}

impl HTTPServer {
    pub fn new(thread_num: usize, con_buffer_len: usize) -> Self {
        HTTPServer {
            threads: Vec::with_capacity(thread_num),
            thread_num: thread_num,
            con_buffer: Arc::new((Mutex::new(VecDeque::with_capacity(con_buffer_len)), Condvar::new())),
            con_buffer_len: con_buffer_len
        }
    }

    pub fn run(mut self) -> Result<(), Box<dyn Error>> {
        {
            for i in 0..self.thread_num {
                let con_buffer = self.con_buffer.clone();
                self.threads.push(thread::spawn(move || {
                    work(con_buffer);
                }));
            }
        }

        let listener = TcpListener::bind("127.0.0.1:8080")?;
        loop {
            match listener.accept() {
                Ok((socket, _addr)) => {
                    push_con(socket, &self.con_buffer, self.con_buffer_len);
                }
                Err(e) => println!("couldn't get client: {:?}", e),
            }
        }
    }
}
