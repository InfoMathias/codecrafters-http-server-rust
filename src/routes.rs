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

    router.add_route(
        "/user-agent",
        "GET",
        Box::new(|params: &HashMap<String, String>| {
            let binding = "".to_string();
            let agent = params.get("user-agent").unwrap_or(&binding);
            format_user_agent(agent)
        })
    )
}

fn echo(val: &str) -> String {
    println!("{}", val);

    format!(
        "\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        val.len(),
        val
    )
}

fn format_user_agent(agent: &str) -> String {
    println!("{}", agent);

    format!(
        "\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        agent.len(),
        agent
    )
}
