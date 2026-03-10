use crate::response::Response;
use crate::request::Request;

pub fn route(request: Request) -> String {
    match request.path.as_str() {
        "/" => {
            Response::new()
                .header("Content-Type", "text/plain")
                .body("Welcome to Rust HTTP Server")
                .to_http_string()
        }
        "/hello" => {
            Response::new()
                .header("Content-Type", "text/plain")
                .body("Hello Satyam!")
                .to_http_string()
        }
        "/json" => {
            Response::new()
                .header("Content-Type", "application/json")
                .body(r#"{"message":"hello"}"#)
                .to_http_string()
        }
        _ => {
            Response::new()
                .status(404, "Not Found")
                .header("Content-Type", "text/plain")
                .body("404 Page Not Found")
                .to_http_string()
        }
    }
}
