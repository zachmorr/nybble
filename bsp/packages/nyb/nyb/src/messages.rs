use std::io::Read;
use std::{io::Write, path::PathBuf};
use std::mem;
use bincode::Options;
use libnyb::serdes;
use anyhow::{anyhow, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};


pub type MsgLen = u32;


pub async fn server_send<D: Serialize>(mut stream: tokio::net::TcpStream, data: D) -> Result<()> {
    let serialized_data = serdes().serialize(&data)?;
    let len = serialized_data.len() as MsgLen;
    if len > MsgLen::MAX {
        return Err(anyhow!("Data size ({len}) is too large"));
    }
    let len_bytes = len.to_le_bytes().to_vec();

    stream.write(&len_bytes).await?;
    stream.write(&serialized_data).await?;
    Ok(())
}

pub async fn server_recv<D: DeserializeOwned>(mut stream: tokio::net::TcpStream) -> Result<D> {
    let mut len_buf = [0; mem::size_of::<MsgLen>()]; 
    stream.read_exact(&mut len_buf).await?;
    
    let len = MsgLen::from_le_bytes(len_buf);
    let mut buf = vec![0; len as usize];
    stream.read_exact(&mut buf).await?;

    Ok(serdes().deserialize(&buf)?)
}


pub fn client_send<D: Serialize>(mut stream: std::net::TcpStream, data: D) -> Result<()> {
    let serialized_data = serdes().serialize(&data)?;
    let len = serialized_data.len() as MsgLen;
    if len > MsgLen::MAX {
        return Err(anyhow!("Data size ({len}) is too large"));
    }
    let len_bytes = len.to_le_bytes().to_vec();

    stream.write(&len_bytes)?;
    stream.write(&serialized_data)?;
    Ok(())
}

pub fn client_recv<D: DeserializeOwned>(mut stream: std::net::TcpStream) -> Result<D> {
    let mut len_buf = [0; mem::size_of::<MsgLen>()]; 
    stream.read_exact(&mut len_buf)?;
    
    let len = MsgLen::from_le_bytes(len_buf);
    let mut buf = vec![0; len as usize];
    stream.read_exact(&mut buf)?;

    Ok(serdes().deserialize(&buf)?)
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Ack ();

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Start (pub PathBuf, pub PathBuf);

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Std {
    Out(String),
    Err(String),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct StdIn (pub String);