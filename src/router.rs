use std::{
    net::{TcpStream},
    io::{BufReader, prelude::*},
    collections::{HashMap},
};

type Handler = Box<dyn Fn(&HashMap<String, String>) -> String>;

pub struct Router {
    routes: HashMap<String, HashMap<String, Handler>>,
}

impl Router {    

    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, path: &str, method: &str, handler: Handler) {
            self.routes
                .entry(path.to_string())
                .or_insert_with(HashMap::new)
                .insert(method.to_string(), Box::new(handler));
    }

    pub fn respond(&self, mut stream: &TcpStream) {

        let (method, path) = Self::parse_routing_args(&stream);
        let (status, body) = self.handle(&path, &method); 
        
        let response = format!("HTTP/1.1 {} {}", status.to_string(), body);

        println!("{}", response);

        stream.write_all(response.as_bytes()).unwrap();
    }

    pub fn handle(&self, path: &str, method: &str) -> (String, String) {
        println!("{} {}", method, path);

        for (route_path, methods_map) in &self.routes {
            if let Some(params) = Self::match_path(route_path, path) {
                if let Some(handler) = methods_map.get(method) {
                    return ("200".to_string(), handler(&params));
                } else {
                    let allowed: Vec<String> = methods_map.keys().cloned().collect();
                    return (
                        "405".to_string(),
                        format!("Method Not Allowed\nAllow: {}", allowed.join(", ")),
                    );
                }
            }
        }

        ("404".to_string(), "Not Found\r\n\r\n".to_string())
    }

    fn match_path(route: &str, actual: &str) -> Option<HashMap<String, String>> {
        let route_parts: Vec<&str> = route.trim_matches('/').split('/').collect();
        let actual_parts: Vec<&str> = actual.trim_matches('/').split('/').collect();

        if route_parts.len() != actual_parts.len() {
            return None;
        }

        let mut params = HashMap::new();

        for (r, a) in route_parts.iter().zip(actual_parts.iter()) {
            if r.starts_with(':') {
                params.insert(r[1..].to_string(), a.to_string());
            } else if r != a {
                return None;
            }
        }

        println!("{:?}", params);

        Some(params)
    }

    fn parse_routing_args(stream: &TcpStream) -> (String, String) {
        let buf_reader = BufReader::new(stream);

        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap_or_default())
            .take_while(|line| !line.is_empty())
            .collect();

        http_request
            .get(0)
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut parts = line.split_whitespace();
                (
                    parts.next().unwrap_or_default().to_string(),  // Method
                    parts.next().unwrap_or_default().to_string(),  // Path
                )
            })
            .unwrap_or_else(|| (String::new(), String::new()))
    }
}
