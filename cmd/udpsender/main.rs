use std::io::{self, BufRead, Write};
use std::net::UdpSocket;

fn reader(socket: &UdpSocket) {
    let mut buffer = [0u8; 8];

    loop {
        let (data, _) = match socket.recv_from(&mut buffer) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to Read {} ", e);
                break;
            }
        };

        let chunk: String = match str::from_utf8(&buffer[..data]) {
            Ok(d) => d.to_string(),
            Err(e) => {
                eprintln!("Failed to Convet Byte to Char");
                break;
            }
        };
        eprint!("{}", chunk);
    }
}
fn main() -> io::Result<()> {
    let socket = match UdpSocket::bind("127.0.0.1:8080") {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error in Socket :{:?}", e);
            return Ok(());
        }
    };
    reader(&socket);
    return Ok(());
}
