// mod usb;
// mod tasks;

use anyhow::anyhow;
use std::env;
use std::path::Path;
use libnyb::{open_serial, start_io};



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder().format_timestamp(None).init();
    let args: Vec<String> = env::args().collect();

    let serial_name = args.get(1).ok_or(anyhow!("Not enough arguments!"))?;
    let serial_path = Path::new(serial_name);

    let serial = open_serial(serial_path)?;
    let (writer, mut reader) = start_io(serial)?;

    loop {
        if let Some(payload) = reader.recv().await {
            log::trace!("{:?}", payload);
            writer.send(payload).await?; 
        } else {
            return Err(anyhow!("Reader thread died!"));
        }
    }
}





