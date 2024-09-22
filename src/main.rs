use clap::{Parser, Subcommand};
use core::str;
use defaults::*;
use std::{
    io::{self, stdin, BufRead, BufReader, BufWriter, Write},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
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
    let Args {
        mode,
        address,
        port,
    } = Args::parse();

    let socket_address = SocketAddr::new(address, port);
    match mode {
        OperationMode::Client => match start_client(socket_address.clone()) {
            Ok(_) => println!("Client started"),
            Err(_) => println!("Failed to start client"),
        },
        OperationMode::Server => match start_server(socket_address.clone()) {
            Ok(_) => println!("Server started"),
            Err(_) => println!("Failed to start server"),
        },
    }
}

fn start_client(socket_address: SocketAddr) -> io::Result<()> {
    let stream = TcpStream::connect(&socket_address)?;
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

fn start_server(socket_address: SocketAddr) -> io::Result<()> {
    let listener = TcpListener::bind(&socket_address)?;
    let (stream, _) = listener.accept()?;
    let mut reader = BufReader::new(&stream);

    loop {
        let mut incoming: Vec<u8> = vec![];
        let num_bytes_read = reader.read_until(b'\n', &mut incoming)?;

        // Check for EOF (stream closed)
        if num_bytes_read == 0 {
            return Ok(());
        }

        println!("{}", String::from_utf8_lossy(&incoming).trim());
    }
}
