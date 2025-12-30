#[derive(Debug, PartialEq)]
pub enum ParserState {
    RequestLine,
    Headers,
    Done,
}
