use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    println!("Starting client");
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Connected to {:?}", stream.peer_addr().unwrap());
    let message = b"Hi server";
    println!(
        "Sending message = {:?} to {:?}",
        str::from_utf8(message).unwrap(),
        stream.peer_addr().unwrap()
    );
    stream.write_all(message).unwrap();

    let mut buffer = [0; 128];
    let n = stream.read(&mut buffer)?;
    println!(
        "Received message = {:?} from {:?}",
        str::from_utf8(&buffer[0..n]).unwrap(),
        stream.peer_addr().unwrap()
    );
    Ok(())
}
