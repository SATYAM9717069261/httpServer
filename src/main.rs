use std::fs::File;
use std::io::{self, Read};
fn get_lines_reader(mut file: File) -> Vec<String> {
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
    let file = match File::open("message.txt") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return Ok(());
        }
    };
    let lines: Vec<String> = get_lines_reader(file);
    for line in &lines {
        eprint!("{}\n", line);
    }
    Ok(())
}
