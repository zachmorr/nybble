use std::{fs::File, path::Path};
use anyhow::{anyhow, Result};
use tokio::{sync::mpsc::{self, Receiver, Sender}, task::spawn_blocking};
use bincode::config::{FixintEncoding, Infinite, LittleEndian, WithOtherEndian, WithOtherIntEncoding, WithOtherLimit};
use bincode::{DefaultOptions, Options};
use termios::{self, Termios};
use std::os::fd::AsRawFd;


pub const PORT: u16 = 1930;
pub const PRODUCT_ID: u16 = 0x1234;
pub const VENDOR_ID: u16 = 0xABCD;
pub const MAX_SIZE: u16 = 512;
pub const EP_IN_ADDR: u8 = 0x81;
pub const EP_OUT_ADDR: u8 = 0x01;

pub type ID = u32;
pub type Payload = Box<[u8]>;
pub type SerDes = WithOtherEndian<WithOtherIntEncoding<WithOtherLimit<DefaultOptions, Infinite>, FixintEncoding>, LittleEndian>;

pub fn serdes() -> SerDes {
    bincode::DefaultOptions::new()
        .with_no_limit()
        .with_fixint_encoding()
        .with_little_endian()
}


pub fn open_serial(
    tty: &Path
) -> std::io::Result<File> {
    let serial = std::fs::File::options()
        .read(true)
        .write(true)
        .open(tty)?;
    

    // set to raw mode
    let serial_fd = serial.as_raw_fd();
    let mut termios = Termios::from_fd(serial_fd)?;
    termios::cfmakeraw(&mut termios);
    termios::tcsetattr(serial_fd, termios::TCSANOW, &termios)?;
    Ok(serial)
}

pub fn start_io(
    name: String,
    serial: File
) -> std::io::Result<(Sender<Payload>, Receiver<Payload>)> {
    let name_clone = name.clone();
    let writer_file = serial.try_clone()?;
    let (writer_tx, writer_rx) = mpsc::channel(10);
    spawn_blocking(|| {
        let _ = writer_thread(name_clone, writer_file, writer_rx);
    });

    let name_clone = name.clone();
    let reader_file = serial.try_clone()?;
    let (reader_tx, reader_rx) = mpsc::channel(10);
    spawn_blocking(|| {
        let _ = reader_thread(name_clone, reader_file, reader_tx);
    });

    Ok((writer_tx, reader_rx))
}

pub fn reader_thread(
    name: String,
    mut file: File,
    sender: mpsc::Sender<Payload>,
) -> Result<()> {
    loop {
        let packet = serdes().deserialize_from(&mut file)?;
        log::trace!(target: &name, "read {:?}", packet);
        sender.blocking_send(packet)?;
    }
}

pub fn writer_thread(
    name: String,
    mut file: File,
    mut receiver: mpsc::Receiver<Payload>,
) -> Result<()> { 
    loop {
        let packet = receiver.blocking_recv()
            .ok_or(anyhow!("channel closed!"))?;

        log::trace!(target: &name, "wrote {:?}", packet);
        serdes().serialize_into(&mut file, &packet)?;
    }
}


// pub fn serialize<T: Serialize>(data: &T) -> bincode::Result<Box<[u8]>> {
//     let options = bincode::DefaultOptions::new()
//         .with_no_limit()
//         .with_fixint_encoding()
//         .with_little_endian();

//     Ok(options.serialize(data)?.into_boxed_slice())
// }

// pub fn deserialize<T: DeserializeOwned>(data: &Box<[u8]>) -> bincode::Result<T> {
//     let options = bincode::DefaultOptions::new()
//         .with_no_limit()
//         .with_fixint_encoding()
//         .with_little_endian()
//         .reject_trailing_bytes();

//     Ok(options.deserialize(data)?)
// }


// async fn find_available_stream_id(
//     task_map: &StreamMap
// ) -> Result<u32> {
//     let task_map_guard = task_map.read().await;
//     (100..=u32::MAX)
//         .find(|n| !task_map_guard.contains_key(&n))
//         .ok_or_else(|| anyhow!("no stream IDs available?!?"))
// }

// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// pub struct Ctrl (pub ID, pub Command);

// #[non_exhaustive]
// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// pub enum Command {
//     Start(Task),
//     Ready(Result<(), OpenError>),
//     Close(ID),
// }

// #[non_exhaustive]
// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// pub enum Task {
//     Echo,
//     Pull,
//     Push
// }

// #[non_exhaustive]
// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// pub enum OpenError {
//     IDTaken,
//     UnkownTask,
// }

// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// pub struct Echo (pub Box<[u8]>);

// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// pub enum EchoError {
// }

