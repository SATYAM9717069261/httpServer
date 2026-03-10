#[cfg(test)]
mod tests {

    use crate::request::Request;
    use crate::server::route;

    #[test]
    fn test_hello_route() {

        let raw = b"GET /hello HTTP/1.1\r\nHost: localhost\r\n\r\n";

        let request = Request::parse(raw);

        let response = route(request);

        assert!(response.contains("Hello Satyam!"));
    }
}
