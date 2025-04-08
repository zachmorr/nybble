use anyhow::{anyhow, Result};
use tokio::sync::mpsc;
use crate::ndb::Packet;
use crate::ndb::Data;


#[derive(Debug)]
pub struct Channel {
    id: u32,
    rx: mpsc::Receiver<Data>,
    tx: mpsc::Sender<Packet>,
}

impl Channel {
    pub fn new(
        id: u32,
        writer: &mpsc::Sender<Packet>,
    ) -> (mpsc::Sender<Data>, Channel) {
        let (tx, rx) = mpsc::channel(10);
        (
            tx,
            Channel {
                id: id,
                rx: rx,
                tx: writer.clone(),
            },
        )
    }

    pub async fn recv(&mut self) -> Result<Data>
    {
        self.rx
            .recv()
            .await
            .ok_or_else(|| anyhow!("channel was closed!"))
    }

    pub async fn send(&self, data: Data) -> Result<()> 
    {
        let packet = Packet {
            id: self.id,
            data: data,
        };
        self.tx.send(packet).await?;
        Ok(())
    }
}
