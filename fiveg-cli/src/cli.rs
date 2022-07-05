use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser, default_value = "localhost")]
    pub db_ip: String,

    #[clap(short, long, value_parser, default_value = "27017")]
    pub port: u32,

    #[clap(short, long, value_parser, default_value = "1")]
    pub n_subscribers: u32,

    // need error check
    #[clap(short, long, value_parser, default_value = "208930000000003")]
    pub imsi: u64,
}
