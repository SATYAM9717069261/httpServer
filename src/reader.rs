use std::io::Read;

pub fn read_to_parser<R, P>(mut reader: R, parser: &mut P) -> std::io::Result<()>
where
    R: Read,
    P: crate::parser::Parser,
{
    let mut buf = [0u8; 32];

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }

        let consumed = parser.parse(&buf[..n])?;

        if parser.is_done() {
            break;
        }
    }

    Ok(())
}
