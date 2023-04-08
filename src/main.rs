use std::{error::Error, net::{TcpListener, TcpStream}, io::Write};

use websockets::HttpRequest;


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

