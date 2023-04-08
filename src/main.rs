use std::{
    collections::HashMap,
    error::Error,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream)?;
        println!("Connection established!");
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    println!("Reading request...");
    let req = HttpRequest::new(&mut stream);
    println!("Request: {:#?}", req);
    stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())?;
    Ok(())
}

#[derive(Debug)]
struct HttpRequest {
    method: String,
    target: String,
    http_version: String,
    headers: HashMap<String, String>,
    body: String,
}

impl HttpRequest {
    fn new(request_string: &TcpStream) -> HttpRequest {
        let mut bufreader = BufReader::new(request_string);
        let mut first_line = String::new();
        bufreader.by_ref().read_line(&mut first_line).unwrap();
        let mut first_line = first_line.split(' ');
        let method = first_line.next().unwrap();
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
            method: method.to_string(),
            target: target.to_string(),
            http_version: http_version.to_string(),
            headers,
            body,
        }
    }
}
