pub fn build_headers(body: &Option<String>) -> String {
    match body {
        None => String::from("Connection: close\r\n\r\n"),

        Some(b) if b.is_empty() => String::from("Connection: close\r\n\r\n"),

        Some(b) => format!(
            "Content-Type: application/json\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\r\n",
            b.len()
        ),
    }
}