mod server;
mod http;
use server::Server;
use std::io;

fn main() -> io::Result<()> {
    let server = Server::new("127.0.0.1:8080", "public");
    server.run()
}
