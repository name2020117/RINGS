use clap::Parser;
use hello_tcp::cli::Args;
use std::{
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str,
};

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    println!("Starting server on {}:{:?}", args.address.trim(), args.port);
    let listener = TcpListener::bind(format!("{}:{}", args.address.trim(), args.port))?;
    println!(
        "Listening for connections on {}",
        listener.local_addr().unwrap()
    );

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut file = File::create("f.txt")?;
    println!("Received connection from {:?}", stream.peer_addr().unwrap());
    let mut buffer = [0; 128];
    let n = stream.read(&mut buffer)?;
    println!(
        "Received message = {:?} from {:?}",
        str::from_utf8(&buffer[0..n]).unwrap(),
        stream.peer_addr().unwrap()
    );
    write!(
        file,
        "{:?}\n from, {:?}\n",
        str::from_utf8(&buffer[0..n]),
        stream.peer_addr().unwrap()
    )?;
    let reply = b"Hi client";
    println!(
        "Sending message = {:?} to {:?}",
        str::from_utf8(reply).unwrap(),
        stream.peer_addr()?
    );
    stream.write_all(reply)?;
    Ok(())
}
