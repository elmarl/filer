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
