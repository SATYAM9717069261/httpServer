use std::io::Result;

pub trait Parser {
    fn parse(&mut self, input: &[u8]) -> Result<usize>;
    fn is_done(&self) -> bool;
}
