use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    
    stream.read(&mut buffer)?;

    // Fixed: Using String::from_utf8_lossy instead of String::from_utf8
    let request = String::from_utf8_lossy(&buffer[..]);
    
    println!("Received request:\n{}", request);

    let request_line = request.lines().next().unwrap_or("");
    let (method, path, _version) = parse_request_line(request_line);

    let response = match (method, path) {
        ("GET", "/") => create_response(
            "200 OK",
            "text/plain",
            "Hello, World!"
        ),
        ("GET", "/about") => create_response(
            "200 OK",
            "text/plain",
            "About this server: A basic Rust HTTP implementation"
        ),
        _ => create_response(
            "404 Not Found",
            "text/plain",
            "404 - Page not found"
        ),
    };

    stream.write_all(response.as_bytes())?;
    
    Ok(())
}

fn parse_request_line(request_line: &str) -> (&str, &str, &str) {
    let mut parts = request_line.split_whitespace();
    (
        parts.next().unwrap_or(""),
        parts.next().unwrap_or(""),
        parts.next().unwrap_or(""),
    )
}

fn create_response(status: &str, content_type: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {}\r\n\
         Content-Type: {}\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
        status,
        content_type,
        body.len(),
        body
    )
}