use clap::Parser;
use indicatif::ProgressBar;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use filer::utils::ProgressCommand;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, value_parser)]
    ip: Option<String>,

    #[clap(short, long, value_parser)]
    file: Option<String>,

    #[clap(short, long, value_parser)]
    path: Option<String>,

    #[clap(long, value_parser, default_value_t = 7979)]
    port: u16,
}

///
/// File transfer tool.
/// To send a file, provide an IP address and a path to a file.
/// To receive a file, run the tool with no arguments, or a port argument, default 7878.
///
/// cargo run -- --ip 127.0.0.1 -f ./Cargo.toml
/// cargo run
fn main() {
    let args = Args::parse();
    let bar = Arc::new(ProgressBar::new(1));

    let (tx, rx): (Sender<ProgressCommand>, Receiver<ProgressCommand>) = mpsc::channel();
    if let (Some(ip), Some(file)) = (args.ip, args.file) {
        // client
        match filer::client::send_file(ip, file, args.port, tx) {
            Ok(()) => (),
            Err(err) => println!("Failed sending file. {err}"),
        };
    } else if let Some(path) = args.path {
        {
            // server
            match filer::server::listen(path, args.port, tx) {
                Ok(()) => (),
                Err(err) => println!("Error receiving file. {err}"),
            };
        }
    } else {
        println!("See help for instructions.");
        std::process::exit(1);
    }
    for received in rx {
        match received {
            ProgressCommand::SetLength(l) => bar.set_length(l),
            ProgressCommand::Inc() => bar.inc(1),
            ProgressCommand::Done() => bar.finish(),
        }
    }
    println!("Done!");
}

// #[inline(always)]
// fn _use_receive_mode(ip: &Option<String>, file: &Option<String>) -> bool {
//     if let (Some(_ip), Some(_file)) = (ip, file) {
//         return true;
//     }
//     false
// }

// fn send_file(
//     ip: &String,
//     file: &String,
//     port: &u16,
//     tx: Sender<ProgressCommand>,
// ) -> Result<(), Error> {
//     let mut socket = TcpStream::connect(format!("{}:{}", ip, port))?;
//     let mut bytes_read;
//     let mut file = File::open(file)?;
//     let size = file.metadata()?.len();
//     const BUFFER_LEN: usize = 1024;
//     let units = size as f64 / BUFFER_LEN as f64;
//     let units = units.ceil() as u64;
//     tx.send(ProgressCommand::SetLength(units)).unwrap();
//     socket.write(&size.to_be_bytes())?;
//     let mut buffer = [0u8; BUFFER_LEN];
//     loop {
//         tx.send(ProgressCommand::Inc()).unwrap();
//         bytes_read = file.read(&mut buffer)?;
//         socket.write(&buffer)?;
//         if bytes_read != BUFFER_LEN {
//             tx.send(ProgressCommand::Done()).unwrap();
//             break;
//         }
//     }
//     return Ok(());
// }

// fn handle_connection(
//     mut stream: TcpStream,
//     path: String,
//     tx: Sender<ProgressCommand>,
// ) -> Result<(), Error> {
//     let mut file = File::create(path)?;
//     const BUFFER_LEN: usize = 1024;
//     const SIZE_LEN: usize = 8;
//     let mut buff = [0u8; BUFFER_LEN];
//     let mut size_buff = [0u8; SIZE_LEN];
//     let mut bytes_copied: usize;
//     let mut size_reader = BufReader::with_capacity(8, &stream);
//     let mut reader = BufReader::with_capacity(BUFFER_LEN, &stream);
//     size_reader.read(&mut size_buff)?;
//     let units = (u64::from_be_bytes(size_buff) - SIZE_LEN as u64) / BUFFER_LEN as u64;
//     tx.send(ProgressCommand::SetLength(units)).unwrap();

//     loop {
//         tx.send(ProgressCommand::Inc()).unwrap();
//         bytes_copied = reader.read(&mut buff)?;
//         file.write(&buff)?;
//         if bytes_copied < BUFFER_LEN {
//             tx.send(ProgressCommand::Done()).unwrap();
//             break;
//         }
//     }
//     let response = "Transfer successful!";

//     stream.write(response.as_bytes())?;
//     stream.flush()?;
//     Ok(())
// }
