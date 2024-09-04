pub struct Memory {
    pub data: [u8; 256], // 256 Bytes of Memory
}


impl Memory {
    pub fn new() -> Self {
        Memory { data: [0; 256] }
    }

    pub fn read(&self, addr: u8) -> u8 {
        self.data[ addr as usize ]
    }

    pub fn write(&mut self, addr: u8, value: u8) {
        self.data[ addr as usize ] = value
    }
}