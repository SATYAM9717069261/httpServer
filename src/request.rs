use bytes::Bytes;
use std::{
    collections::HashMap,
    io::{Error, Read},
};

#[derive(Debug, Clone, PartialEq)]
pub struct RequestLine {
    pub method: String,
    pub request_target: String,
    pub http_version: String,
}
#[derive(Debug)]
pub struct Headers {
    pub headers: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub enum ParserState {
    Init,
    Done,
}

#[derive(Debug)]
pub struct Request {
    pub request_line: RequestLine,
    pub state: ParserState,
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

impl RequestLine {
    fn request_parsing(&self, line: &[u8]) -> Result<(Self, usize), &'static str> {
        //print!("REQ=> {:?}", String::from_utf8(line).unwrap());
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

impl Request {
    pub fn new() -> Self {
        Request {
            request_line: RequestLine {
                method: String::new(),
                request_target: String::new(),
                http_version: String::new(),
            },
            state: ParserState::Init,
        }
    }

    pub fn parse(&mut self, data: &[u8]) -> Result<u32, Error> {
        let mut read: usize = 0;
        loop {
            match self.state {
                ParserState::Init => {
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
                    self.state = ParserState::Done;
                }
                ParserState::Done => {
                    break;
                }
            }
        }
        return Ok(1);
    }

    pub fn done(&self) -> bool {
        return self.state == ParserState::Done;
    }

    pub fn request_from_reader<R: Read>(mut reader: R) -> Result<Self, Error> {
        let mut request: Request = Request::new();
        let mut buffer: Vec<u8> = vec![0u8; 30]; // data,size
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
        eprintln!("Request from Main => {:?}", request);
        return Ok(request);
    }
}
