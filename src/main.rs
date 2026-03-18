use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write, Read};
use std::path::PathBuf;
use std::fs::File;
use std::collections::HashMap;
mod enums;
mod models;

use models::Request;
use models::get_routing;

fn handle_connection(mut stream: TcpStream, router_map: &HashMap<String, PathBuf>) -> Result<(), std::io::Error>{
    let mut stream_reader = BufReader::new(&mut stream);
    let mut request_line = String::new();

    stream_reader.read_line(&mut request_line)?;

    
    println!("{request_line}");
    let request: Request = Request::new(request_line)
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let file_requested_by_path = router_map.get(&request.path[1..]).ok_or(std::io::Error::new(std::io::ErrorKind::InvalidData, "InvalidData"))?;
    let mut file = File::open(file_requested_by_path)?;

    let mut body = String::new();

    file.read_to_string(&mut body)?;

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    );
    
    stream.write(response.as_bytes())?;
    return Ok(());
}

fn main() -> std::io::Result<()> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:80").expect("Failed to bind host");
    let router_map = get_routing()?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_connection(stream, &router_map) {
                    println!("Erron on client connection: {}", e);
                };
            }
            Err(e) => println!("{:?}", e),
        }
    }

    return Ok(());
}
