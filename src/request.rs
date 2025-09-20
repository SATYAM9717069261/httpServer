use std::{collections::HashMap, io::Bytes};

#[derive(Debug, Clone, PartialEq)]
pub struct RequestLine {
    pub method: String,
    pub request_target: String,
    pub http_version: String,
}

pub struct Request {
    pub RequestLine: RequestLine,
    pub Headers: HashMap<String, String>,
    pub Body: Bytes<u8>,
}

impl RequestLine {
    pub fn from_str(line: &str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() != 3 {
            return Err("invalid request line");
        }
        let method: String = parts[0].to_string();
        let request_target: String = parts[1].to_string();
        let http_version: String = parts[2]
            .strip_prefix("HTTP/")
            .ok_or("missing HTTP prefix")?
            .to_string();

        return Ok(Self {
            method,
            request_target,
            http_version,
        });
    }
}

impl Request {
    pub fn RequestFromReader(line: &str) -> Result<Self, &'static str> {
        return {};
    }
}
