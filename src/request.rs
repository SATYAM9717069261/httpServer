use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
}

impl Request {
    pub fn parse(buffer: &[u8]) -> Self {
        let request_string = String::from_utf8_lossy(buffer);
        let mut lines = request_string.lines();
        let request_line = lines.next().unwrap();
        let mut parts = request_line.split_whitespace();

        let method = parts.next().unwrap_or("").to_string();
        let path = parts.next().unwrap_or("").to_string();
        let version = parts.next().unwrap_or("").to_string();

        let mut headers = std::collections::HashMap::new();

        for line in lines {
            if line.is_empty() {
                break;
            }
            if let Some((key, value)) = line.split_once(":") {
                headers.insert(
                    key.trim().to_string(),
                    value.trim().to_string(),
                );
            }
        }

        Request {
            method,
            path,
            version,
            headers,
        }
    }
}
