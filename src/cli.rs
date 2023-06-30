use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Full path to config INI file
    #[arg(short, long)]
    pub config_file: Option<String>,

    /// Network interface the proxy will listen on
    #[arg(short, long)]
    pub bind_interface: Option<String>,

    /// Port the proxy will listen on
    #[arg(short = 'p', long)]
    pub bind_port: Option<u16>,

    /// Number of threads to spawn to handle incoming StatsD messages
    #[arg(short, long)]
    pub threads: Option<u8>,
}
