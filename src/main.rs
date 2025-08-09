#[allow(unused_imports)]
use std::{
    net::{TcpListener, TcpStream},
};

mod routes;
mod router;
use crate::router::Router;

fn main() {

    let mut router = Router::new();
    routes::build_routes(&mut router);
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
         match stream {
             Ok(ref _stream) => {
                println!("accepted new connection");
                let stream = stream.unwrap();
                router.respond(&stream);
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
    }
}
