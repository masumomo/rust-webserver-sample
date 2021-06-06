use std::convert::TryFrom;
// use std::convert::TryInto;
// crate means root
use crate::http::{ParseError, Request, Response, StatusCode};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: ParseError) -> Response {
        println!("Failed to parse from connection {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }
    pub fn run(self, mut handler: impl Handler + Send + 'static) {
        println!("Hello, Server! PORT:{}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        let thread_handler = Arc::new(Mutex::new(handler));
        loop {
            let cloned_thread_handler = Arc::clone(&thread_handler);
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    thread::spawn(move || {
                        // You can use underscore to ignore argument
                        let mut buffer = [0; 1024];
                        match stream.read(&mut buffer) {
                            Ok(_) => {
                                println!(
                                    "Received a request :{}",
                                    String::from_utf8_lossy(&mut buffer)
                                );

                                let mut m = cloned_thread_handler.lock().unwrap();
                                let response = match Request::try_from(&buffer[..]) {
                                    Ok(request) => {
                                        println!("Parse a request :");
                                        dbg!(&request);
                                        m.handle_request(&request)
                                    }
                                    Err(e) => {
                                        println!("Failed to parse from connection :{:?}", e);
                                        m.handle_bad_request(e)
                                    }
                                };
                                if let Err(e) = response.send(&mut stream) {
                                    println!("Failed to send response :{}", e);
                                }
                            }
                            Err(e) => println!("Failed to read from connection :{}", e),
                        }
                    });
                }
                Err(e) => println!("Failed to establish a connection :{}", e),
                _ => println!("Something wrong"),
            }
        }
    }
}
// }
