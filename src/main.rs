use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpStream};
use stream::reader::Reader;
use stream::packet::Packet;

mod stream {
    pub mod reader;
    pub mod writer;
    pub mod packet;
}

fn connect(server: &String, major: u32, minor: u32) {
    match TcpStream::connect(server) {
        Ok(mut stream) => {
            println!("Connected to: {} \n", server);

            let id = 10100;
            let mut packet = Packet::new();
            let client_hello = packet.build(id, major, minor);
            println!("SENDING Packet ID: {}", id);

            stream.write_all(&client_hello).expect("Failed to send packet");

            let mut header_buffer = [0u8; 7];
            match stream.read_exact(&mut header_buffer) {
                Ok(_) => {
                    let mut header_reader = Reader::new(&header_buffer);
                    
                    println!("\nRECEIVED MESSAGE");
                    println!("----- HEADER -----");
                    let packet_id = header_reader.read_u16();
                    println!("Packet ID: {}", packet_id);
                    let packet_size = header_reader.read_u24();
                    println!("Packet Size: {}", packet_size);
                    let packet_version = header_reader.read_u16();
                    println!("Packet Version: {}", packet_version);

                    let mut body_buffer = vec![0u8; packet_size as usize];
                    match stream.read_exact(&mut body_buffer) {
                        Ok(_) => {
                            // println!("DEBUG: Received packet body: {:?} \n", &body_buffer);

                            let mut reader = Reader::new(&body_buffer);

                            println!("----- PACKET -----");
                            let server_response = match packet_version {
                                8 => reader.read_byte() as u32,
                                _ => reader.read_u32(),
                            };

                            println!("Server Response: {}", server_response);

                            println!("----- PACKET IN HEX -----");
                            println!("{} \n", to_hex(&body_buffer));
                        }
                        Err(e) => {
                            eprintln!("Failed to read packet body: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read header: {}", e);
                }
            }

            stream.shutdown(Shutdown::Both).expect("Failed to shut down the connection")
        },
        Err(e) => {
            println!("Couldn't connect to server: {}", e);
        }
    }
}

fn main() {
    let mut server = String::new();
    let mut version = String::new();

    print!("Server in 'SERVER:PORT' format (e.g. game.brawlstarsgame.com:9339): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut server)
        .expect("Failed to read input");
    server = server.trim().to_string();

    print!("Version in 'MAJOR.MINOR' format (e.g. 57.325): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut version)
        .expect("Failed to read input");
    version = version.trim().to_string();

    let split_version: Vec<&str> = version.split('.').collect();

    if split_version.len() != 2 {
        println!("Version must be in 'MAJOR.MINOR' format");
        return;
    }

    let major: u32 = match split_version[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid major version");
            return;
        }
    };

    let minor: u32 = match split_version[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid minor version");
            return;
        }
    };

    connect(&server, major, minor);

    let mut exit_input = String::new();
    println!("\nPress Enter to exit...");
    io::stdin().read_line(&mut exit_input).expect("Failed?");
}

fn to_hex(data: &[u8]) -> String {
    data.iter()
        .map(|byte| format!("\\x{:02X}", byte))
        .collect::<Vec<String>>()
        .join("")
}
