#[allow(unused_imports)]
use std::{
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};

mod routes;
mod router;
use crate::router::Router;

fn main() {
    let mut router = Router::new();
    routes::build_routes(&mut router);

    // Share router safely across threads
    let router = Arc::new(router);

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        let router = Arc::clone(&router); // clone Arc for this thread

        thread::spawn(move || {
            match stream {
                Ok(stream) => {
                    println!("accepted new connection");
                    router.respond(&stream);
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        });
    }
}

