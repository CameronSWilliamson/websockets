use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream, fmt::Display,
};

#[derive(Debug)]
pub struct HttpRequest {
    method: HttpMethod,
    target: String,
    http_version: String,
    headers: HashMap<String, String>,
    body: String,
}

impl HttpRequest {
    pub fn new(request_string: &TcpStream) -> HttpRequest {
        let mut bufreader = BufReader::new(request_string);
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
            headers.insert(key.to_string().to_lowercase(), value.to_string().to_lowercase());
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
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE
}

impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        match value {
            "get" => Self::GET,
            "head" => Self::HEAD,
            "post" => Self::POST,
            "put" => Self::PUT,
            "delete" => Self::DELETE,
            "connect" => Self::CONNECT,
            "options" => Self::OPTIONS,
            "trace" => Self::TRACE,
            _ => panic!("Invalid HTTP method: {}", value)
        }
    }
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GET => write!(f, "GET"),
            Self::HEAD => write!(f, "HEAD"),
            Self::POST => write!(f, "POST"),
            Self::PUT => write!(f, "PUT"),
            Self::DELETE => write!(f, "DELETE"),
            Self::CONNECT => write!(f, "CONNECT"),
            Self::OPTIONS => write!(f, "OPTIONS"),
            Self::TRACE => write!(f, "TRACE"),
        }
    }
}
