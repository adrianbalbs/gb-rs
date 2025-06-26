pub struct Memory {
    ram: [u8; 0xFFFF],
}

/// TODO: I should probably boundary check here shouldn't I...
/// For now assume we'll pass in valid memory ranges, don't worry about mapping
/// sections of memory for now
impl Memory {
    fn read(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }
}
