use crate::header::Header;
use std::io::{Error, Read};

#[derive(Debug, PartialEq)]
pub enum ParserState {
    Init,
    Done,
}

#[derive(Debug)]
pub struct Request {
    header: Header, // incomming data
    state: ParserState,
}

impl Request {
    pub fn new() -> Self {
        Self {
            header: Header::new(),
            state: ParserState::Init,
        }
    }

    pub fn parse(&mut self, data: &[u8]) -> Result<u32, Error> {
        let mut read: usize = 0;
        loop {
            match self.state {
                ParserState::Init => {
                    let (rl, head): (Header, usize) = match self
                        .header
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
                    self.header = rl;
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
