use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

#[derive(Debug)]
struct Request {
    method: String,
    path: String,
}

impl Request {
    fn from(buffer: &[u8]) -> Self {
        let request = String::from_utf8_lossy(buffer);
        let request_line = request.lines().next().unwrap_or("");
        let mut parts = request_line.split_whitespace();

        let method = parts.next().unwrap_or("").to_string();
        let path = parts.next().unwrap_or("").to_string();

        Request { method, path }
    }
}

#[derive(Debug)]
struct Response {
    status_code: u16,
    status_text: String,
    body: String,
    content_type: String,
}

impl Response {
    fn new(status_code: u16, body: &str) -> Self {
        let status_text= match status_code {
            200 => "OK",
            404 => "NOT FOUND",
            _ => "UNKNOWN",
        };

        Response {
            status_code,
            status_text: status_text.to_string(),
            body: body.to_string(),
            content_type: "text/plain".to_string(),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
            self.status_code,
            self.status_text,
            self.body.len(),
            self.content_type,
            self.body
        )
        .into_bytes()
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("cannot use port");

    println!("Start Server: http://127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => eprintln!("Failed connect: {}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer).unwrap();

    let request = Request::from(&buffer);
    println!("Request {:?}", request);

    let response = route(request);
    let _ = stream.write_all(&response.to_bytes());
    let _ = stream.flush();
}

fn route(req: Request) -> Response {
    match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/hello") => Response::new(200, "hello"),
        ("GET", "/goodbye") => Response::new(200, "Goodbye!"),
        ("GET", "/") => Response::new(200, "root"),
        _ => Response::new(404, "Not Found"),
    }
}

