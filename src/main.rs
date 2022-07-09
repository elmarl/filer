use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::io::{BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use indicatif::ProgressBar;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, value_parser)]
    ip: Option<String>,

    #[clap(short, long, value_parser)]
    file: Option<String>,

    #[clap(short, long, value_parser, default_value_t = 7979)]
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
    let bar = Arc::new( ProgressBar::new(1));
    
    let handle;
    let (tx, rx) = mpsc::channel();
    if let (Some(ip), Some(file)) = (args.ip, args.file) {
        // client
        let shared_bar = bar.clone();
        handle = thread::spawn(move || {
            match send_file(&ip, &file, &args.port, tx.clone(), shared_bar) {
                Ok(_res) => (),
                Err(_err) => println!("Could not send file."),
            }
        });
    } else {
        // server
        let shared_bar = bar.clone();
        handle = thread::spawn(move || {
            let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port)).unwrap();
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                match handle_connection(stream, tx.clone(), shared_bar.clone()) {
                    Ok(_result) => (),
                    Err(_err) => println!("Error fetching file."),
                }
            }
        });
    }
    for _received in rx {
        bar.inc(1);
    }
    bar.finish();
    handle.join().unwrap();
    println!("Done!");
}

#[inline(always)]
fn _use_receive_mode(ip: &Option<String>, file: &Option<String>) -> bool {
    if let (Some(_ip), Some(_file)) = (ip, file) {
        return true;
    }
    false
}

fn send_file(
    ip: &String,
    file: &String,
    port: &u16,
    tx: std::sync::mpsc::Sender<&str>,
    bar: Arc<ProgressBar>
) -> Result<(), Error> {
    let mut socket = TcpStream::connect(format!("{}:{}", ip, port))?;

    let mut bytes_read;
    let mut file = File::open(file)?;
    let size = file.metadata()?.len();
    const BUFFER_LEN: usize = 1024;
    let units = size as f64 / BUFFER_LEN as f64;
    let units = units.ceil() as u64;
    bar.set_length(units);
    let mut buffer = [0u8; BUFFER_LEN];
    loop {
        tx.send("").unwrap();
        bytes_read = file.read(&mut buffer)?;
        socket.write(&buffer)?;
        if bytes_read != BUFFER_LEN {
            break;
        }
    }
    return Ok(());
}

fn handle_connection(
    mut stream: TcpStream,
    tx: std::sync::mpsc::Sender<&str>,
    bar: Arc<ProgressBar>
) -> Result<(), Error> {
    let mut file = File::create("./new_file.gif")?;
    const BUFFER_LEN: usize = 1024;
    let mut buff = [0u8; BUFFER_LEN];
    let mut bytes_copied: usize;
    let mut reader = BufReader::with_capacity(BUFFER_LEN, &stream);
    loop {
        tx.send("").unwrap();
        bar.set_length(bar.length() + 1);
        bytes_copied = reader.read(&mut buff)?;
        file.write(&buff)?;
        if bytes_copied < BUFFER_LEN {
            break;
        }
    }

    let response = "Transfer successful!";

    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
