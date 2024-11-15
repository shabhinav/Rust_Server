mod server;
mod http;
use server::Server;

fn main() -> std::io::Result<()> {
    let server = Server::new("127.0.0.1:8080");
    server.run()
}
