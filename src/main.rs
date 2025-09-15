use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = match File::open("message.txt") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return Ok(());
        }
    };
    let mut buffer = [0u8; 8];
    let mut left_over = String::new();
    loop {
        let data = match file.read(&mut buffer) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to Read {} ", e);
                return Ok(());
            }
        };

        if data == 0 {
            break;
        }

        let mut chunk: String = match str::from_utf8(&buffer[..data]) {
            Ok(d) => d.to_string(),
            Err(e) => {
                eprintln!("Failed to Convet Byte to Char");
                return Ok(());
            }
        };
        let tmp_chars = left_over.clone() + &chunk;

        let iter: Vec<&str> = tmp_chars.split('\n').collect();
        let size = iter.len();
        for data in 0..size - 1 as usize {
            eprintln!(" ==> {:?}", iter[data]);
        }
        left_over = iter[size - 1].to_string();
    }

    if !left_over.is_empty() {
        eprintln!("end ==> {:?}", left_over);
    }

    Ok(())
}
