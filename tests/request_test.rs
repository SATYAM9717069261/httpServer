use httpServer::request::Request;
use std::io::{Read, Result};

#[derive(Debug)]
struct ChunkReader {
    data: Vec<u8>,
    bytes_per_read: usize,
    head_pos: usize,
}

impl ChunkReader {
    fn new(data: &str, bytes_per_read: usize) -> Self {
        Self {
            data: data.as_bytes().to_vec(),
            bytes_per_read: bytes_per_read,
            head_pos: 0,
        }
    }
}
impl Read for ChunkReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        print!(" Buffer Got  +>{:?} ", buf);
        if self.head_pos >= self.data.len() {
            return Ok(0);
        }
        let last_indx = (self.head_pos + self.bytes_per_read).min(self.data.len());
        let wrote_bytes = last_indx - self.head_pos;
        eprintln!(
            "DETAILS {:?} {} {} {}",
            self.data, self.head_pos, last_indx, wrote_bytes
        );
        buf[..wrote_bytes].copy_from_slice(&self.data[self.head_pos..last_indx]);
        eprintln!("Bufer =>  {:?}", buf);
        self.head_pos += wrote_bytes;
        Ok(wrote_bytes)
    }
}
#[test]
fn test_chunk_reader() {
    let reader = ChunkReader::new(
        "GET /coffee HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n",
        30,
    );

    let (request, state) = match Request::request_from_reader(reader) {
        Ok(Request) => (Request.request_line, Request.state),
        Err(err) => panic!("Error: {}", err),
    };

    assert_eq!(request.method, "GET");
    assert_eq!(request.request_target, "/coffee");
    assert_eq!(request.http_version, "1.1");
}
