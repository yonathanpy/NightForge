pub fn parse(input: &str) -> &str {
    let input = input.trim().to_uppercase();

    if input.starts_with("USER") {
        "USER"
    } else if input.starts_with("PASS") {
        "PASS"
    } else if input.starts_with("LIST") {
        "LIST"
    } else if input.starts_with("GET") {
        "GET"
    } else if input.starts_with("PUT") {
        "PUT"
    } else {
        "UNKNOWN"
    }
}
