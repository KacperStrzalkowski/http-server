
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
    HTTP
}

impl ResponseHeader {
    pub fn from_str(s: &str) -> Option<ResponseHeader> {
        match s {
            "HTTP" => Some(ResponseHeader::HTTP),
            _ => None
        }
    }

    pub fn get_header_str(&self) -> String {
        match self {
            ResponseHeader::HTTP => "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: ".to_string(),
            _ => "".to_string(),
        }
    }
}