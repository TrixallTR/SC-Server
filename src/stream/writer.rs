pub struct Writer {
    pub stream: Vec<u8>
}

impl Writer {
    pub fn new() -> Writer {
        Writer {
            stream: Vec::new()
        }
    }

    pub fn write<T: Into<Vec<u8>>>(&mut self, data: T) {
        self.stream.extend_from_slice(&data.into());
    }

    pub fn write_u16(&mut self, data: u16) {
        self.write(data.to_be_bytes());
    }

    pub fn write_u24(&mut self, data: u32) {
        let bytes = [(data >> 16) as u8, (data >> 8) as u8, data as u8];
        self.write(bytes);
    }

    pub fn write_u32(&mut self, data: u32) {
        self.write(data.to_be_bytes());
    }

    pub fn write_string(&mut self, data: &str) {
        self.write_u32(data.len() as u32);
        self.write(data);
    }

    pub fn length(&self) -> usize {
        return self.stream.len()
    }

    pub fn display(&self) {
        println!("Stream content: {:?} \n", self.stream);
    }
}