use super::Channel;
use anyhow::Result;
use log::{debug, trace};
use crate::ndb::Data;


pub async fn echo_task(mut channel: Channel) -> Result<()> {
    debug!(target: "echo", "starting");
    let packet: Data = channel.recv().await?;
    trace!(target: "echo", "echoing {:?}", packet);
    channel.send(packet).await?;
    debug!(target: "echo", "ending");
    Ok(())
}
