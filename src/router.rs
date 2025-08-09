use std::{
    net::{TcpStream},
    io::{BufReader, prelude::*},
    collections::{HashMap},
};

type Handler = Box<dyn Fn(&HashMap<String, String>) -> String + Send + Sync>;

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

        let (method, path, headers) = Self::parse_routing_args(&stream);
        let (status, body) = self.handle(&path, &method, &headers); 
        
        let response = format!("HTTP/1.1 {} {}", status.to_string(), body);

        println!("{}", response);

        stream.write_all(response.as_bytes()).unwrap();
    }

    pub fn handle(
        &self, 
        path: &str, 
        method: &str,
        headers: &HashMap<String, String>
    ) -> (u16, String) {

        println!("{} {}", method, path);

        for (route_path, methods_map) in &self.routes {
            if let Some(mut params) = Self::match_path(route_path, path) {

                for (k, v) in headers {
                    params.insert(k.clone(), v.clone());
                }

                println!("{:?}", headers);
                println!("{:?}", params);

                if let Some(handler) = methods_map.get(method) {
                    return (200, format!("OK{}", handler(&params)));
                } else {
                    let allowed: Vec<String> = methods_map.keys().cloned().collect();
                    return (
                        405,
                        format!("Method Not Allowed\nAllow: {}", allowed.join(", ")),
                    );
                }
            }
        }

        (404, "Not Found\r\n\r\n".to_string())
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

    fn parse_routing_args(stream: &TcpStream) -> (String, String, HashMap<String, String>) {
        let buf_reader = BufReader::new(stream);

        let http_request: Vec<String> = buf_reader
            .lines()
            .map(|result| result.unwrap_or_default())
            .take_while(|line| !line.is_empty())
            .collect();

        if http_request.is_empty() {
            return (String::new(), String::new(), HashMap::new());
        }

        let mut parts = http_request[0].split_whitespace();
        let method = parts.next().unwrap_or_default().to_string();
        let path = parts.next().unwrap_or_default().to_string();

        let mut headers_map = HashMap::new();
        for line in http_request.iter().skip(1) { 
            if let Some((key, value)) = line.split_once(':') {
                headers_map.insert(key.trim().to_lowercase(), value.trim().to_string());
            }
        }

        (method, path, headers_map)
    }
}
