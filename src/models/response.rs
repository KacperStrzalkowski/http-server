
use std::{fs::File, io::Read};
use crate::enums::ResponseHeader;
use std::net::TcpStream;
use std::io::Write;
pub struct Response {
    response_header: ResponseHeader,
    body_file: File,
}

impl Response {
    pub fn new(response_header: &str, body_file: File) -> Result<Self, ResponseError> {
        let response_header: ResponseHeader = ResponseHeader::from_str(&response_header).ok_or(ResponseError::InvalidHeader)?;
        let body_file: File = body_file;

        return Ok(Self {
            response_header,
            body_file
        })

    }

    pub fn send(&self, stream: &mut TcpStream) -> Result<(), std::io::Error>{
        let response_header_string = self.response_header.get_header_str();
        let mut body_file = &self.body_file;
        let mut body = String::new();
        body_file.read_to_string(&mut body)?;

        let response_body_string = format!("{}{}\r\n\r\n{}", response_header_string, &body.len(), &body);
        stream.write(response_body_string.as_bytes())?;
        return Ok(())
    }
}

#[derive(Debug)]
pub enum ResponseError {
    InvalidHeader
}

impl std::error::Error for ResponseError {}
impl std::fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}