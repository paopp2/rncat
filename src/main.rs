use clap::{Parser, Subcommand};
use defaults::*;
use std::{
    io::{self, stdin, BufWriter, Write},
    net::{IpAddr, TcpStream},
};

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

    match args.mode {
        OperationMode::Client => match start_client(args.address, args.port) {
            Ok(_) => println!("Client started"),
            Err(_) => println!("Failed to start client"),
        },
        OperationMode::Server => todo!("Server mode"),
    }
}

fn start_client(addr: IpAddr, port: u16) -> io::Result<()> {
    let address = format!("{}:{}", addr, port);
    let stream = TcpStream::connect(&address)?;
    let mut writer = BufWriter::new(stream);

    loop {
        let outgoing = {
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            input.as_bytes().to_vec()
        };

        writer.write(&outgoing)?;
        writer.flush()?;
    }
}
