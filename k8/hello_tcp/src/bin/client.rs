use clap::Parser;
use core::time;
use hello_tcp::cli::Args;
use std::{
    fs::File,
    io::{Read, Write},
    net::TcpStream,
    str,
    thread::sleep,
};

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    println!("Starting client, attempting to connect to {}:{:?}", args.address.trim(), args.port);
    let mut file = File::create("f.txt")?;
    let mut stream = TcpStream::connect(format!("{}:{}", args.address.trim(), args.port))?;
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
    // file.write_all(&buffer[0..n])?;
    write!(
        file,
        "{:?}\n from, {:?}\n",
        str::from_utf8(&buffer[0..n]).unwrap(),
        stream.peer_addr()?
    )?;
    sleep(time::Duration::from_secs(900));
    Ok(())
}
