use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::Router;

pub fn build_routes(router: &mut Router) {
    router.add_route(
        "/", 
        "GET",
        Box::new(|_params: &HashMap<String, String>| (200, "OK\r\n\r\n".to_string())),
    );

    router.add_route(
        "/echo/:str",
        "GET", 
        Box::new(|params: &HashMap<String, String>| {
            let binding = "".to_string();
            let path = params.get("str").unwrap_or(&binding);
            echo(path)
        })
    );

    router.add_route(
        "/user-agent",
        "GET",
        Box::new(|params: &HashMap<String, String>| {
            let binding = "".to_string();
            let agent = params.get("user-agent").unwrap_or(&binding);
            format_user_agent(agent)
        })
    );

    router.add_route(
        "files/:filename",
        "GET",
        Box::new(|params: &HashMap<String, String>| {
            let binding = "".to_string();
            let file_name = params.get("filename").unwrap_or(&binding);
            let directory = params.get("directory").unwrap_or(&binding);
            send_file_content(file_name, directory)
        })
    );
}

fn echo(val: &str) -> (u16, String) {
    println!("{}", val);

    (200, format!(
        "OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        val.len(),
        val
    ))
}

fn format_user_agent(agent: &str) -> (u16, String) {
    println!("{}", agent);

    (200, format!(
        "OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        agent.len(),
        agent
    ))
}

fn send_file_content(file_name: &str, directory: &str) -> (u16, String) {
    println!("{}", file_name);
    println!("{}", directory);

    
    let path_str = format!("{}{}", directory, file_name);
    let path = Path::new(&path_str);
    
    if !path.exists() {
        return (404, "Not Found\r\n\r\n".to_string()) 
    }

    let file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return (500, "Internal Server Error\r\n\r\n".to_string()),
    };

    (
        200, 
        format!(
            "OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}", 
            file_content.len(), 
            file_content
        ) 
    )
}
