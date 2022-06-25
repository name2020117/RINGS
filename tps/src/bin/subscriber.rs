// use chrono::{offset::Utc, DateTime};
use clap::Parser;
use kafka::consumer::Consumer;
use std::{str, time::SystemTime};
use teleps::cli::Args;
use teleps::message::Message;

fn main() {
    let args = Args::parse();
    println!("Running with args = {:?}", args);
    // let mut wtr = csv::Writer::from_writer(io::stdout());
    let mut wtr = csv::Writer::from_path("logs/send_receive_times.csv").unwrap();
    // WARN: not handling write error potential
    wtr.write_record(&["sender", "send_time", "receive_time"])
        .unwrap();
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
                        // WARN: not handling parsing message failure potential
                        let mut message: Message = Message::new_from_string(m).unwrap();
                        println!("{}", message);
                        println!("{:?}", message);
                        message.add_recieve_time(SystemTime::now().into());
                        println!("message time = {}\n", &message.duration().unwrap());
                        wtr.write_record(&[
                            message.sender_ip,
                            message.send_time.unwrap().to_string(),
                            message.receive_time.unwrap().to_string(),
                        ])
                        .unwrap();
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
