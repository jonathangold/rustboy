use std::env;
use std::fs;
use std::io::Read;

pub struct Memory {
    contents: Box<[u16]>
}

impl Memory {
    pub fn new() -> Memory {
        Memory { contents:  vec![0; 0xFFFF].into_boxed_slice() }
    }

    pub fn read_address(&mut self, input:u16) -> usize {
        match input {
            0x0100...0x7FFF => {self.read_rom(input - 0x100) as usize}
            0xFF80...0xFFFE => {self.contents[input as usize] as usize}
            _ => {panic!("Unrecognized Address: {:#x}", input)}
        }
    }
    pub fn read_rom_16(&mut self, addr: u16) -> u16 {
        let bit_lo = self.read_address(addr) as u16;
        let bit_hi = (self.read_address(addr + 1) as u16) << 8;
        bit_hi + bit_lo

    }

    fn read_rom(&self, counter:u16) -> u8 {
        let filename = env::args().nth(1).unwrap();
        let mut file = fs::File::open(&filename).unwrap();
        let mut file_buf = Vec::new();
        file.read_to_end(&mut file_buf).unwrap();
        file_buf[counter as usize]
    }
}
