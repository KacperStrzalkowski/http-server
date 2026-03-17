
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