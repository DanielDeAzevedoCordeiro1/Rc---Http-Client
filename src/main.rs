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


fn http_post(host: &str, path: &str, port: u16, body: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(format!("{}:{}", host, port))?;
    let content_length = body.len();

    let request = format!(
        "POST {} HTTP/1.1\r\n\
Host: {}\r\n\
Content-Type: application/json\r\n\
Content-Length: {}\r\n\
Connection: close\r\n\
\r\n\
{}",
        path, host, content_length, body
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
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 6{
        panic!("Digite os argumentos na seguinte ordem ---> rc METODO HOST PATH PORTA BODY");
    }

    match args[1].as_str().to_uppercase().as_str() {
        "POST" => {
            let body = http_post(&args[2], &args[3], args[4].parse().unwrap(), &args[5]
            ).unwrap();
            println!("{}", body);
        },
        "GET" => {
            let body = http_get(&args[2], &args[3], args[4].parse().unwrap()).unwrap();
            println!("{}", body);
        },
        _ => panic!("Metodo invalido")
    }
}
