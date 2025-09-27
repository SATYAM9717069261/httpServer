use std::{
    collections::HashMap,
    io::{Error, Read},
    net::TcpStream,
};

#[derive(Debug, Clone, PartialEq)]
pub struct RequestLine {
    pub method: String,
    pub request_target: String,
    pub http_version: String,
}
const HTTP_VERSION: [&str; 2] = ["HTTP/1.1", "HTTP/2.0"];

fn validatehttp(http_version: &str) -> bool {
    for version in HTTP_VERSION.iter() {
        if http_version == *version {
            return true;
        }
    }
    return false;
}
const SEPRATOR: &[u8] = b"\r\n";

impl RequestLine {
    fn request_parsing(&self, line: &[u8]) -> Result<(Self, usize), &'static str> {
        let idx = match line.windows(SEPRATOR.len()).position(|w| w == SEPRATOR) {
            Some(idx) => idx,
            None => return Err("Invalid request line"),
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

#[derive(Debug)]
pub struct Headers {
    pub headers: HashMap<String, String>,
}
/*
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
} */

#[derive(Debug, PartialEq)]
enum ParserState {
    init,
    done,
}

#[derive(Debug)]
pub struct Request {
    pub request_line: RequestLine,
    pub state: ParserState,
}

impl Request {
    pub fn parse(&mut self, data: &[u8]) -> Result<u32, Error> {
        let mut read: usize = 0;
        loop {
            match self.state {
                ParserState::init => {
                    let (rl, head): (RequestLine, usize) = match self
                        .request_line
                        .request_parsing(&data[read..])
                    {
                        Ok(d) => d,
                        Err(er) => {
                            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, er));
                        }
                    };
                    if head == 0 {
                        break;
                    }
                    self.request_line = rl;
                    read += head;
                    self.state = ParserState::done;
                }
                ParserState::done => {
                    break;
                }
            }
        }
        return Ok(1);
    }

    pub fn done(&self) -> bool {
        return self.state == ParserState::done;
    }

    pub fn request_from_reader(mut reader: TcpStream) -> Result<Self, Error> {
        let mut request: (Request, ParserState);

        let mut buffer = [0u8, 254];
        let mut head_location = 0;
        loop {
            let lines_readed = match reader.read(&mut buffer) {
                Ok(count) => count,
                Err(e) => {
                    eprint!("Error {}", e);
                    return Err(e);
                }
            };
            if lines_readed == 0 {
                break;
            }
            head_location += lines_readed;

            let count = match request.parse(&buffer[..head_location]) {
                Ok(c) => c,
                Err(er) => {
                    eprint!("Error {:?}", er);
                    return Err(er);
                }
            };

            // move buffer data towards front of buffer or remove readed Portion from Buffer
            buffer.copy_within(head_location.., 0);
            head_location -= count as usize;

            if request.done() {
                break;
            }
        }
        return Ok(request);
    }
}
