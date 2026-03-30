use crate::messages::{client_send, Start};
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};
use anyhow::{anyhow, Result};


pub fn run() -> Result<()> {
    let stream = connect_to_server()?;
    client_send(stream, Start(
        PathBuf::from("/dev/ttyACM0"), 
        PathBuf::from("/bin/ls"), 
    ))?;
    Ok(())
}

fn connect_to_server() -> Result<TcpStream> {
    let addr = SocketAddr::from(([127, 0, 0, 1], libnyb::PORT));
    match TcpStream::connect(addr) {
        Ok(stream) => Ok(stream),
        Err(err) => {
            match err.kind() {
                std::io::ErrorKind::ConnectionRefused => exec_server(),
                _ => Err(err.into()),    
            }
        },
    }
}

fn exec_server() -> Result<TcpStream> {
    let nyb_path = std::env::current_exe()
        .expect("Failed to get the path of the currently running process");

    println!("Server not detected, starting server on port {}...", libnyb::PORT);
    let _ = Command::new(nyb_path)
        .arg("server")
        .env("RUST_LOG", "trace")
        .current_dir(Path::new("/"))
        .spawn()?;


    let timeout = Duration::from_secs(2);
    let start = Instant::now();

    while start.elapsed() < timeout {
        let addr = SocketAddr::from(([127, 0, 0, 1], libnyb::PORT));
        if let Ok(stream) = TcpStream::connect(addr) {
            return Ok(stream);
        }

        std::thread::sleep(Duration::from_millis(500)); // prevent tight spin
    }
    
    Err(anyhow!("Timeout while waiting for server to start"))
}


