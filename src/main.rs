use std::fmt::Debug;
use std::net::TcpListener;

use clap::Parser;
use clap::{arg, command};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1")]
    addr: String,

    #[arg(short, long)]
    start_port: u32,

    #[arg(short, long)]
    end_port: u32,
}

fn is_port_open(server_addr: &String, port: u32) -> bool {
    let addr = format!("{}:{}", server_addr, port);
    if let Ok(listener) = TcpListener::bind(&addr) {
        drop(listener);
        true
    } else {
        false
    }
}

fn main() {
    let args = Args::parse();

    if args.start_port > args.end_port {
        println!("ERROR: start port should be greater than end port")
    }

    let port_range = args.start_port..=args.end_port;

    for port in port_range {
        if is_port_open(&args.addr, port) {
        } else {
            println!("PORT {} at ADDR {} is OCCUPIED", port, &args.addr);
        }
    }
}
