use super::Channel;
use log::{debug, trace};
use libnyb::{Echo, EchoError};

pub async fn echo_task(mut channel: Channel) -> Result<(), EchoError> {
    debug!(target: "echo", "starting");
    let packet: Echo = channel.recv().await;
    trace!(target: "echo", "echoing {:?}", packet);
    channel.send(packet).await;
    debug!(target: "echo", "ending");
    Ok(())
}
