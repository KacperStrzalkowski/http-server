use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
mod enums;
mod models;

use models::Request;

fn handle_connection(mut stream: TcpStream) -> Result<(), std::io::Error>{
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_line = String::new();

    buf_reader.read_line(&mut request_line)?;

    let request: Request = Request::new(request_line).expect("Failed to parse request"); // Rebuild to result, cause code will panic at even one client error
    println!("{}", request.path);

    stream.write(br#"        
    <html>
        <body>Chuj</body>
    </html>"#).expect("Failed to write");
    return Ok(());
}

fn main() -> std::io::Result<()> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:80").expect("Failed to bind host");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream)?;
            }
            Err(e) => println!("{:?}", e),
        }
    }

    return Ok(());
}
