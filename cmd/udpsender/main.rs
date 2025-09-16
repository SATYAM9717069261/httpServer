use std::io::{self, BufRead, Write};
use std::net::UdpSocket;

fn reader(socket: &UdpSocket) -> io::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    // if we didn't lock then every readline it lock and Unlock
    // stdin : effeciecy issue

    loop {
        print!(" > ");
        io::stdout().flush()?;
        let mut line = String::new();
        let n = handle.read_line(&mut line)?;
        if n == 0 {
            break;
        }
        socket.send(line.as_bytes())?;
    }
    Ok(())
}
fn main() -> io::Result<()> {
    let socket = match UdpSocket::bind("127.0.0.1:8080") {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error in Socket :{:?}", e);
            return Ok(());
        }
    };
    socket.connect("127.0.0.1:8080")?;
    reader(&socket)?;
    return Ok(());
}
