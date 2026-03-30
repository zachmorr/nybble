#![allow(dead_code, unused_imports)]

mod server;
mod devices;
mod run;
mod messages;

use server::server;
use run::run;
use devices::devices;
use anyhow::Result;
use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List connected Nybbles
    Devices,
    /// Start nyb server
    Server,
    /// Run command on Nybble
    Run,
}


fn main() -> Result<()> {
    env_logger::builder().format_timestamp(None).init();
    let args = Args::parse();

    match args.command {
        Commands::Devices => devices(),
        Commands::Run => run()?,
        Commands::Server => server()?,
    }
    Ok(())
}








    // let nybbles = find_nybble();
    // println!("{:?}", nybbles);
    // if nybbles.len() == 0 {
    //     return Err(anyhow!("No Nybbles found!"))
    // }

    // let serial = open_serial(&nybbles[0].clone())?;
    // let (writer, mut reader) = start_io(serial)?;

    // let mut count = 0u8;
    // loop {
    //     let payload = Box::new([count]);
    //     writer.send(payload).await?;
    //     if let Some(payload) = reader.recv().await {
    //         log::debug!("{:?}", payload);
    //     } else {
    //         return Err(anyhow!("Reader thread died!"));
    //     }
    //     count += 1;
    // }

// fn start_io(
//     nyb: &Nybble,
// ) -> anyhow::Result<(Sender<Payload>, Receiver<Payload>)> {
    
//     // Start reader
//     // let handle = nyb.open()?;
//     // let reader = NybbleIO::from(handle);
//     let (reader_tx, reader_rx) = mpsc::channel(10);
//     // spawn_blocking(|| {
//     //     libnyb::reader_thread(reader, reader_tx);
//     //     // if let Err(err) = reader_thread(handle.into(), reader_tx) {
//     //     //     log::error!(target: "main", "reader thread died with {err}");
//     //     // }
//     // });
    
//     // Start writer
//     let handle = nyb.open()?;
//     let writer = NybbleIO::from(handle);
//     let (writer_tx, writer_rx) = mpsc::channel(10);
//     spawn_blocking(|| {
//         libnyb::writer_thread(writer, writer_rx);
//         // if let Err(err) = writer_thread(handle.into(), writer_rx) {
//         //     log::error!(target: "main", "writer thread died with {err}");
//         // }
//     });

//     Ok((writer_tx, reader_rx))
// }

// fn reader_thread(
//     mut nyb: NybbleIO,
//     sender: mpsc::Sender<Packet>,
// ) -> anyhow::Result<()> {
//     let options = bincode::DefaultOptions::new()
//         .with_no_limit()
//         .with_fixint_encoding()
//         .with_little_endian()
//         .allow_trailing_bytes();

//     loop {
//         let packet = options.deserialize_from(&mut nyb)?; // deserialize_from consumes reader???
//         log::trace!(target: "reader", "read {:?}", packet);
//         sender.blocking_send(packet)?;
//     }
// }

// fn writer_thread(
//     mut nyb: NybbleIO,
//     mut receiver: mpsc::Receiver<Packet>,
// ) -> anyhow::Result<()> {
//     let options = bincode::DefaultOptions::new()
//         .with_no_limit()
//         .with_fixint_encoding()
//         .with_little_endian()
//         .reject_trailing_bytes();
    
//     loop {
//         let packet = receiver.blocking_recv()
//             .ok_or_else(|| anyhow!("channel was closed!"))?;

//         log::trace!(target: "writer", "writing {:?}", packet);
//         options.serialize_into(&mut nyb, &packet)?;
//     }
// }



// #[derive(Parser)]
// #[command(version, about, long_about = None)]
// #[command(propagate_version = true)]
// struct CLI {
//     #[command(subcommand)]
//     command: Commands,
// }

// #[derive(Subcommand)]
// enum Commands {
//     List,
//     Echo(EchoArgs),
// }

// #[derive(Args)]
// struct EchoArgs {
//     string: String,
// }

// let cli = CLI::parse();
// match cli.command {
//     Commands::List => print_nybble_list()?,
//     Commands::Echo(args) => echo(args)?,
// };

// fn print_nybble_list() -> Result<()> {
//     let nybble_list = Nybble::list()?;

//     for nyb in nybble_list {
//         println!("{:?}", nyb.serial()?);
//     }
//     Ok(())
// }

// fn echo(args: EchoArgs) -> Result<()> {
//     let nybble = Nybble::find(None)?
//         .ok_or(anyhow!("No Nybbles found!"))?;
//     let stream = Stream::open(nybble, ECHO_TASK)?;


//     // stream.write(Data::Echo(EchoData::Data(args.string.into_boxed_str().into())))?;
//     stream.write(Echo(args.string.into_boxed_str().into()))?;
//     let response: Echo = stream.read()?;
//     // match response {
//     //     Data::Echo(EchoData::Data(data)) => {
//     //         println!("{}", String::from_utf8(data.to_vec())?)
//     //     },
//     //     _ => return Err(anyhow!("Invalid echo response!")),
//     // };
//     Ok(())
// }