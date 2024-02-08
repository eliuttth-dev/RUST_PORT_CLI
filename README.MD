# Port Sniffer CLI - Rust

A simple command-line tool written in Rust to listen on a specified port and handle incoming connections. It also provides the functionality to check if a specific port is available for use.

### Usage

To run the Port Sniffer CLI, use the following command:

```bash
cargo run -- <port> <num_threads>
```

Replace <port> with the port number you want to listen on and <num_threads> with the number of threads to use for handling connections.

If you want to check the availability of a specific port, use the -j flag followed by the port number. For example:

```bash
cargo run -- -j <port>
```

### Installation

To install the Port Sniffer CLI, make sure you have Rust and Cargo installed on your system. Then, follow these steps:

1. Clone the repository or download the code.
2. Navigate to the directory containing the `main.rs` file.
3. Run the following command to build and execute the program:

```bash
cargo run -- <port> <num_threads>
```
