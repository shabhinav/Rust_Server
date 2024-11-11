pub fn parse_request_line(request_line: &str) -> (&str, &str, &str) {
    let mut parts = request_line.split_whitespace();
    (
        parts.next().unwrap_or(""),
        parts.next().unwrap_or(""),
        parts.next().unwrap_or(""),
    )
}
