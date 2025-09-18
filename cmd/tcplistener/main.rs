use std::fs::File;
use std::io::{self, Read};
use std::net::{TcpListener, TcpStream};

fn get_lines_reader(mut file: TcpStream) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    let mut buffer = [0u8; 8];
    let mut left_over = String::new();
    loop {
        let data = match file.read(&mut buffer) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to Read {} ", e);
                return vec![];
            }
        };

        if data == 0 {
            break;
        }

        let chunk: String = match str::from_utf8(&buffer[..data]) {
            Ok(d) => d.to_string(),
            Err(e) => {
                eprintln!("Failed to Convet Byte to Char");
                return vec![];
            }
        };
        let tmp_chars = left_over.clone() + &chunk;

        let iter: Vec<&str> = tmp_chars.split('\n').collect();
        let size = iter.len();
        for data in 0..size - 1 as usize {
            result.push(iter[data].to_string());
        }
        left_over = iter[size - 1].to_string();
    }
    result
}
fn main() -> io::Result<()> {
    let listen = match TcpListener::bind("127.0.0.1:8080") {
        Ok(d) => d,
        Err(e) => {
            eprint!("Tcp Error: {:?}", e);
            return Ok(());
        }
    };

    for stream in listen.incoming() {
        match stream {
            Ok(stream) => {
                eprintln!(" Data  +> {:?}", stream);
                let lines: Vec<String> = get_lines_reader(stream);
                for line in &lines {
                    eprint!("{}\n", line);
                }
            }
            Err(e) => println!("couldn't get client: {e:?}"),
        }
    }
    Ok(())
}
