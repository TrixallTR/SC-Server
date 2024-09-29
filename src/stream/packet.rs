use super::writer::Writer;

pub struct Packet {
    pub header: Writer,
    pub packet: Writer
}

impl Packet {
    pub fn new() -> Packet {
        Packet {
            header: Writer::new(),
            packet: Writer::new()
        }
    }

    pub fn write_header(&mut self, id: u16) {
        self.header.write_u16(id);
        self.header.write_u24(self.packet.length() as u32);
        self.header.write_u16(0);
    }

    pub fn write_packet(&mut self, major: u32, minor: u32, build: u32) {
        self.packet.write_u32(0);
        self.packet.write_u32(0);
        self.packet.write_u32(major);
        self.packet.write_u32(build);
        self.packet.write_u32(minor);
        self.packet.write_string("");
        self.packet.write_u32(2);
        self.packet.write_u32(2);
    }

    pub fn build(&mut self, id: u16, major: u32, minor: u32, build: u32) -> Vec<u8> {
        self.write_packet(major, minor, build);
        self.write_header(id);
        let mut complete_packet = self.header.stream.clone();
        complete_packet.extend(&self.packet.stream);
        complete_packet
    }

    pub fn display(&self) {
        println!("----- HEADER -----");
        self.header.display();
        println!("----- PACKET -----");
        self.packet.display();
    }
}