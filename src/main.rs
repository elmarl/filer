use clap::Parser;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Error;
use std::fs::File;
use std::io::Read;

///
/// File transfer tool.
/// To send a file, provide an IP address and a path to a file.
/// To receive a file, run the tool with no arguments, or a port argument, default 7878.
///

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

fn main() {
    let args = Args::parse();
    // println!("{}", &args.ip.unwrap());
    // println!("{}", &args.file.unwrap());
    // println!("{}", args.ip.is_empty());
    // println!("{}", args.file.unwrap().is_empty());
    // println!("{}", use_receive_mode(&args.ip, &args.file));

    // if args.ip.as_ref().and(args.file.as_ref()) != None {
    //     println!("Both check!");
    // }
    if let (Some(ip), Some(file)) = (args.ip, args.file) {
        println!("Sending file {} to {}", file, ip);
        // let mut socket = TcpStream::connect(format!("127.0.0.1:{}", args.port))?;
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
            handle_connection(stream);
        }
    }
    // if use_receive_mode(&args.ip, &args.file) {
    //     println!("Sending file {} to {}", &args.file, &args.ip)
    // } else {
    //     println!("Waiting for connection on port {}", args.port)
    // }
    // println!("Hello {}! {}x", args.name, args.count);
    // return;
    // let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();

    //     println!("Connection established!");
    //     handle_connection(stream);
    // }
}

#[inline(always)]
fn _use_receive_mode(ip: &Option<String>, file: &Option<String>) -> bool {
    if let (Some(_ip), Some(_file)) = (ip, file) {
        return true;
    }
    false
}

fn connect(ip: &String, _file: &String, port: &u16) -> Result<bool, Error> {
    let mut socket = TcpStream::connect(format!("{}:{}", ip, port))?;
    
    let msg = std::fs::read("./Cargo.toml").unwrap();
    socket.write(&msg[..])?;
    return Ok(true);
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    // replace invalid sequences with U+FFFD REPLACEMENT CHARACTER
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // let response = "HTTP/1.1 200 OK\r\n\r\n\r\n";
    let response = format!("{}\r\nAccept: {}\r\n\r\n", "HTTP/1.1 200 OK", "*");

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
