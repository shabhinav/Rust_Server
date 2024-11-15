use std::io;
use std::net::TcpListener;
use crate::http::request::RequestHandler;

pub struct Server {
    address: String,
    handler: RequestHandler,
}

impl Server {
    pub fn new(address: &str, public_dir: &str) -> Self {
        Server {
            address: address.to_string(),
            handler: RequestHandler::new(public_dir),
        }
    }

    pub fn run(&self) -> io::Result<()> {
        let listener = TcpListener::bind(&self.address)?;
        println!("Server running on {}", self.address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(e) = self.handler.handle_client(stream) {
                        eprintln!("Error handling client: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }

        Ok(())
    }
}
