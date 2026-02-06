use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use crate::http::{build_headers, HttpRequest, ServerAddress};

pub fn send_request(
    request: HttpRequest,
    server_info: ServerAddress,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(format!("{}:{}", server_info.host, server_info.port))?;

    let headers = build_headers(&request.body);

    let mut req = format!(
        "{} {} HTTP/1.1\r\nHost: {}\r\n{}",
        request.method.as_str(),
        request.path,
        server_info.host,
        headers,
    );

    if let Some(b) = request.body {
        req.push_str(&b);
    }

    stream.write_all(req.as_bytes())?;

    let mut reader = BufReader::new(stream);

    let mut status = String::new();
    reader.read_line(&mut status)?;

    loop {
        let mut header = String::new();
        reader.read_line(&mut header)?;
        if header.trim().is_empty() {
            break;
        }
    }

    let mut body_resp = String::new();
    reader.read_to_string(&mut body_resp)?;

    Ok(body_resp)
}