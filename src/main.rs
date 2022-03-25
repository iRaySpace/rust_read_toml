use serde::Deserialize;
use std::fs::read_to_string;
use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

#[derive(Debug, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
struct Config {
    server: ServerConfig,
}

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];
    stream.read(&mut buffer).unwrap();
    stream.write(&buffer).unwrap();
    stream.flush().unwrap();
}

fn serve(server_config: ServerConfig) {
    let address = format!("{}:{}", server_config.host, server_config.port);
    let listener = TcpListener::bind(address.clone()).unwrap();

    println!("Listening on {}...", address.clone());
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_stream(stream);
    }
}

fn main() {
    println!("Reading config.toml...");
    let config: Config = toml::from_str(
        &read_to_string("config/config.toml")
            .expect("Something went wrong with reading config.toml..."),
    )
    .expect("Unable to parse config.toml...");
    serve(config.server);
}
