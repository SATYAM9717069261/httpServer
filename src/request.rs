use std::{collections::HashMap, io::Bytes};

#[derive(Debug, Clone, PartialEq)]
pub struct RequestLine {
    pub method: String,
    pub request_target: String,
    pub http_version: String,
}

fn validatehttp(nam: &str) -> bool {
    return nam == "HTTP/1.1";
}
impl RequestLine {
    fn request_parsing(line: &str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() != 3 {
            return Err("invalid request line");
        }
        let method: String = parts[0].to_string();
        let request_target: String = parts[1].to_string();

        if validatehttp(parts[2]) != true {
            return Err("Validation Fail on http");
        }

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

#[derive(Debug)]
pub struct Headers {
    pub headers: HashMap<String, String>,
}

impl Headers {
    pub fn header_parser(header_str: &str) -> Result<Self, &'static str> {
        let mut headers: HashMap<String, String> = HashMap::new();
        for line in header_str.split(SEPRATOR) {
            if line.trim().is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.splitn(2, ":").map(|s| s.trim()).collect();
            if parts.len() != 2 {
                return Err("Something Wrong");
            }
            headers.insert(parts[0].to_string(), parts[1].to_string());
        }
        return Ok(Self { headers });
    }
}

#[derive(Debug)]
pub struct Request {
    pub request_line: RequestLine,
    pub headers: Headers,
}
const SEPRATOR: &str = "\r\n";
impl Request {
    pub fn request_from_reader(request: &str) -> Result<Self, &'static str> {
        let parts: Vec<_> = request.match_indices(SEPRATOR).map(|(i, _)| i).collect();
        let request_line: RequestLine;
        match RequestLine::request_parsing(&request[..parts[0]]) {
            Ok(data) => {
                request_line = data;
            }
            Err(er) => {
                panic!("request Parsing Error {:?}", er);
            }
        }
        let headers: Headers;
        match Headers::header_parser(&request[parts[0]..]) {
            Ok(data) => {
                headers = data;
            }
            Err(er) => {
                panic!("header Parsing Error {:?}", er);
            }
        }
        return Ok(Self {
            request_line,
            headers,
        });
    }
}
