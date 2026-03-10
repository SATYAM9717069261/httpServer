use std::collections::HashMap;

pub struct Response {
    status_code: u16,
    status_text: String,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {

    pub fn new() -> Self {
        Response {
            status_code: 200,
            status_text: "OK".to_string(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn status(mut self, code: u16, text: &str) -> Self {
        self.status_code = code;
        self.status_text = text.to_string();
        self
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    pub fn to_http_string(&self) -> String {
        let mut response = format!(
            "HTTP/1.1 {} {}\r\n",
            self.status_code,
            self.status_text
        );

        for (k, v) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", k, v));
        }

        response.push_str(&format!("Content-Length: {}\r\n", self.body.len()));

        response.push_str("\r\n");

        response.push_str(&self.body);

        response
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_response_builder() {

        let response = Response::new()
            .status(200, "OK")
            .header("Content-Type", "text/plain")
            .body("hello")
            .to_http_string();

        assert!(response.contains("HTTP/1.1 200 OK"));
        assert!(response.contains("Content-Type: text/plain"));
        assert!(response.contains("Content-Length: 5"));
        assert!(response.contains("hello"));
    }
}
