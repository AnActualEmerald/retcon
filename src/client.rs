use std::{
    io::Write,
    net::{SocketAddr, TcpStream},
    time::Duration,
};

use anyhow::{Context, Result};

const EXIT_PHRASES: [&'static str; 3] = ["q", "exit", "quit"];

pub fn start() -> Result<()> {
    let mut rl = rustyline::Editor::<()>::new()?;
    println!("Please enter the ip address and port to connect to (default is 127.0.0.1:37015): ");

    let raw = rl.readline("-> ")?;
    let mut parts = raw.split(":");
    let addr = parts.next().unwrap();
    let port = match parts.next() {
        Some(p) => p,
        None => "37015",
    };

    let stream = connect(&addr, &port)?;
    println!("Connected!");
    handle_input(&mut rl)?;

    Ok(())
}

fn connect(addr: &str, port: &str) -> Result<TcpStream> {
    println!("Attempting to connect...");
    TcpStream::connect_timeout(
        &format!("{}:{}", addr, port)
            .parse()
            .context("Failed to parse server address")?,
        Duration::new(5, 0),
    )
    .with_context(|| format!("Unable to connect to server at {}:{}", addr, port))
}

fn handle_input(rl: &mut rustyline::Editor<()>) -> Result<()> {
    loop {
        let line = rl.readline("->")?;

        if EXIT_PHRASES.contains(&line.as_str()) {
            break;
        }
    }

    Ok(())
}
