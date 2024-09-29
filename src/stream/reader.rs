pub struct Reader {
    pub stream: Vec<u8>
}

impl Reader {
    pub fn new(data: &[u8]) -> Reader {
        Reader {
            stream: data.to_vec()
        }
    }

    pub fn read(&mut self, size: usize) -> Vec<u8> {
        if size > self.stream.len() {
            panic!("Requested size bigger than stream length");
        }
        let result = self.stream[..size].to_vec();
        self.stream.drain(..size);
        result
    }

    pub fn read_u16(&mut self) -> u16 {
        let result = self.read(2);
        u16::from_be_bytes(result.try_into().expect("Failed to convert bytes to u16"))
    }

    pub fn read_u24(&mut self) -> u32 {
        let result = self.read(3);
        ((result[0] as u32) << 16) | ((result[1] as u32) << 8) | (result[2] as u32)
    }

    pub fn read_u32(&mut self) -> u32 {
        let result = self.read(4);
        u32::from_be_bytes(result.try_into().expect("Failed to convert bytes to u32"))
    }

    pub fn read_byte(&mut self) -> u8 {
        let result = self.read(1);
        u8::from_be_bytes(result.try_into().expect("Failed to convert bytes to u8"))
    }

    pub fn length(&self) -> usize {
        return self.stream.len();
    }

    pub fn display(&self) {
        println!("Stream content: {:?} \n", self.stream);
    }
}