use std::env;
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use tokio::net::TcpStream as AsyncTcpStream;
use tokio::time::{self, Duration};
use tokio_util::compat::Tokio02AsyncReadCompatExt;

#[tokio::main]
async fn main() {
    // Read command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: ./port_scanner <ip> <port-range>");
        return;
    }
    let ip = match IpAddr::from_str(&args[1]) {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("Invalid IP address.");
            return;
        }
    };
    let ports: Vec<u16> = match parse_port_range(&args[2]) {
        Ok(ports) => ports,
        Err(e) => {
            eprintln!("Invalid port range: {}", e);
            return;
        }
    };

    // Scan ports
    for port in ports {
        if is_port_open(&ip, port).await {
            println!("Port {} is open.", port);
        }
    }
}

async fn is_port_open(ip: &IpAddr, port: u16) -> bool {
    match time::timeout(Duration::from_secs(5), AsyncTcpStream::connect((ip, port))).await {
        Ok(Ok(stream)) => {
            // Port is open
            drop(stream);
            true
        }
        _ => false, // Port is closed or unreachable
    }
}

fn parse_port_range(port_range: &str) -> Result<Vec<u16>, &'static str> {
    let mut ports = Vec::new();
    let parts: Vec<&str> = port_range.split('-').collect();
    if parts.len() != 2 {
        return Err("Invalid port range format.");
    }
    let start_port = match parts[0].parse::<u16>() {
        Ok(port) => port,
        Err(_) => return Err("Invalid start port."),
    };
    let end_port = match parts[1].parse::<u16>() {
        Ok(port) => port,
        Err(_) => return Err("Invalid end port."),
    };
    if start_port > end_port {
        return Err("Start port must be less than or equal to end port.");
    }
    for port in start_port..=end_port {
        ports.push(port);
    }
    Ok(ports)
}
