use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::io::{BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::thread;
use utils::ProgressCommand;

pub const BUFFER_LEN: usize = 1024;

// use crate::BUFFER_LEN;
pub mod utils {
    pub enum ProgressCommand {
        SetLength(u64),
        Inc(),
        Done(),
    }
}
pub mod server{
    use super::*;

    pub fn listen(path: String, port: u16, tx: Sender<ProgressCommand>) -> Result<(), Error> {
        let handle = thread::spawn(move || {
            let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
            match listener.accept() {
                Ok((stream, _addr)) => match handle_connection(stream, path, tx) {
                    Ok(_result) => (),
                    Err(_err) => println!("Error fetching file."),
                },
                Err(e) => println!("couldn't get client: {e:?}"),
            }
        });
        handle.join().expect("Error joining thread.");
        Ok(())
    }
    fn handle_connection(
        mut stream: TcpStream,
        path: String,
        tx: Sender<ProgressCommand>,
    ) -> Result<(), Error> {
        let mut file = File::create(path)?;
        const SIZE_LEN: usize = 8;
        let mut buff = [0u8; BUFFER_LEN];
        let mut size_buff = [0u8; SIZE_LEN];
        let mut bytes_copied: usize;
        let mut size_reader = BufReader::with_capacity(8, &stream);
        let mut reader = BufReader::with_capacity(BUFFER_LEN, &stream);
        size_reader.read(&mut size_buff)?;
        let units = (u64::from_be_bytes(size_buff) - SIZE_LEN as u64) / BUFFER_LEN as u64;
        tx.send(ProgressCommand::SetLength(units)).unwrap();
    
        loop {
            tx.send(ProgressCommand::Inc()).unwrap();
            bytes_copied = reader.read(&mut buff)?;
            file.write(&buff)?;
            if bytes_copied < BUFFER_LEN {
                tx.send(ProgressCommand::Done()).unwrap();
                break;
            }
        }
        let response = "Transfer successful!";
    
        stream.write(response.as_bytes())?;
        stream.flush()?;
        Ok(())
    }
}

pub mod client {
    use super::*;

    pub fn send_file(
        ip: String,
        file: String,
        port: u16,
        tx: Sender<ProgressCommand>,
    ) -> Result<(), Error> {
        let handle = thread::spawn(move || {
            let mut socket = TcpStream::connect(format!("{}:{}", ip, port)).unwrap();
            let mut bytes_read;
            let mut file = File::open(file).unwrap();
            let size = file.metadata().unwrap().len();
            let units = size as f64 / BUFFER_LEN as f64;
            let units = units.ceil() as u64;
            tx.send(ProgressCommand::SetLength(units)).unwrap();
            socket.write(&size.to_be_bytes()).unwrap();
            let mut buffer = [0u8; BUFFER_LEN];
            loop {
                tx.send(ProgressCommand::Inc()).unwrap();
                bytes_read = file.read(&mut buffer).unwrap();
                socket.write(&buffer).unwrap();
                if bytes_read != BUFFER_LEN {
                    tx.send(ProgressCommand::Done()).unwrap();
                    break;
                }
            }
        });
        handle.join().unwrap();
        Ok(())
    }
}
