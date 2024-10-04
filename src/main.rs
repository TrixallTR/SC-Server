use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpStream};
use stream::reader::Reader;
use stream::packet::Packet;

mod stream {
    pub mod reader;
    pub mod writer;
    pub mod packet;
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

    print!("Version in 'MAJOR.MINOR[.BUILD]' format (e.g. 57.325 or 57.325.1): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut version)
        .expect("Failed to read input");
    version = version.trim().to_string();

    connect(&server, parse_version(version));

    let mut exit_input = String::new();
    println!("\nPress Enter to exit...");
    io::stdin().read_line(&mut exit_input).unwrap();
}


fn connect(server: &String, version: Vec<u32>) {
    match TcpStream::connect(server) {
        Ok(mut stream) => {
            println!("Connected to: {} \n", server);

            let id = 10100;
            let mut packet = Packet::new();
            let client_hello = packet.build(id, version[0], version[1], version[2]);
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

            stream.shutdown(Shutdown::Both).unwrap();
        },
        Err(e) => {
            eprintln!("Couldn't connect to server: {}", e);
        }
    }
}

fn parse_version(version: String) -> Vec<u32> {
    let split_version: Vec<&str> = version.split('.').collect();

    if split_version.len() < 2 {
        eprintln!("Version must be in 'MAJOR.MINOR.BUILD' OR 'MAJOR.MINOR' format");
        main();
    }

    let major: u32 = split_version[0].parse().expect("Invalid major");
    let minor: u32 = split_version[1].parse().expect("Invalid minor");
    let build: u32 = if split_version.len() == 3 { split_version[2].parse().expect("Invalid build") } else { 1 };

    return vec![major, minor, build];
}

fn to_hex(data: &[u8]) -> String {
    data.iter()
        .map(|byte| format!("\\x{:02X}", byte))
        .collect::<Vec<String>>()
        .join("")
}