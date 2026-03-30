use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::net::SocketAddr;
use std::os::fd::IntoRawFd;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use anyhow::Result;
use daemonize::Daemonize;
use libnyb::{open_serial, writer_thread, Payload};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Runtime;
use tokio::spawn;
use tokio::sync::mpsc::{self, Sender};
use tokio::task::spawn_blocking;
use crate::devices::find_nybble;
use crate::messages::{server_recv, Start};

type ConnectionMap = Arc<Mutex<HashMap<PathBuf, Sender<Payload>>>>;


pub fn server() -> Result<()> {
    // Daemonize::new()
    //     .working_directory("/")
    //     .start()?;

    // let pid = std::process::id();
    // let file = fs::File::create(&format!("/tmp/nyb.{pid}.log")).unwrap();
    // let fd = file.into_raw_fd();
    // unsafe {
    //     libc::dup2(fd, libc::STDERR_FILENO);
    // }
    log::info!("Daemon started");

    let runtime = Runtime::new().expect("Failed to create Tokio runtime");
    runtime.block_on(async {
        let connection_map: ConnectionMap = Arc::new(Mutex::new(HashMap::new()));
        if let Err(err) = connection_manager(connection_map.clone()).await {
            log::error!("Connection manager died with error: {err}")
        }
    });

    // std::process::exit(0);
    Ok(())
}


async fn connection_manager(map: ConnectionMap) -> Result<()> {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        let mut map = map.lock().unwrap();
        for dev in find_nybble() {
            if !map.contains_key(&dev) {
                log::info!("Detected Nybble at {dev:?}");
                let serial = open_serial(&dev)?;
                let (writer_tx, mut writer_rx) = mpsc::channel(10);
                map.insert(dev.clone(), writer_tx);
                
                let mut writer_file = serial.try_clone()?;
                let dev_clone = dev.clone();
                let mut map_clone = map.clone();
                spawn_blocking(move || {
                    log::info!("{dev_clone:?} writer started");
                    while let Some(data) = writer_rx.blocking_recv() {
                        if let Err(err) = writer_file.write(&data) {
                            log::error!("Error writing to {dev_clone:?}: {err}");
                            break;
                        } else {
                            log::trace!(target: &format!("{dev_clone:?}"), "wrote {:?}", data);
                        }
                    }
                    log::warn!("{dev_clone:?} disconnected");
                    map_clone.remove(&dev_clone);
                });
            }
        }
    }
}


async fn start_listening() -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], libnyb::PORT));
    let listener = TcpListener::bind(addr).await?;
    log::info!("Listening on {}", addr);

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async {
            if let Err(err) = handle_client(socket).await {
                log::error!("{:?}", err);
            }
        });
    }
}

async fn handle_client(stream: TcpStream) -> Result<()> {
    log::info!("Handling client");
    let Start(dev, bin) = server_recv(stream).await?;
    log::debug!("{:?} {:?}", dev, bin);
    Ok(())
}
