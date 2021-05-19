use std::convert::TryFrom;
// use std::convert::TryInto;
// crate means root
use crate::http::{Request,Response, StatusCode,ParseError};
use std::io::{Read, Write};
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: ParseError) -> Response {
        println!("Failed to parse from connection {}", e );
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    addr :String,
    }

    impl Server {
        pub fn new(addr:String) -> Self {
            Server {
                addr
            }
        }
        pub fn run(self, mut handler: impl Handler) {
            println!("Hello, Server! PORT:{}", self.addr);

            let listener = TcpListener::bind(&self.addr).unwrap();
            // dbg!(listener);

            loop {
                match listener.accept() {
                    Ok((mut stream, addr)) => { // You can use underscore to ignore argument
                        let mut buffer = [0;1024];
                        match stream.read(&mut buffer){
                            Ok(_) => {
                                println!("Received a request :{}", String::from_utf8_lossy(&mut buffer));
                                
                                let response = match Request::try_from(&buffer[..]) {
                                    Ok(request) => {
                                        handler.handle_request(&request)
                                        // println!("Parse a request :");
                                        // dbg!(request);
                                        // Response::new(
                                        //     StatusCode::Ok, 
                                        //     Some("<h1>It works!</h1><br />Uhhhhh!".to_string())
                                        // )
                                    },
                                    Err(e) => {
                                        handler.handle_bad_request(e)
                                        // println!("Failed to parse from connection :{}", e );
                                        // Response::new(StatusCode::BadRequest, None)
                                    }
                                };
                                if let Err(e) = response.send(&mut stream) {
                                    println!("Failed to send response :{}", e );
                                }
                            }
                            Err(e) => println!("Failed to read from connection :{}", e )
                        }
                    }
                    Err(e) => println!("Failed to establish a connection :{}", e ),
                    _ => println!("Something wrong"),
                }
            }
        }
    }

