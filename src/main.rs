use revbin::Config;
use std::env;
use std::process;

fn main() {
    let args = env::args().collect();
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(err) = config.run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
