mod http;


fn main() {
    let mut server: http::HttpServer = http::HttpServer::new("0.0.0.0:5500");
    server.add_route("/app", "{name: 'Hello World'}");
    server.start();
}
