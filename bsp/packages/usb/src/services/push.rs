use std::io::Write;
use super::Channel;
use crate::ndb::PushData;
use crate::ndb::Data;
use log::{debug, trace, warn};
use anyhow::{Context, Result, anyhow};


pub async fn push_task(mut channel: Channel) -> Result<()> {
    let path = match channel.recv().await? {
        Data::Push(PushData::Path(path)) => {
            trace!(target: "push", "recv {:?}", path);
            path
        },
        variant => {
            let err = format!("expecting path packet, got {:?}", variant);
            warn!(target: "push", "{err}");
            channel.send(Data::Err(err)).await.context("Sending err")?;
            return Ok(());
        },
    };

    debug!(target: "push", "opening {}", path.display());
    if path.is_dir() {
        return Err(anyhow!(format!("{} does not exist", path.display())));
    }

    let mut file = std::fs::File::options().write(true).create(true).open(&path)?;
    channel.send(Data::Ok).await.context("Sending ok packet")?;

    loop {
        match channel.recv().await? {
            Data::Push(PushData::Data(data)) => {
                trace!(target: "push", "recv {:?}", data);
                file.write_all(&data)?;
                channel.send(Data::Ok).await.context("Sending ok packet")?;
            },
            Data::Ok => break,
            variant => {
                let err = format!("expecting data packet, got {:?}", variant);
                warn!(target: "push", "{err}");
                channel.send(Data::Err(err)).await.context("Sending err")?;
                return Ok(());
            },
        }
    }

    debug!(target: "push", "done");
    Ok(())
}