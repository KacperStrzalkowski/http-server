
#[derive(Debug)]
pub enum Method {
    GET,
    POST
}

impl Method {
    pub fn from_str(s: &str) -> Option<Method> {
        match s {
            "GET" => Some(Method::GET),
            "POST" => Some(Method::POST),
            _ => None
        }
    }
}


pub enum ResponseHeader {
    HTTP(HttpStatus)
}

impl ResponseHeader {
    pub fn from_str(s: &str, status: HttpStatus) -> Option<ResponseHeader> {
        match s {
            "HTTP" => Some(ResponseHeader::HTTP(status)),
            _ => None
        }
    }

    pub fn get_header_str(&self) -> String {
        match self {
            ResponseHeader::HTTP(val) => format!("HTTP/1.1 {}\r\nContent-Type: text/html\r\nContent-Length: ", val.get_status_str()),
        }
    }
}

pub enum HttpStatus {
    Ok200,
    NotFound404
}

impl HttpStatus {
    pub fn get_status_str(&self) -> String {
        match self {
            HttpStatus::Ok200 => "200 OK".to_string(),
            HttpStatus::NotFound404 => "404 Not Found".to_string(),
        }
    }
}