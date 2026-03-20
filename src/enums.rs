
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


pub enum ContentType {
    HTML,
    CSS,
    JS,
    PLAIN
}

impl ContentType {
    pub fn from_str(s: &str) -> ContentType {
        match s {
            "html" => ContentType::HTML,
            "css" => ContentType::CSS,
            "js" => ContentType::JS,
            _ => ContentType::PLAIN
        }
    }

    
    pub fn get_header_str(&self, status: &HttpStatus) -> String {
        let status = status.get_status_str();
        return format!("HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: ", status, self.get_content_type_str());
    }
    
    fn get_content_type_str(&self) -> String {
        match self {
            ContentType::HTML => "text/html".to_string(),
            ContentType::CSS => "text/css".to_string(),
            ContentType::JS => "application/javascript".to_string(),
            ContentType::PLAIN => "text/plain".to_string(),
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