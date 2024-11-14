use clap::Parser;
use rsasim::cli::Args;

fn main() {
    let args = Args::parse();    
    if args.generate_keys {
        if let Some(p) = args.p {
            if let Some(q) = args.q {
                let keys = rsasim::gen_keys(p, q);
                if let Err(e) = rsasim::write_keys(keys) {
                    println!("{}", e)
                }
            }
        }
    } else if args.encrypt {
        if let Some(msg) = args.message {
            if let Err(e) = rsasim::encrypt(msg) {
                println!("{:?}", e)
            }
        }
    } else if args.decrypt {
        if let Some(msg) = args.message {
            if let Err(e) = rsasim::decrypt(msg) {
                println!("{:?}", e)
            }
        }
    }
} 
