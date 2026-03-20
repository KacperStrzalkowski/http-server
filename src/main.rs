use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
mod enums;
mod models;

use models::get_routing;
use models::Request;
use models::Response;

fn handle_connection(
    mut stream: TcpStream,
    router_map: &HashMap<String, PathBuf>,
) -> Result<(), std::io::Error> {
    let mut stream_reader = BufReader::new(&mut stream);
    let mut request_line = String::new();

    stream_reader.read_line(&mut request_line)?;

    println!("{request_line}");
    let request: Request = Request::new(request_line)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let response = Response::new(&request.path[1..], &router_map)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    response.send(&mut stream)?;

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
