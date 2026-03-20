#[derive(Debug)]
pub enum Method {
    GET,
    POST,
}

impl Method {
    pub fn from_str(s: &str) -> Option<Method> {
        match s {
            "GET" => Some(Method::GET),
            "POST" => Some(Method::POST),
            _ => None,
        }
    }
}

pub enum ContentType {
    Html,
    Css,
    Js,
    Plain,
}

impl ContentType {
    pub fn from_str(s: &str) -> ContentType {
        match s {
            "html" => ContentType::Html,
            "css" => ContentType::Css,
            "js" => ContentType::Js,
            _ => ContentType::Plain,
        }
    }

    pub fn get_header_str(&self, status: &HttpStatus) -> String {
        let status = status.get_status_str();
        return format!(
            "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: ",
            status,
            self.get_content_type_str()
        );
    }

    fn get_content_type_str(&self) -> String {
        match self {
            ContentType::Html => "text/html".to_string(),
            ContentType::Css => "text/css".to_string(),
            ContentType::Js => "application/javascript".to_string(),
            ContentType::Plain => "text/plain".to_string(),
        }
    }
}

pub enum HttpStatus {
    Ok200,
    NotFound404,
}

impl HttpStatus {
    pub fn get_status_str(&self) -> String {
        match self {
            HttpStatus::Ok200 => "200 OK".to_string(),
            HttpStatus::NotFound404 => "404 Not Found".to_string(),
        }
    }
}
