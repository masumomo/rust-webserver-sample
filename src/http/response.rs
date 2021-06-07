use super::status_code::StatusCode;
use std::io::{Result as IoResult, Write};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    // Dynamic Dispatch
    // dyn does this
    //

    // Static Dispatch
    // impl does this
    // pub fn send(&self, stream: &mut impl TcpStream) -> IoResult<()> {
    // pub fn send(&self, stream: &mut impl File) -> IoResult<()> {

    //                              ⬇️ dyn is dynamic
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
