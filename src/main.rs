use crate::client::send_request;
use crate::http::{HttpRequest, Method, ServerAddress};

mod client;
mod http;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 5 || args.len() > 6 {
        panic!("Uso: rc METODO HOST PATH PORTA [BODY]");
    }

    let body = if args.len() == 6 {
        Some(args[5].clone())
    } else {
        None
    };

    let method = Method::from(args[1].as_str());

    let server_info = ServerAddress::from([args[2].as_str(), args[4].as_str()]);

    let request = HttpRequest::new(method, args[3].clone(), body);

    match send_request(request, server_info) {
        Ok(resp_body) => println!("{}", resp_body),
        Err(e) => eprintln!("Erro: {}", e),
    }
}
