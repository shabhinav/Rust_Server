use std::io::{Read, Write};
use std::net::TcpStream;
use super::parser::parse_request_line;
use super::response::create_response;

pub fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

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
