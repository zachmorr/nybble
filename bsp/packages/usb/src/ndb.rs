use serde::{Deserialize, Serialize};
use tokio::sync::{RwLock, mpsc};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Packet {
    pub id: u32,
    pub data: Data,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Data {
    Ok,
    Err(String),
    Ctrl(CtrlData),
    Echo(EchoData),
    Pull(PullData),
    Push(PushData),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum CtrlData {
    OpenStream(u32),
    CloseStream(u32),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum EchoData {
    Data(Box<[u8]>)
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum PullData {
    Path(PathBuf),
    Data(Box<[u8]>),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum PushData {
    Path(PathBuf),
    Data(Box<[u8]>),
}