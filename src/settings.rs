use std::{net::{IpAddr, Ipv4Addr}, iter::FromIterator, io::Read};
use std::fs::File;
use regex::Regex;

const DEFAULT_PORT: u32 = 5001;


pub struct Settings {
    pub bind_interface: String,
    pub bind_port: u32,
    pub destinations: Vec<String>
}


impl Settings {
    pub fn load<'a>(filename: &str) -> Self {
        let mut contents = String::new();
        let f = File::open(filename);
        if f.is_ok() {
            if ! f.unwrap().read_to_string(&mut contents).is_ok() {
                println!("Failed to read config.ini contents");
            }
        } else {
            println!("Failed to read config.ini")
        }

        let re = Regex::new(r"(bind_interface|bind_port|destinations)\s*=\s*([^\n]+)").expect("Failed to compile regex");

        let mut bind_interface = String::new();
        let mut bind_port = DEFAULT_PORT;
        let mut destinations = Vec::<String>::new();

        for cap in re.captures_iter(contents.as_str()) {
            match &cap[1] {
                "bind_interface" => {
                    bind_interface = String::from(&cap[2]);
                },
                "bind_port" => {
                    bind_port = cap[2].parse().unwrap();
                }
                "destinations" => {
                    // TODO: ensure valid IP address and port
                    destinations = cap[2].split(' ').map(|i|  String::from(i)).collect()
                },
                _ => println!("Other: {:?}", cap)
            }
        }
        

        Settings {
            bind_interface: bind_interface,
            bind_port: bind_port,
            destinations: destinations
        }

    }
}