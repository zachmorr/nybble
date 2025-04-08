use std::io::Read;
use super::Channel;
use crate::ndb::Data;
use crate::ndb::PullData;
use anyhow::anyhow;
use log::{debug, trace, warn};
use anyhow::{Context, Result};


pub async fn pull_task(mut channel: Channel) -> Result<()> {
    let path = match channel.recv().await? {
        Data::Pull(PullData::Path(path)) => {
            trace!(target: "pull", "recv {:?}", path);
            path
        },
        variant => {
            let err = format!("expecting path packet, got {:?}", variant);
            warn!(target: "pull", "{err}");
            channel.send(Data::Err(err)).await.context("Sending err")?;
            return Ok(());
        },
    };
    
    debug!(target: "pull", "opening {}", path.display());
    if !path.exists() {
        return Err(anyhow!(format!("{} does not exist", path.display())));  
    }

    let mut file = std::fs::File::options().read(true).open(&path)?;
    channel.send(Data::Ok).await.context("Sending ok packet")?;

    debug!(target: "pull", "sending {}", path.display());
    let mut buffer = [0u8; 400]; // need to fix buffer size problems
    loop {
        match file.read(&mut buffer)? {
            0 => {
                channel.send(Data::Ok).await.context("Sending ok")?;
                break;
            }
            n => {
                let data = buffer[..n].to_vec().into_boxed_slice();
                channel.send(Data::Pull(PullData::Data(data))).await.context("sending data")?;
                match channel.recv().await? {
                    Data::Ok => {
                        continue;
                    },
                    variant => {
                        let err = format!("expecting ok packet, got {:?}", variant);
                        warn!(target: "push", "{err}");
                        channel.send(Data::Err(err)).await.context("Sending err")?;
                        return Ok(());
                    },
                }
            }
        }
    }
    Ok(())
}
