use crate::enums::Method;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
}

impl Request {
    pub fn new(raw_request: String) -> Option<Self>{
        let mut parts = raw_request.split_whitespace();

        return Some(Self {
            method: Method::from_str(parts.next()?)?,
            path: parts.next()?.to_string(),
        })
    }
}