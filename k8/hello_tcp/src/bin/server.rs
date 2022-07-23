use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str,
};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
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
    println!("Received connection from {:?}", stream.peer_addr().unwrap());
    let mut buffer = [0; 128];
    let n = stream.read(&mut buffer)?;
    println!(
        "Received message = {:?} from {:?}",
        str::from_utf8(&buffer[0..n]).unwrap(),
        stream.peer_addr().unwrap()
    );
    let reply = b"Hi client";
    println!(
        "Sending message = {:?} to {:?}",
        str::from_utf8(reply).unwrap(),
        stream.peer_addr().unwrap()
    );
    stream.write_all(reply)?;
    Ok(())
}
