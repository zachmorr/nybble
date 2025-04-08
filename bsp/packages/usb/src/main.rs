#[allow(unused)]
mod ndb;
mod usb;
mod services;

use anyhow::{anyhow, Context, Result, Error};
use bincode::Options;
use log::{debug, trace, warn, error};
use std::collections::HashMap;
use std::env;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tokio::task::spawn_blocking;
use usb::Function;
use ndb::{CtrlData, Data, Packet};
use std::pin::Pin;
use std::future::Future;

pub type StreamID = u32;
pub type StreamMap = Arc<RwLock<HashMap<StreamID, mpsc::Sender<Data>>>>;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder().format_timestamp(None).init();
    let args: Vec<String> = env::args().collect();
    let ffs_dir_arg = args.get(1).expect("Not enough arguments!");
    let ffs_dir = Path::new(ffs_dir_arg);

    let mut function = Function::new(ffs_dir)
        .context(format!("opening function at {}", ffs_dir.display()))?;

    function
        .write_descriptors()
        .context(format!("writing descriptors"))?;

    let (writer, reader) = function
        .open_endpoints()
        .context("opening endpoints")?;

    // Start reader
    let (reader_tx, mut reader_rx) = mpsc::channel(10);
    spawn_blocking(|| {
        if let Err(err) = reader_thread(reader, reader_tx) {
            error!(target: "main", "reader thread died with {err}");
        }
    });

    // Start writer
    let (writer_tx, writer_rx) = mpsc::channel(10);
    spawn_blocking(|| {
        if let Err(err) = writer_thread(writer, writer_rx) {
            error!(target: "main", "writer thread died with {err}");
        }
    });

    // Start event listener 
    spawn_blocking(move || {
        loop {
            debug!(target: "event", "{:?}", function.event());
        }
    });

    // Start packet multiplexer
    let task_map: StreamMap = Arc::new(RwLock::new(HashMap::new()));
    loop {
        let packet = reader_rx.recv().await
            .ok_or_else(|| anyhow!("channel was closed!"))?;
        trace!(target: "main", "recv {:?}", packet);

        if packet.id == 0 { // Handle control packet
            let writer_clone = writer_tx.clone();
            let task_map_clone = task_map.clone();

            tokio::spawn(async move {
                if let Err(err) = handle_ctrl_packet(
                    packet.data,
                    task_map_clone,
                    &writer_clone
                ).await.context("handling control packet") {
                    let err_data = Data::Err(format!("{err:#}"));
                    let err_packet = Packet {
                        id: 0,
                        data: err_data
                    };
                    let _res = writer_clone.send(err_packet).await;    
                };
            });
        } else { // Forward packet
            let id = packet.id;
            match task_map.read().await.get(&id) {
                Some(tx) => {
                    debug!(target: "main", "forwarding to {id}");
                    tx.send(packet.data).await
                        .context(format!("failed to send packet to {id}"))?;
                }
                None => {
                    warn!(target: "main", "stream {id} not open, dropping");
                }
            }
        }
    }
}



async fn handle_ctrl_packet(
    data: Data,
    task_map: StreamMap,
    writer: &mpsc::Sender<Packet>,
) -> Result<()> {
    match data {
        Data::Ctrl(CtrlData::OpenStream(task_id)) => {
            open_stream(task_id, &task_map, &writer).await?;
        },
        command => {
            return Err(anyhow!(format!("unknown command {:?}", command)));
        }
    }
    Ok(())
}

async fn open_stream(
    task_id: u32,
    task_map: &StreamMap,
    writer: &mpsc::Sender<Packet>,
) -> Result<()> {
    let stream_id = find_available_stream_id(&task_map).await?;

    debug!(target: "ctrl", "creating stream {stream_id} channel to {task_id}");
    let (tx, task_channel) = services::Channel::new(stream_id, &writer);
    let task: Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> = match task_id {
        0 => Box::pin(services::echo_task(task_channel)),
        1 => Box::pin(services::pull_task(task_channel)),
        2 => Box::pin(services::push_task(task_channel)),
        _ => {
            let err = format!("unknown task num {task_id}");
            warn!(target: "ctrl", "{err}");
            ctrl_send(&writer, Data::Err(err)).await?;
            return Ok(());
        }
    };

    task_map.write().await.insert(stream_id, tx);
    let task_map_clone = task_map.clone();
    let writer_clone = writer.clone();
    tokio::spawn(async move {
        if let Err(err) = task.await {
            warn!(target: "ctrl", "task {stream_id} error: {err}");
            let err_data = Data::Err(format!("{err:#}"));
            let err_packet = Packet {
                id: stream_id,
                data: err_data
            };
            let _res = writer_clone.send(err_packet).await;            
        };
        debug!("removing stream {} from map", stream_id);
        task_map_clone.write().await.remove(&stream_id);
    });

    let response = Data::Ctrl(CtrlData::OpenStream(stream_id));
    ctrl_send(&writer, response).await?;
    Ok(())
}

async fn ctrl_send(
    writer: &mpsc::Sender<Packet>,
    data: Data,
) -> Result<()> {
    writer
        .send(Packet {
            id: 0,
            data: data,
        })
        .await?;
    Ok(())
}

async fn find_available_stream_id(task_map: &StreamMap) -> Result<u32> {
    let task_map_guard = task_map.read().await;
    (100..=u32::MAX)
        .find(|n| !task_map_guard.contains_key(&n))
        .ok_or_else(|| anyhow!("no stream IDs available?!?"))
}


fn reader_thread(
    mut reader: std::fs::File,
    channel: mpsc::Sender<Packet>,
) -> Result<()> {
    let options = bincode::DefaultOptions::new()
        .with_no_limit()
        .with_fixint_encoding()
        .with_little_endian()
        .allow_trailing_bytes();

    let mut raw_bytes = Vec::new();
    loop {
        let mut buf = [0; 512];
        let n = reader.read(&mut buf)?;
        raw_bytes.extend_from_slice(&buf[..n]);
        trace!(target: "reader", "buffer {:?}", &raw_bytes);

        loop {
            match options.deserialize(&raw_bytes) {
                Ok(packet) => {
                    trace!(target: "reader", "read {packet:?}");
                    let consumed_bytes = options.serialized_size(&packet)?;
                    raw_bytes = raw_bytes.split_off(consumed_bytes as usize);
                    channel.blocking_send(packet)?;
                },
                Err(err) => {
                    match *err {
                        bincode::ErrorKind::Io(_) => {},
                        _ => {
                            warn!(target: "reader", "dropping packet: {err:#}");
                            raw_bytes = Vec::new();
                        },
                    }
                    break;
                },
            }
        }
    }
}


fn writer_thread(
    mut writer: std::fs::File,
    mut channel: mpsc::Receiver<Packet>,
) -> Result<()> {
    let options = bincode::DefaultOptions::new()
        .with_no_limit()
        .with_fixint_encoding()
        .with_little_endian()
        .reject_trailing_bytes();
    
    loop {
        let packet = channel.blocking_recv()
            .ok_or_else(|| anyhow!("channel was closed!"))?;

        trace!(target: "writer", "writing {packet:?}");
        let raw_bytes = match options.serialize(&packet) {
            Ok(bytes) => bytes,
            Err(err) => {
                warn!("dropping packet, serialization error {err:#}");
                continue;
            }
        };
        if let Err(err) = writer.write_all(&raw_bytes) {
            warn!("write error {err:#}");
            continue;
        }
    }
}