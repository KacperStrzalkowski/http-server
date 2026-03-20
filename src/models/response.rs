use crate::enums::{ContentType, HttpStatus};
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::path::PathBuf;
use std::{fs::File, io::Read};
pub struct Response {
    response_header: ContentType,
    http_status: HttpStatus,
    body_context: String,
}

impl Response {
    pub fn new(
        requested_path: &str,
        router_map: &HashMap<String, PathBuf>,
    ) -> Result<Self, ResponseError> {
        let requested_file = match router_map.get(requested_path) {
            Some(path) => FileResult::Found(path.clone()),
            None => FileResult::NotFound("404 Not Found".to_string()),
        };
        let http_status;
        let response_header;
        let body_context = match requested_file {
            FileResult::Found(path) => {
                response_header = path.extension()
                    .and_then(|e| e.to_str())
                    .map(ContentType::from_str)
                    .unwrap_or(ContentType::PLAIN);

                let mut file = File::open(path).map_err(|_| ResponseError::InvalidFile)?;

                let mut contents = String::new();

                file.read_to_string(&mut contents)
                    .map_err(|_| ResponseError::InvalidFile)?;
                http_status = HttpStatus::Ok200;
                contents
            }
            FileResult::NotFound(val) => {
                http_status = HttpStatus::NotFound404;
                response_header = ContentType::PLAIN;
                val
            }
        };

        return Ok(Self {
            response_header,
            http_status,
            body_context,
        });
    }

    pub fn send(&self, stream: &mut TcpStream) -> Result<(), std::io::Error> {
        let response_header_string = self.response_header.get_header_str(&self.http_status);
        let body = &self.body_context;

        let response_body_string =
            format!("{}{}\r\n\r\n{}", response_header_string, &body.len(), &body);
        stream.write(response_body_string.as_bytes())?;
        return Ok(());
    }
}

#[derive(Debug)]
pub enum ResponseError {
    InvalidFile,
}

impl std::error::Error for ResponseError {}
impl std::fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq)]
enum FileResult {
    Found(PathBuf),
    NotFound(String),
}
