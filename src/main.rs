use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

pub struct Request<'a> {
    method: &'a str,
    host: &'a str,
    path: &'a str,
    port: u16,
    body: Option<&'a str>,
}

impl<'a> Request<'a> {
    pub fn  new(method: &'a str, host: &'a str, path: &'a str, port: u16, body: Option<&'a str>) -> Self {
        Self { method , host, path, port, body }
    }
}


fn http_get(request: Request) -> Result<String, Box<dyn std::error::Error>> {

    let mut stream = TcpStream::connect(format!("{}:{}", request.host, request.port))?;

    let request = format!(
        "GET {} HTTP/1.1\r\n\
Host: {}\r\n\
Connection: close\r\n\
\r\n",
        request.path, request.host
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


fn http_post(request: Request) -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(format!("{}:{}", request.host, request.port))?;
    let content_length = request.body.unwrap().len();

    let request = format!(
        "POST {} HTTP/1.1\r\n\
Host: {}\r\n\
Content-Type: application/json\r\n\
Content-Length: {}\r\n\
Connection: close\r\n\
\r\n\
{}",
        request.path, request.host, content_length, request.body.unwrap()
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

fn http_delete(request: Request) -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(format!("{}:{}", request.host, request.port))?;

    let request = format!(
        "DELETE {} HTTP/1.1\r\n\
Host: {}\r\n\
Connection: close\r\n\
\r\n\
",
        request.path, request.host
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

fn http_put(request: Request) -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(format!("{}:{}", request.host, request.port))?;
    let content_length = request.body.unwrap().len();

    let request = format!(
        "PUT {} HTTP/1.1\r\n\
Host: {}\r\n\
Content-Type: application/json\r\n\
Content-Length: {}\r\n\
Connection: close\r\n\
\r\n\
{}",
        request.path, request.host, content_length, request.body.unwrap()
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
    }

    let mut body = String::new();
    reader.read_to_string(&mut body)?;
    Ok(body)

}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!("{:?}", args);

    if args.len() > 6{
        panic!("Digite os argumentos na seguinte ordem ---> rc METODO HOST PATH PORTA BODY");
    }

    let body = if args.len() == 6 {
        Some(&args[5] as &str)
    } else {
        None
    };

    let request_args = Request::new(
        &args[1],
        &args[2],
        &args[3],
        args[4].parse::<u16>().unwrap(),
        body
    );

    match request_args.method.to_uppercase().as_str() {
        "POST" => {
            let body = http_post(request_args).unwrap();
            println!("{}", body);
        },
        "GET" => {
            let body = http_get(request_args).unwrap();
            println!("{}", body);
        },
        "PUT" => {
            let body = http_put(request_args).unwrap();
            println!("{}", body);
        },
        "DELETE" => {
            let body = http_delete(request_args).unwrap();
            println!("{}", body);
        }
        _ => panic!("Metodo invalido")
    }
}
