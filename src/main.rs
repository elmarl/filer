use clap::Parser;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Error;
use std::fs::File;
use std::io::Read;

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
/// cargo run --
fn main() {
    let args = Args::parse();
    if let (Some(ip), Some(file)) = (args.ip, args.file) {
        println!("Sending file {} to {}", file, ip);
        match connect(&ip, &file, &args.port) {
            Ok(_res) => println!("File sent."),
            Err(_err) => println!("Could not send file.")
        }
        } else {
        println!("Waiting for connection on port {}", args.port);
        let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port)).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("Connection established!");
            match handle_connection(stream) {
                Ok(_result) => println!("File received."),
                Err(_err) => println!("Error fetching file.")
            }
        }
    }
}

#[inline(always)]
fn _use_receive_mode(ip: &Option<String>, file: &Option<String>) -> bool {
    if let (Some(_ip), Some(_file)) = (ip, file) {
        return true;
    }
    false
}

fn connect(ip: &String, file: &String, port: &u16) -> Result<(), Error> {
    let mut socket = TcpStream::connect(format!("{}:{}", ip, port))?;
    
    let msg = std::fs::read(file).unwrap();
    socket.write(&msg[..])?;
    return Ok(());
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Error> {
    let mut data: Vec<u8> = Vec::new();
    stream.read_to_end(&mut data).unwrap();
    let mut file = File::create("./new_file.toml")?;
    file.write_all(&data)?;

    // let mut buffer = [0; 1024];

    // stream.read(&mut buffer).unwrap();

    // // write file
    // let mut file = File::create("./new_file.toml")?;
    // // Write a slice of bytes to the file
    // file.write_all(&buffer)?;
    

    // // replace invalid sequences with U+FFFD REPLACEMENT CHARACTER
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // let response = "HTTP/1.1 200 OK\r\n\r\n\r\n";
    let response = format!("{}\r\nAccept: {}\r\n\r\n", "HTTP/1.1 200 OK", "*");

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    Ok(())
}
