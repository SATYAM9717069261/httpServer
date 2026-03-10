use std::net::{TcpListener, TcpStream};
use std::io::{Read,Write};
use crate::request::Request;
use crate::route::route;
use crate::pool::ThreadPool;

const serverRunAt:&str= "127.0.0.1:8080";

pub fn start() {
    let listener = TcpListener::bind(serverRunAt).expect("Failed to bind port");
    println!("Server run on {}",serverRunAt);
    let pool = ThreadPool::new(4);

    //handle Multiple Connection
    for stream in listener.incoming(){
        match stream{
            Ok(stream) =>{
                 pool.execute(|| {
                    handleConnection(stream);
                });
//                handleConnection(stream);
            },
            Err(exc)=>{
                println!("Connection failed: {}", exc);
            }
        }
    }
}

pub fn handleConnection(mut stream:TcpStream){
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let req = Request::parse(&buffer);
    let response = route(req);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_request_line() {

        let raw_request = b"GET /hello HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let request = Request::parse(raw_request);

        assert_eq!(request.method, "GET");
        assert_eq!(request.path, "/hello");
        assert_eq!(request.version, "HTTP/1.1");
    }

    #[test]
    fn test_parse_headers() {

        let raw_request = b"GET / HTTP/1.1\r\nHost: localhost\r\nUser-Agent: Chrome\r\n\r\n";
        let request = Request::parse(raw_request);

        assert_eq!(request.headers.get("Host").unwrap(), "localhost");
        assert_eq!(request.headers.get("User-Agent").unwrap(), "Chrome");
    }

    #[test]
    fn test_parse_complete_request() {

        let raw_request = b"GET /api HTTP/1.1\r\nHost: example.com\r\nAccept: */*\r\n\r\n";
        let request = Request::parse(raw_request);

        assert_eq!(request.method, "GET");
        assert_eq!(request.path, "/api");
        assert_eq!(request.version, "HTTP/1.1");

        assert_eq!(request.headers.get("Host").unwrap(), "example.com");
        assert_eq!(request.headers.get("Accept").unwrap(), "*/*");
    }

    #[test]
    fn test_empty_headers() {

        let raw_request = b"GET / HTTP/1.1\r\n\r\n";
        let request = Request::parse(raw_request);

        assert_eq!(request.method, "GET");
        assert_eq!(request.path, "/");
        assert!(request.headers.is_empty());
    }
}
