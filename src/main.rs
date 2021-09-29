use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    //Iniciar el servidor
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(&address).unwrap();
    println!("Servidor iniciado en {}", &address);

    //Escuchar por conexiones
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

//Manejas conexiones
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("Stream recibido!");
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET /index HTTP/1.1";
    let content: String;
    if buffer.starts_with(get) {
        content = fs::read_to_string("index.html").unwrap();
    } else {
        content = fs::read_to_string("404.html").unwrap();
    }
    stream.write(build_response(content).as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn build_response(content: String) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    )
}
