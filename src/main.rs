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
    let (server, version, save) = get_config();
    connect(&server, version, save);

    println!("\nPress Enter to exit...");
    let mut exit_input = String::new();
    io::stdin().read_line(&mut exit_input).unwrap();
}

fn connect(server: &String, version: Vec<u32>, save: bool) {
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

                            if save {
                                let file_name = format!("packet_{}.bin", packet_id);
                                let mut full_packet = header_buffer.to_vec();
                                full_packet.extend_from_slice(&body_buffer);
                                std::fs::write(&file_name, &full_packet).expect("Failed to save packet to file");
                                println!("Packet saved to: {}", file_name);
                            }
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

fn get_config() -> (String, Vec<u32>, bool) {
    loop {
        let mut server = String::new();
        let mut version = String::new();
        let mut save = String::new();

        print!("Server in 'SERVER:PORT' format (e.g. game.brawlstarsgame.com:9339): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut server).unwrap();
        server = server.trim().to_string();

        print!("Version in 'MAJOR.MINOR[.BUILD]' format (e.g. 57.325 or 57.325.1): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut version).unwrap();
        version = version.trim().to_string();

        print!("Save Packet To File? (Y/N): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut save).unwrap();
        save = save.trim().to_string();

        let split_server: Vec<&str> = server.split(':').collect();
        let split_version: Vec<&str> = version.split('.').collect();

        if split_server.len() == 2 {
            if split_version.len() >= 2 {
                let save = if save.to_uppercase() == "Y" { true } else { false };
                let major = split_version[0].parse().expect("Invalid major");
                let minor = split_version[1].parse().expect("Invalid minor");
                let build = if split_version.len() == 3 { split_version[2].parse().expect("Invalid build") } else { 1 };

                return (server, vec![major, minor, build], save);
            } 
            else {
                eprintln!("Version must be in 'MAJOR.MINOR.BUILD' OR 'MAJOR.MINOR' format");
            }
        } 
        else {
            eprintln!("Server must be in 'SERVER:PORT' format");
        }
    }
}

fn to_hex(data: &[u8]) -> String {
    data.iter()
        .map(|byte| format!("\\x{:02X}", byte))
        .collect::<Vec<String>>()
        .join("")
}