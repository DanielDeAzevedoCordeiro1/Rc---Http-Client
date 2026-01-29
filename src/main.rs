use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;


fn http_get(host: &str, path: &str, port: u16) -> Result<String, Box<dyn std::error::Error>> {

    let mut stream = TcpStream::connect(format!("{}:{}", host, port))?;

    let request = format!(
        "GET {} HTTP/1.1\r\n\
Host: {}\r\n\
Connection: close\r\n\
\r\n",
        path, host
    );

    stream.write_all(request.as_bytes())?;

    let mut reader = BufReader::new(stream);

    let mut status = String::new();
    reader.read_line(&mut status)?;
    println!("{}", status);

    loop {
        let mut headers = String::new();
        reader.read_line(&mut headers)?;

        if headers == "\r\n" { break; };

        println!("{}", headers);
    }

    let mut body = String::new();
    reader.read_to_string(&mut body)?;

    Ok(body)
}

fn main() {

}
