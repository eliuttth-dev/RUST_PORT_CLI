use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // Check for -help flag
    if args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
        println!("Usage: {} <port> <num_threads>", args[0]);
        std::process::exit(0);
    }

    // Check for -j flag to show available ports
    if args.len() == 3 && args[1] == "-j" {
        let port: u16 = match args[2].parse() {
            Ok(p) => p,
            Err(_) => {
                eprintln!("Invalid port number");
                std::process::exit(1);
            }
        };
        show_available_ports(port);
        std::process::exit(0);
    }

    if args.len() != 3 {
        eprintln!("Usage: {} <port> <num_threads>", args[0]);
        std::process::exit(1);
    }

    let port: u16 = match args[1].parse() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("Invalid port number");
            std::process::exit(1);
        }
    };

    let num_threads: usize = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid number of threads");
            std::process::exit(1);
        }
    };

    let listener = match TcpListener::bind(format!("0.0.0.0:{}", port)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind to port: {}", e);
            std::process::exit(1);
        }
    };

    println!("Port sniffer listening on port {}", port);

    for stream in listener.incoming().take(num_threads as usize) {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    while let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received: {}", request);
    }
}

fn show_available_ports(port: u16) {
    match TcpListener::bind(format!("0.0.0.0:{}", port)) {
        Ok(_) => println!("Port {} is available to use", port),
        Err(_) => println!("Port {} is not available", port),
    }
}
