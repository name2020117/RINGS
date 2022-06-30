use clap::Parser;
use kafka::producer::{Producer, Record, RequiredAcks};
use local_ip_address::local_ip;
use opentelemetry::{
    global, sdk,
    trace::{TraceError, Tracer},
};
use std::{
    thread::sleep,
    time::{Duration, SystemTime},
};
use teleps::cli::Args;
use teleps::message::Message;
use teleps::interfaces::get_interface_ip;

fn init_tracer() -> Result<sdk::trace::Tracer, TraceError> {
    opentelemetry_jaeger::new_pipeline().install_simple()
}

#[allow(unreachable_code)]
fn main() -> Result<(), opentelemetry::trace::TraceError> {
    let args = Args::parse();
    println!("Running with args = {:?}", args);
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = init_tracer().expect("failed to initialize tracer");
    tracer.in_span("doing_work_oltp", |_| {
        let mut producer =
            Producer::from_hosts(vec![format!("{}:{}", args.broker_address, args.port)])
                .with_ack_timeout(Duration::from_secs(1))
                .with_required_acks(RequiredAcks::One)
                .create()
                .unwrap();
        println!("connected to server");
        for i in 0..(args.n_messages) {
            let message = Message::new(
                i,
                args.interface.clone(),
                get_interface_ip(&args.interface).to_string(),
                local_ip().unwrap().to_string(),
                args.broker_address.clone(),
                args.topic.clone(),
                Some(SystemTime::now().into()),
            );
            println!("sending: {:?}", message);
            producer
                .send(&Record::from_value(
                    &args.topic,
                    message.to_string().unwrap().as_bytes(),
                ))
                .unwrap();
            println!("sent the message");
            sleep(Duration::from_secs(args.sleep.into()));
        }
    });
    global::shutdown_tracer_provider();
    Ok(())
}
