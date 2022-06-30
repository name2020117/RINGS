use clap::Parser;
use kafka::consumer::Consumer;
use local_ip_address::local_ip;
use std::{fs, str, time::SystemTime};
use teleps::cli::Args;
use teleps::message::Message;

fn main() {
    let args = Args::parse();
    println!("Running with args = {:?}", args);
    match fs::create_dir("./logs") {
        Ok(()) => {}
        Err(e) => {
            println!("{}", e);
        }
    };
    let mut wtr = csv::Writer::from_path("logs/send_receive_times.csv").unwrap();
    let mut consumer = Consumer::from_hosts(vec![format!("{}:{}", args.broker_address, args.port)])
        .with_topic(args.topic)
        .create()
        .unwrap();
    println!("connected to server");
    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                match str::from_utf8(m.value) {
                    Ok(m) => {
                        let mut message: Message = Message::new_from_string(m).unwrap();
                        message.add_recieve_time(SystemTime::now().into());
                        message.add_reciever_ip(local_ip().unwrap().to_string());
                        println!("{}", message);
                        println!("message time = {}\n", &message.duration().unwrap());
                        // this should not be an error, since it should always write
                        wtr.serialize(message).unwrap();
                    }
                    Err(e) => println!("received invalid utf8 sequence: {}", e),
                }
            }
            consumer
                .consume_messageset(ms)
                .expect("I don't know why this would fail");
        }
        consumer.commit_consumed().unwrap();
        wtr.flush().unwrap();
    }
}
