use httpServer::request::{Request, RequestLine};

#[test]
fn test_request_line_parse_root() {
    let input =
        "GET / HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n";
    let req = RequestLine::from_str(input).expect("should parse");
    assert_eq!(req.method, "GET");
    assert_eq!(req.request_target, "/");
    assert_eq!(req.http_version, "1.1");
}

#[test]
fn test_request_line_parse_path() {
    let input = "GET /coffee HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n";
    let req = RequestLine::from_str(input).expect("should parse");
    assert_eq!(req.method, "GET");
    assert_eq!(req.request_target, "/coffee");
    assert_eq!(req.http_version, "1.1");
}

#[test]
fn test_request_line_parse_error() {
    let input = "/coffee HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\nE";
    let err = RequestLine::from_str(input).unwrap_err();
    assert_eq!(err, "invalid request line");
}

#[test]
fn test_request_line_parse_root() {
    let input =
        "GET / HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n";
    let req = Request::request_from_reader(input).expect("should parse");
    assert_eq!(req.request_line.method, "GET");
    assert_eq!(req.request_line.request_target, "/");
    assert_eq!(req.request_line.http_version, "1.1");
}

#[test]
fn test_request_line_parse_path() {
    let input = "GET /coffee HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n";
    let req = Request::request_from_reader(input).expect("should parse");
    assert_eq!(req.request_line.method, "GET");
    assert_eq!(req.request_line.request_target, "/coffee");
    assert_eq!(req.request_line.http_version, "1.1");
}

#[test]
fn test_request_line_parse_error() {
    let input = "/coffee HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\nE";
    let err = Request::request_from_reader(input).unwrap_err();
    assert_eq!(err, "invalid request line");
}
