pub struct Registers {
    pub a: u8, // Accumulator
    pub x: u8, // Index Register
    pub status: u8, // Status Register (flags)
}


impl Registers {
    pub fn new() -> Self {
        Registers { a: 0, x: 0, status: 0 }
    }
}