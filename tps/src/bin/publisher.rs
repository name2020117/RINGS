use kafka::producer::{Producer, Record, RequiredAcks};
use local_ip_address::local_ip;
use opentelemetry::{
    global, sdk,
    trace::{TraceError, Tracer},
};
use std::{
    // fmt::format,
    thread::sleep,
    time::{Duration, SystemTime},
};
use teleps::message::Message;
use teleps::cli::Args;
use clap::Parser;

fn init_tracer() -> Result<sdk::trace::Tracer, TraceError> {
    opentelemetry_jaeger::new_pipeline().install_simple()
}

#[allow(unreachable_code)]
fn main() -> Result<(), opentelemetry::trace::TraceError> {
    let args = Args::parse();
    println!("Running with args = {:?}", args);
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = init_tracer().expect("failed to initialize tracer");
    // FIXME: should likely pass a function to the tracer instead of this
    tracer.in_span("doing_work_oltp", |_| {
        // just to test the CLI
        let mut producer =
            // Producer::from_hosts(vec![RECEIVER_IP.to_string() + ":" + RECEIVER_PORT])
            Producer::from_hosts(vec![format!("{}:{}", args.broker_address, args.port)])
                .with_ack_timeout(Duration::from_secs(1))
                .with_required_acks(RequiredAcks::One)
                .create()
                .unwrap();
        println!("connected to server");
        for _ in 1..10 {
            let message = Message::new(
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
            sleep(Duration::from_secs(5));
        }
    });
    global::shutdown_tracer_provider();
    Ok(())
}
