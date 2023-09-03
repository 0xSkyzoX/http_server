use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

struct Route {
    path: String,
    response: String,
}

pub struct HttpServer {
    host: String,
    listener: TcpListener,
    routes: Vec<Route>,
}

impl HttpServer {
    pub fn new(host: &str) -> Self {
        let listener = TcpListener::bind(&host).unwrap();
        HttpServer {
            host: host.to_string(),
            listener,
            routes: Vec::new(),
        }
    }

    pub fn start(&self) {
        println!("Starting server on host: {}", self.host);

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_connection(stream);
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let request = String::from_utf8_lossy(&buffer);
        let response = self.route_request(&request);
        let mut body = request.split("\r\n");
        let body_size = body.clone().count();
        if let Some(body_line) = body.nth(body_size-1) {
            // This is the body
            println!("{}", body_line);
        } else {
            println!("Second line not found.");
        }

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn route_request(&self, request: &str) -> String {
        for route in &self.routes {
            if request.contains(&route.path) {
                return format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    route.response.len(),
                    route.response
                );
            }
        }
        return "HTTP/1.1 404 Not Found\r\n\r\n404 Not Found".to_string();
    }

    pub fn add_route(&mut self, path: &str, response: &str) {
        let route = Route {
            path: path.to_string(),
            response: response.to_string(),
        };
        self.routes.push(route);
    }
}
