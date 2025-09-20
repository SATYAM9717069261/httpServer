use httpServer::request::RequestLine;

#[test]
fn test_request_line_parse_root() {
    let input = "GET / HTTP/1.1";
    let req = RequestLine::from_str(input).expect("should parse");
    assert_eq!(req.method, "GET");
    assert_eq!(req.request_target, "/");
    assert_eq!(req.http_version, "1.1");
}

#[test]
fn test_request_line_parse_path() {
    let input = "GET /coffee HTTP/1.1";
    let req = RequestLine::from_str(input).expect("should parse");
    assert_eq!(req.method, "GET");
    assert_eq!(req.request_target, "/coffee");
    assert_eq!(req.http_version, "1.1");
}

#[test]
fn test_request_line_parse_error() {
    let input = "INVALID_LINE";
    let err = RequestLine::from_str(input).unwrap_err();
    assert_eq!(err, "invalid request line");
}
