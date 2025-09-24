use httpServer::request::Request;
use std::io::{self, Read, Result};

#[test]
fn test_request_parse_root() {
    let input =
        "GET / HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n";
    let req = Request::request_from_reader(input).expect("should parse");
    assert_eq!(req.request_line.method, "GET");
    assert_eq!(req.request_line.request_target, "/");
    assert_eq!(req.request_line.http_version, "1.1");
}

#[test]
fn test_request_parse_path() {
    let input = "GET /coffee HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n";
    let req = Request::request_from_reader(input).expect("should parse");
    assert_eq!(req.request_line.method, "GET");
    assert_eq!(req.request_line.request_target, "/coffee");
    assert_eq!(req.request_line.http_version, "1.1");
}

#[test]
fn test_request_parse_header_error() {
    let input = "GET /coffee/12 HTTP/3.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\nE";
    let err = Request::request_from_reader(input).unwrap_err();
    assert_eq!(err, "Validation Fail on http");
}

#[test]
fn test_request_parse_error() {
    let input = "/coffee HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\nE";
    let err = Request::request_from_reader(input).unwrap_err();
    assert_eq!(err, "invalid request line");
}

struct ChunkReader {
    data: Vec<u8>,
    bytes_per_read: usize,
    head_pos: usize,
}

impl ChunkReader {
    /**
     * @input{data:String,bytes_per_read:u8}
     * bytes_per_read means entire String/vvector is divided into chunks
     */
    fn new(data: &str, bytes_per_read: usize) -> Self {
        Self {
            data: data.as_bytes().to_vec(),
            bytes_per_read,
            head_pos: 0,
        }
    }
}
impl Read for ChunkReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        //store n bytes in buf and return how many bytes are stored
        if self.head_pos >= self.data.len() {
            return Ok(0);
        }
        let last_indx = (self.head_pos + self.bytes_per_read).min(self.data.len());
        let wrote_bytes = last_indx - self.head_pos;
        buf[..wrote_bytes].copy_from_slice(&self.data[self.head_pos..last_indx]);
        self.head_pos += wrote_bytes;
        Ok(wrote_bytes)
    }
}

#[test]
fn test_chunk_reader() {
    let mut reader = ChunkReader::new(
        "GET / HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n",
        3,
    );
    let mut buf = [0u8; 4];

    let n = reader.read(&mut buf).unwrap();
    eprintln!("1 {} => {:?}", n, &buf);
    assert_eq!(&buf[..n], b"Hel");

    let n = reader.read(&mut buf).unwrap();
    eprintln!("2 {} => {:?}  {:?}", n, &buf, reader.data);
    assert_eq!(&buf[..n], b"loW");

    let n = reader.read(&mut buf).unwrap();
    eprintln!("3 {} => {:?}", n, &buf);
    assert_eq!(&buf[..n], b"orl");

    let n = reader.read(&mut buf).unwrap();
    eprintln!("4 {} => {:?}", n, &buf);
    assert_eq!(&buf[..n], b"d");

    let n = reader.read(&mut buf).unwrap();
    eprintln!("5 {} => {:?}", n, &buf);
    assert_eq!(n, 0);
}
