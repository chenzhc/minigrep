use log::info;
use minigrep::{init, run, Config};
use std::{env,  process};

fn main() {
    init();

    // let args: Vec<String> = env::args().collect();
    // info!("{:?}", args);

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        info!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    info!("Searching for {}", config.query);
    info!("In file {}", config.filename);

    if let Err(e) = run(config) {
        info!("Application error: {}", e);
        process::exit(1);
    }
}



