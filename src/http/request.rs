use std::io::{self, Read, Write};
use std::net::TcpStream;
use super::static_handler::StaticFileServer;

pub struct RequestHandler {
    static_handler: StaticFileServer,
}

impl RequestHandler {
    pub fn new(public_dir: &str) -> Self {
        RequestHandler {
            static_handler: StaticFileServer::new(public_dir),
        }
    }

    pub fn handle_client(&self, mut stream: TcpStream) -> io::Result<()> {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;

        let request = String::from_utf8_lossy(&buffer[..]);
        let request_line = request.lines().next().unwrap_or("");
        let parts: Vec<&str> = request_line.split_whitespace().collect();

        if parts.len() < 2 {
            return Ok(());
        }

        let method = parts[0];
        let path = parts[1];

        println!("Received request: {} {}", method, path);

        let response = match (method, path) {
            ("GET", request_path) => {
                match self.static_handler.serve_file(request_path) {
                    Ok((content_type, contents)) => {
                        println!("Successfully read file, sending response");
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
                        full_response
                    }
                    Err(e) => {
                        println!("Error serving file: {:?}", e);
                        format!(
                            "HTTP/1.1 404 Not Found\r\n\
                            Content-Type: text/plain\r\n\
                            Content-Length: 19\r\n\
                            Connection: close\r\n\
                            \r\n\
                            404 - File not found"
                        )
                        .into_bytes()
                    }
                }
            }
            _ => {
                format!(
                    "HTTP/1.1 405 Method Not Allowed\r\n\
                    Content-Type: text/plain\r\n\
                    Content-Length: 22\r\n\
                    Connection: close\r\n\
                    \r\n\
                    Method not allowed: {}"
                    , method
                )
                .into_bytes()
            }
        };

        stream.write_all(&response)?;
        stream.flush()?;
        
        Ok(())
    }
}
