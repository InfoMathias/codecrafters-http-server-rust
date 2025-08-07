#[allow(unused_imports)]
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
         match stream {
             Ok(ref _stream) => {
                println!("accepted new connection");
                let stream = stream.unwrap();
                handle_connection(stream);
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    let mut endpoint: String = String::new();

    if !http_request[0].is_empty() {
        endpoint = http_request[0]
            .split(" ")
            .nth(1)
            .unwrap_or_default()
            .to_string()
            .split("/")
            .nth(1)
            .unwrap_or_default()
            .to_string();
    }

    let mut response = "HTTP/1.1 200 OK\r\n\r\n";

    if endpoint != "" {
        response = "HTTP/1.1 404 Not Found\r\n\r\n";
    }

    stream.write_all(response.as_bytes()).unwrap();
}
