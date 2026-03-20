use crate::enums::Method;
#[derive(Debug)]
pub struct Request {
    pub _method: Method,
    pub path: String,
}

impl Request {
    pub fn new(raw_request: String) -> Result<Self, ParseError> {
        let mut parts = raw_request.split_whitespace();

        let method_str = parts.next().ok_or(ParseError::EmptyRequest)?.to_string();
        let method = Method::from_str(&method_str).ok_or(ParseError::InvalidMethod(method_str))?;
        let path = parts.next().ok_or(ParseError::MissingPath)?.to_string();

        return Ok(Self {
            _method: method,
            path: path,
        });
    }
}

#[derive(Debug)]
pub enum ParseError {
    EmptyRequest,
    InvalidMethod(String),
    MissingPath,
}

impl std::error::Error for ParseError {}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidMethod(message) => {
                write!(f, "Error: {}", message)
            }
            _ => {
                write!(f, "Error: {:?}", self)
            }
        }
    }
}
