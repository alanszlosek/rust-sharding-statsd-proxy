use serde::{Serialize, Deserialize};
use serde_yaml;
use std::{net::{IpAddr, Ipv4Addr}, iter::FromIterator};
use std::fs;
use std::fs::File;

const DEFAULT_PORT: u32 = 5001;

fn default_interface() -> String {
    String::from("0.0.0.0")
}
fn default_port() -> u32 {
    5001
}
fn default_destinations() -> Vec<String> {
    Vec::<String>::new()
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default = "default_interface")]
    pub bind_interface: String,
    #[serde(default = "default_port")]
    pub bind_port: u32,
    // bind this lifetime to struct lifetime ... i think that's what's happening here
    #[serde(default = "default_destinations")]
    pub destinations: Vec<String>
}


impl Settings {
    pub fn load<'a>(filename: &str) -> Self {
        match fs::read_to_string(filename) {
            Ok(yaml) => match serde_yaml::from_str(yaml.as_str()) {
                Ok(val) => val,
                Err(_) => serde_yaml::from_str("").unwrap()
            },
            Err(_) => serde_yaml::from_str("").unwrap()
        }
        /*
        Settings {
            bind_interface: match i.general_section().get("bind_interface") {
                // Parse/convert str to IpAddr
                // Fail if IP isn't valid. Don't fall back to a default because in production, that unexpected behavior
                // would get lost in log files and could be a potential security risk.
                Some(val) => val.parse().unwrap(),
                None => std::net::IpAddr::V4(Ipv4Addr::UNSPECIFIED)
            },
            bind_port: match i.general_section().get("bind_port") {
                // TODO: error handling for invalid port
                Some(val) => val.parse().unwrap(),
                None => DEFAULT_PORT
            },
            destinations: Vec::new().extend(
                match i.general_section().get("destinations") {
                    Some(val) => val.split_whitespace().collect(),
                    None => Vec::new()
                }
            )
        }
        */
    }
}