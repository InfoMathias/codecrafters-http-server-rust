use std::collections::HashMap;

use crate::Router;

pub fn build_routes(router: &mut Router) {
    router.add_route(
        "/", 
        "GET",
        Box::new(|_params: &HashMap<String, String>| "\r\n\r\n".to_string()),
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
}

fn echo(val: &str) -> String {
    println!("{}", val);

    format!(
        "\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        val.len(),
        val
    )
}
