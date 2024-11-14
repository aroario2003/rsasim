use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "1.0.0", about = "A rsa simulator", long_about = None)]
pub struct Args {
    #[arg(short)]
    pub p: Option<u128>,

    #[arg(short)]
    pub q: Option<u128>,

    #[arg(short, long)]
    pub message: Option<u128>,

    #[arg(short, long, default_value_t = false)]
    pub encrypt: bool,

    #[arg(short, long, default_value_t = false)]
    pub decrypt: bool,

    #[arg(short, long, default_value_t = false)]
    pub generate_keys: bool
}
