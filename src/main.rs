mod http;
mod server;
use http::Method;
use http::Request;
use server::Server;

fn main() {
    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;
    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run();
}
