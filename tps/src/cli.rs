use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser, default_value="127.0.0.1")]
    pub broker_address: String,

    #[clap(short, long, value_parser, default_value="9092")]
    pub port: u32,

    #[clap(short, long, value_parser, default_value="test")]
    pub topic: String,
}
