#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    method: String,
    request_target: String,
    http_version: String,
}

const HTTP_VERSION: [&str; 2] = ["HTTP/1.1", "HTTP/2.0"];
const SEPRATOR: &[u8] = b"\r\n";

fn validatehttp(http_version: &str) -> bool {
    for version in HTTP_VERSION.iter() {
        if http_version == *version {
            return true;
        }
    }
    return false;
}
impl Header {
    pub fn new() -> Header {
        Header {
            method: String::new(),
            request_target: String::new(),
            http_version: String::new(),
        }
    }
    pub fn request_parsing(&self, line: &[u8]) -> Result<(Self, usize), &'static str> {
        let idx = match line.windows(SEPRATOR.len()).position(|w| w == SEPRATOR) {
            Some(idx) => idx,
            None => return Err("Invalid request line Didn't found /\r/\n [Increase Buffer Size]"),
        };

        let header = &line[0..idx];
        let head = header.len() + SEPRATOR.len();

        let header_str = std::str::from_utf8(header).map_err(|_| "Invalid UTF-8")?;

        let parts: Vec<&str> = header_str.split_whitespace().collect();

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
        let rl = Self {
            method,
            request_target,
            http_version,
        };
        return Ok((rl, head));
    }
}
