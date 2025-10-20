#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]
use std::{env, error::Error, fs, process};

use dotenv::dotenv;
use log::info;

// init log config
pub fn init() {
    dotenv().ok();
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
}


pub fn run(config: Config) -> anyhow::Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    for line in search(&config.query, &contents) {
        info!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> anyhow::Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    return results;
}