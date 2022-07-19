use crate::proto::cl_rcon;
use crate::proto::sv_rcon;
use anyhow::{bail, Context, Result};
use cl_rcon::RequestT;
use prost::Message;
use std::thread;
use std::{io::ErrorKind, time::Duration};
use tokio::io::*;
use tokio::net::TcpStream;

const EXIT_PHRASES: [&'static str; 3] = ["q", "exit", "quit"];
const MAX_NETCON_INPUT_LEN: usize = 2048usize;

pub async fn start() -> Result<()> {
    let mut rl = rustyline::Editor::<()>::new()?;
    loop {
        println!(
            "Please enter the ip address and port to connect to (default is 127.0.0.1:37015): "
        );

        let raw = rl.readline("-> ")?;
        //Quit if the user wants
        if EXIT_PHRASES.contains(&raw.as_str()) {
            break;
        }
        let mut parts = raw.split(":");
        let addr = match parts.next() {
            Some("") => "127.0.0.1",
            Some(a) => a,
            None => anyhow::bail!("Target address was None"),
        };
        let port = match parts.next() {
            Some(p) => p,
            None => "37015",
        };

        //Try to connect, or ask for address again
        //TcpStream closes itself when dropped, so no need to shut it down manually
        let stream = match connect(&addr, &port).await {
            Ok(s) => s,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        let (mut read, mut write) = tokio::io::split(stream);
        let t1 = tokio::spawn(async move {
            loop {
                thread::sleep(Duration::from_millis(50));
                recv(&mut read).await;
            }
        });
        println!("Connected!");
        //Break from the loop if not true
        if !handle_input(&mut rl, &mut write).await? {
            break;
        }
    }

    Ok(())
}

///Try to connect to an RCON server
///
///Times out aftet 10 seconds
async fn connect(addr: &str, port: &str) -> Result<TcpStream> {
    println!("Attempting to connect...");
    let t = std::net::TcpStream::connect_timeout(
        &format!("{}:{}", addr, port)
            .parse()
            .with_context(|| format!("Coudln't parse address '{}:{}'", addr, port))?,
        Duration::from_secs(10),
    )
    .with_context(|| format!("Unable to connect to server at {}:{}", addr, port))?;
    TcpStream::from_std(t).context("Couldn't convert to tokio stream")
}

///Read user input. Quits on q, quit, or exit.
///
///Returns true if user asked to disconnect
async fn handle_input(
    rl: &mut rustyline::Editor<()>,
    stream: &mut WriteHalf<TcpStream>,
) -> Result<bool> {
    loop {
        let line = rl.readline("->")?;

        if EXIT_PHRASES.contains(&line.as_str()) {
            break;
        }

        if line == "disconnect" {
            return Ok(true);
        }

        if line.is_empty() {
            continue;
        }

        if let Some(buf) = parse_input(&line) {
            send(stream, buf).await?;
        }
    }

    Ok(false)
}

fn parse_input(raw: &str) -> Option<Vec<u8>> {
    //if this is None return None immediately
    let parts = raw.split_once(' ')?;
    match parts.0 {
        "PASS" => Some(serialize(parts.1, "", RequestT::ServerdataRequestAuth)),
        "SET" => Some(serialize(
            parts.0,
            parts.1,
            RequestT::ServerdataRequestSetvalue,
        )),
        _ => Some(serialize(raw, "", RequestT::ServerdataRequestAuth)),
    }
}

fn serialize(buf: &str, val: &str, t: cl_rcon::RequestT) -> Vec<u8> {
    let mut req = cl_rcon::Request::default();
    req.request_id = Some(-1);
    req.set_request_type(t);

    match t {
        RequestT::ServerdataRequestAuth | RequestT::ServerdataRequestSetvalue => {
            req.request_buf = Some(buf.to_string());
            req.request_val = Some(val.to_string());
        }

        RequestT::ServerdataRequestExeccommand => {
            req.request_buf = Some(buf.to_string());
        }
        _ => {}
    }
    req.encode_to_vec()
}

///Send a message to the stream
#[inline]
async fn send(stream: &mut tokio::io::WriteHalf<TcpStream>, buf: Vec<u8>) -> Result<()> {
    stream
        .write_all(&buf)
        .await
        .with_context(|| format!("Failed to write message to socket"))?;
    Ok(())
}

async fn recv(stream: &mut ReadHalf<TcpStream>) -> Result<()> {
    let mut buf = String::with_capacity(MAX_NETCON_INPUT_LEN);

    loop {
        match stream.read_to_string(&mut buf).await {
            Ok(0) => {
                println!("Server disconnected");
                return Ok(());
            }
            Ok(_n) => continue,
            Err(e) => {
                if e.kind() == ErrorKind::WouldBlock {
                    break;
                } else {
                    bail!("Stream encountered an error: {:#?}", e);
                }
            }
        }
    }

    Ok(())
}

fn process_rec(buf: &str) -> Result<()> {
    let mut acc = vec![];
    for c in buf.chars() {
        if c == '\r' {
            let res = sv_rcon::Response::decode(&*acc)?;
            handle_message(res)?;
            acc.clear();
            continue;
        }
        acc.push(c as u8);
    }

    Ok(())
}

fn handle_message(msg: sv_rcon::Response) -> Result<()> {
    use sv_rcon::ResponseT;

    match msg.response_type() {
        ResponseT::ServerdataResponseAuth | ResponseT::ServerdataResponseConsoleLog => {
            println!("{}", msg.response_buf());
        }

        _ => {}
    }

    Ok(())
}
