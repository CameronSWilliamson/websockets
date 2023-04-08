use std::{
    error::Error,
    io::Write,
    net::{TcpListener, TcpStream},
};

use base64::{Engine, engine::general_purpose};
use sha1::{Sha1, Digest};
use websockets::{HttpRequest, HttpMethod, HttpError, HttpResponse, HttpStatus};

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
    let mut req = HttpRequest::new(stream.try_clone().unwrap());
    println!("Request: {:#?}", req);
    websocket_handler(&mut req)?;
    stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())?;
    Ok(())
}

fn websocket_handler(request: &mut HttpRequest) -> Result<(), HttpError> {
    let guid = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
    if request.method != HttpMethod::Get {
        return Err(HttpError::InvalidMethod);
    }
    if let None = request.headers.get("connection") {
        return Err(HttpError::InvalidUpgrade);
    } else if let Some(conn) = request.headers.get("connection") {
        if conn.to_lowercase() != "upgrade" {
            return Err(HttpError::InvalidUpgrade);
        }
    }

    let mut websocket_key = match request.headers.get("sec-websocket-key") {
        Some(key) => key.to_string(),
        None => return Err(HttpError::InvalidHeader)
    };

    let protocol = match request.headers.get("sec-websocket-protocol") {
        Some(proto) => proto,
        None => return Err(HttpError::InvalidHeader)
    };
    
    websocket_key.push_str(guid);
    let mut hasher = Sha1::new();
    hasher.update(websocket_key.as_bytes());
    let result_key = hasher.finalize();
    
    println!("Result key: {:?}", result_key);
    let encoded: String = general_purpose::STANDARD.encode(result_key);
    println!("Encoded: {}", encoded);
    let mut response = HttpResponse::new(HttpStatus::SwitchingProtocols);
    response.add_header("Upgrade", "websocket");
    response.add_header("Connection", "Upgrade");
    response.add_header("Sec-WebSocket-Accept", &encoded);
    response.add_header("Sec-WebSocket-Protocol", protocol); // Only necessary if running multiple
                                                             // websocket handlers
    println!("Response: {}", response);
    //request.stream.write_all(response.to_string().as_bytes()).unwrap();
    request.stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
    Ok(())
}
