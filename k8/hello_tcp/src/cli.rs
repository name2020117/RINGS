use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser, default_value="0.0.0.0")]
    pub address: String,

    #[clap(short, long, value_parser, default_value="8080")]
    pub port: u32,
}
