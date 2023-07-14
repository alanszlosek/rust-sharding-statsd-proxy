use regex::Regex;
use std::fs::File;
use std::{io::Error, io::Read};


const DEFAULT_BIND_INTERFACE: &str = "0.0.0.0";
const DEFAULT_BIND_PORT: u16 = 8125;
const DEFAULT_THREADS: u8 = 1;

pub struct Settings {
    pub bind_interface: String,
    pub bind_port: u16,
    pub destinations: Vec<String>,
    pub threads: u8,
}

// Clippy suggests we implement Default for Settings ...
// TODO: look into what and what

impl Settings {
    pub fn new() -> Self {
        Settings {
            bind_interface: String::from(DEFAULT_BIND_INTERFACE),
            bind_port: DEFAULT_BIND_PORT,
            destinations: Vec::<String>::new(),
            threads: DEFAULT_THREADS,
        }
    }
    pub fn load(filename: &str) -> Result<Self, Error> {
        let mut contents = String::new();
        File::open(filename)?.read_to_string(&mut contents)?;

        let re = Regex::new(r"(bind_interface|bind_port|destinations|threads)\s*=\s*([^\n]+)")
            .expect("Failed to compile regex");

        let mut settings = Settings::new();

        for cap in re.captures_iter(contents.as_str()) {
            match &cap[1] {
                "bind_interface" => {
                    settings.bind_interface = String::from(&cap[2]);
                }
                "bind_port" => {
                    settings.bind_port = cap[2].parse().unwrap();
                }
                "destinations" => {
                    // TODO: ensure valid IP address and port,
                    // by parsing to IpAddr
                    settings.destinations = cap[2].split(' ').map(|i| String::from(i)).collect()
                }
                "threads" => {
                    settings.threads = cap[2].parse().unwrap();
                }
                _ => println!("Other: {:?}", cap),
            }
        }

        Ok(settings)
    }

    pub fn merge(&mut self, args: crate::cli::Args) -> &Self {
        if let Some(bind_interface) = args.bind_interface {
            println!("Got bind_interface {}", bind_interface);
            self.bind_interface = bind_interface;
        }
        if let Some(bind_port) = args.bind_port {
            println!("Got bind_port {}", bind_port);
            self.bind_port = bind_port;
        }
        if let Some(threads) = args.threads {
            println!("Got threads {}", threads);
            self.threads = threads;
        }
        self
    }

    // TODO: another function for merging from environment variables
}
