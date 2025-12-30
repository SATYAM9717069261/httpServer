use super::state::ParserState;
use crate::request::Request;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct RequestParser {
    state: ParserState,
    buffer: Vec<u8>,
    request: Request,
}

impl RequestParser {
    pub fn new() -> Self {
        Self {
            state: ParserState::RequestLine,
            buffer: Vec::new(),
            request: Request {
                method: String::new(),
                target: String::new(),
                version: String::new(),
                headers: Vec::new(),
            },
        }
    }

    pub fn take_request(self) -> Request {
        self.request
    }

    fn bad() -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "bad request line")
    }

    fn parse_request_line(&mut self) -> std::io::Result<usize> {
        let Some(pos) = find_crlf(&self.buffer) else {
            return Ok(0);
        };
        let line = &self.buffer[..pos];
        let line_str = std::str::from_utf8(line)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid utf8"))?;

        let mut parts = line_str.split_whitespace();

        self.request.method = parts.next().ok_or(bad())?.to_string();
        self.request.target = parts.next().ok_or(bad())?.to_string();
        self.request.version = parts.next().ok_or(bad())?.to_string();
        let consumed = pos + 2;
        self.buffer.drain(..consumed);
        self.state = ParserState::Headers;
        Ok(consumed)
    }

    pub fn parse(&mut self, input: &[u8]) -> std::io::Result<usize> {
        self.buffer.extend_from_slice(input);
        match self.state {
            ParserState::RequestLine => self.parse_request_line(),
            ParserState::Headers => Ok(0),
            ParserState::Done => Ok(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_request_line() {
        let mut p = RequestParser::new();

        p.parse(b"GET / HTTP/1.1\r\n").unwrap();

        assert_eq!(p.request.method, "GET");
        assert_eq!(p.request.target, "/");
        assert_eq!(p.request.version, "HTTP/1.1");
        assert_eq!(p.state, ParserState::Headers);
    }

    #[test]
    fn parses_split_request_line() {
        let mut p = RequestParser::new();

        p.parse(b"GET / HT").unwrap();
        assert_eq!(p.state, ParserState::RequestLine);

        p.parse(b"TP/1.1\r\n").unwrap();
        assert_eq!(p.request.version, "HTTP/1.1");
        assert_eq!(p.state, ParserState::Headers);
    }
}
