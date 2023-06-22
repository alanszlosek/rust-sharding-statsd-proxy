use regex::Regex;
use std::fs::File;
use std::{
    io::Read,
    iter::FromIterator,
    net::{IpAddr, Ipv4Addr},
};

const DEFAULT_BIND_INTERFACE: &str = "0.0.0.0";
const DEFAULT_BIND_PORT: u32 = 8125;

pub struct Settings {
    pub bind_interface: String,
    pub bind_port: u32,
    pub destinations: Vec<String>,
    pub threads: u8,
}

impl Settings {
    pub fn load<'a>(filename: &str) -> Self {
        let mut contents = String::new();
        let f = File::open(filename);
        if f.is_ok() {
            if !f.unwrap().read_to_string(&mut contents).is_ok() {
                println!("Failed to read config.ini contents");
            }
        } else {
            println!("Failed to read config.ini")
        }

        let re = Regex::new(r"(bind_interface|bind_port|destinations|threads)\s*=\s*([^\n]+)")
            .expect("Failed to compile regex");

        let mut bind_interface = String::from(DEFAULT_BIND_INTERFACE);
        let mut bind_port = DEFAULT_BIND_PORT;
        let mut destinations = Vec::<String>::new();
        let mut threads: u8 = 1;

        for cap in re.captures_iter(contents.as_str()) {
            match &cap[1] {
                "bind_interface" => {
                    bind_interface = String::from(&cap[2]);
                }
                "bind_port" => {
                    bind_port = cap[2].parse().unwrap();
                }
                "destinations" => {
                    // TODO: ensure valid IP address and port,
                    // by parsing to IpAddr
                    destinations = cap[2].split(' ').map(|i| String::from(i)).collect()
                }
                "threads" => {
                    threads = cap[2].parse().unwrap();
                }
                _ => println!("Other: {:?}", cap),
            }
        }

        Settings {
            bind_interface: bind_interface,
            bind_port: bind_port,
            destinations: destinations,
            threads: threads,
        }
    }
}
