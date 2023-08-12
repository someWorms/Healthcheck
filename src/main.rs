use std::{env, thread, time, process};
use url::Url;
use reqwest;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = Configuration::create(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    healthcheck(&cfg);
}

fn healthcheck(cfg: &Configuration) {
    let client = reqwest::blocking::Client::new();
    loop {
        match client.get(&cfg.host).send() {
            Ok(status) => {
                if status.status().is_success() {
                    println!("Checking '{}'. Result: OK({})", cfg.host, status.status().as_u16());
                } else {
                    println!("Checking '{}'. Result: ERR({})", cfg.host, status.status().as_u16());
                }
            }
            Err(_ex) => {
                println!("The '{}' host error", cfg.host);
            }
        }
        thread::sleep(time::Duration::from_secs(cfg.interval));
    }
}

struct Configuration {
    interval: u64,
    host: String,
}

impl Configuration {
    fn create(args: &Vec<String>) -> Result<Configuration, &'static str> {

        // First arg is the binary + 2 user defined args.
        if args.len() != 3 {
            return Err("You must pass 2 arguments: interval and hostname");
        }

        // Check if interval is int
        let interval = match args[1].parse::<u64>() {
            Ok(interval) => interval,
            Err(_ex) => return Err("Interval is not a number."),
        };

        // Validate url (we could do so with regex also)
        let host = match Url::parse(&args[2]) {
            Ok(host) => host.to_string(),
            Err(_ex) => return Err("URL parsing error. Hostname is not valid"),
        };

        Ok(Configuration {
            interval,
            host,
        })
    }
}