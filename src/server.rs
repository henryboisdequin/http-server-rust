use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, err: &ParseError) -> Response {
        println!("Failed to parse request: {}", err);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Server is running on {} 🚀", self.addr);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];

                    let response = match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(err) => handler.handle_bad_request(&err),
                            }
                        }
                        Err(_err) => unimplemented!(), //println!("Failed to read from connection: {}", err)
                    };

                    if let Err(err) = response.send(&mut stream) {
                        println!("Failed to send response: {}", err);
                    }
                }

                Err(err) => {
                    println!("Failed to establish a connection: {}", err);
                }
            }
        }
    }
}
