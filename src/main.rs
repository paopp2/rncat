use clap::{Parser, Subcommand};
use defaults::*;
use std::net::IpAddr;

mod defaults {
    pub const DEFAULT_IP_ADDR: &str = "127.0.0.1";
    pub const DEFAULT_PORT: u16 = 4444;
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    mode: OperationMode,

    /// If client, IP address to connect to. If server, IP address to bind to.
    #[clap(short, long, default_value = DEFAULT_IP_ADDR, global = true)]
    address: IpAddr,

    /// If client, port to connect to. If server, port to bind to
    #[clap(short, long, default_value_t = DEFAULT_PORT, global = true)]
    port: u16,
}

#[derive(Subcommand, Debug)]
enum OperationMode {
    Server,
    Client,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
