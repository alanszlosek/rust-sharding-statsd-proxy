use clap::Parser;
use std::env;

// Include all public functions and modules from library
use rust_statsd_proxy::*;

fn main() {
    let args = cli::Args::parse();


    /*
    Override order

    * Defaults ... overriden by
    * Config file ... overridden by
    * Environment variables ... overridden by
    * CLI flags. When specified, CLI flags take precedence.
    */
    let mut settings = match settings::Settings::load("config.ini") {
        Ok(s) => {
            println!("Loaded settings from config.ini");
            s
        }
        Err(_e) => settings::Settings::new(),
    };

    // Now merge in environment variables
    if let Ok(bind_interface) = env::var("BIND_INTERFACE") {
        settings.bind_interface = bind_interface;
    }
    if let Ok(bind_port) = env::var("BIND_PORT") {
        settings.bind_port = bind_port.parse::<u16>().unwrap();
    }
    if let Ok(threads) = env::var("THREADS") {
        settings.threads = threads.parse::<u8>().unwrap();
    }

    // Now merge in command line arguments
    settings.merge(args);

    run(settings);
}
