use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream, fmt::Display,
};

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub target: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub stream: TcpStream,
}

impl HttpRequest {
    pub fn new(request_stream: TcpStream) -> HttpRequest {
        let mut bufreader = BufReader::new(&request_stream);
        let mut first_line = String::new();
        bufreader.by_ref().read_line(&mut first_line).unwrap();
        let mut first_line = first_line.split(' ');
        let method = HttpMethod::from(first_line.next().unwrap());
        let target = first_line.next().unwrap();
        let http_version = first_line.next().unwrap().trim_end();
        let mut headers = HashMap::new();

        let mut line = String::new();
        while bufreader.by_ref().read_line(&mut line).unwrap() != 0 {
            if line.is_empty() || !line.contains(": ") {
                break;
            }
            let mut line_split = line.split(": ");
            let key = line_split.next().unwrap();
            let value = line_split.next().unwrap().trim_end();
            headers.insert(key.to_string().to_lowercase(), value.to_string());
            line.clear();
        }

        let mut body = String::new();
        if headers.contains_key("content-length") {
            let size_string = headers.get("content-length").unwrap();
            println!("size_string: '{}'", size_string);
            let size = headers.get("content-length").unwrap().parse::<usize>().unwrap();
            let mut buf = vec![0; size];
            bufreader.read_exact(&mut buf).unwrap();
            body = String::from_utf8(buf).unwrap();
        }

        HttpRequest {
            method,
            target: target.to_string(),
            http_version: http_version.to_string(),
            headers,
            body,
            stream: request_stream
        }
    }

    pub fn respond(&mut self, response: &str) {
        self.stream.write_all(response.as_bytes()).unwrap();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace
}

impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "get" => Self::Get,
            "head" => Self::Head,
            "post" => Self::Post,
            "put" => Self::Put,
            "delete" => Self::Delete,
            "connect" => Self::Connect,
            "options" => Self::Options,
            "trace" => Self::Trace,
            _ => panic!("Invalid HTTP method: {}", value)
        }
    }
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Get => write!(f, "GET"),
            Self::Head => write!(f, "HEAD"),
            Self::Post => write!(f, "POST"),
            Self::Put => write!(f, "PUT"),
            Self::Delete => write!(f, "DELETE"),
            Self::Connect => write!(f, "CONNECT"),
            Self::Options => write!(f, "OPTIONS"),
            Self::Trace => write!(f, "TRACE"),
        }
    }
}

#[derive(Debug)]
pub enum HttpError {
    InvalidMethod,
    InvalidTarget,
    InvalidVersion,
    InvalidHeader,
    InvalidUpgrade
}

impl std::error::Error for HttpError {}

impl Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidMethod => write!(f, "Invalid HTTP method"),
            Self::InvalidTarget => write!(f, "Invalid HTTP target"),
            Self::InvalidVersion => write!(f, "Invalid HTTP version"),
            Self::InvalidHeader => write!(f, "Invalid HTTP header"),
            Self::InvalidUpgrade => write!(f, "Invalid HTTP upgrade"),
        }
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status: HttpStatus,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    pub fn new(status: HttpStatus) -> HttpResponse {
        HttpResponse {
            status,
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string().to_lowercase(), value.to_string());
    }

    pub fn add_body(&mut self, body: &str) {
        self.body = body.to_string();
        self.headers.insert("content-length".to_string(), self.body.len().to_string());
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP/1.1 {}\r\n", self.status.to_string())?;
        for (key, value) in &self.headers {
            write!(f, "{}: {}\r\n", key, value)?;
        }
        write!(f, "\r\n{}", self.body)
    }
}

#[derive(Debug)]
pub enum HttpStatus {
    SwitchingProtocols,
    Ok,
    BadRequest,
    NotFound,
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SwitchingProtocols => write!(f, "101 Switching Protocols"),
            Self::Ok => write!(f, "200 OK"),
            Self::BadRequest => write!(f, "400 Bad Request"),
            Self::NotFound => write!(f, "404 Not Found"),
        }
    }
}
