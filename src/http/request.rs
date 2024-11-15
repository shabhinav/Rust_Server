use std::io::{Read, Write};
use std::net::TcpStream;
mod static_server;
use server::static_server::StaticFileServer;

pub fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    
    stream.read(&mut buffer)?;

    // Fixed: Using String::from_utf8_lossy instead of String::from_utf8
    let request = String::from_utf8_lossy(&buffer[..]);
    
    println!("Received request:\n{}", request);

    let request_line = request.lines().next().unwrap_or("");
    let (method, path, _version) = parse_request_line(request_line);
    let static_server = StaticFileServer::new("public");

    let response = match (method, path) {
        ("GET", path) if path.starts_with("/") => {
            println!("path====> {}:", path);
            std::io::stdout().flush().unwrap(); // Flush to ensure it prints immediately
            match static_server.serve_file(path) {
                Ok((content_type, contents)) => {
                    let response = format!(
                        "HTTP/1.1 200 OK\r\n\
                        Content-Type: {}\r\n\
                        Content-Length: {}\r\n\
                        Connection: close\r\n\
                        \r\n",
                        content_type,
                        contents.len()
                    );
                    let mut full_response = response.into_bytes();
                    full_response.extend_from_slice(&contents);
                    stream.write_all(&full_response)?;
                    return Ok(());
                }
                Err(_) => create_response(
                    "404 Not Found",
                    "text/plain",
                    "404 - File not found"
                ),
            }
        },
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