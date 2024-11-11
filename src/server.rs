use std::io;
use std::net::{TcpListener, TcpStream};
use std::thread;
use crate::http::request_handler::handle_client;

pub struct Server {
    address: String,
}


impl Server {
    pub fn new(address:&str) -> Server {
        Server {
            address: address.to_string()
        }
    }

    pub fn run(&self) -> io::Result<()> {
        let listener = TcpListener::bind(&self.address)?;

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
}
