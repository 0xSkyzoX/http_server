mod http;


fn main() {
    let mut server = http::HttpServer::new("0.0.0.0:5000");
    server.add_route("/", "Hello, world!");
    server.start();
}
