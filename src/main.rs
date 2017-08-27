use std::env;
use std::fs;
use std::io::Read;

fn main() {
   let mut cpu: Cpu = Default::default();
   let mut memory_map = MemoryMap::new();
   loop{
    cpu.process(&mut memory_map);
   }
}

fn read_byte_from_rom(counter:u16) -> u8 {
    let filename = env::args().nth(1).unwrap();
    let mut file = fs::File::open(&filename).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf[counter as usize]
    
}

#[derive(Default, Debug)]
struct Cpu {
    //registers
    reg_pc: u16,
    a:u8,
    b:u8,
    c:u8,
    d:u8,
    e:u8,
    f:u8,
    h:u8,
    l:u8,

}

impl Cpu {
    fn process(&mut self, memory_map:&mut MemoryMap) {
        let opcode = read_byte_from_rom(self.reg_pc);
        println!("opcode: {:#x}", opcode);
        match opcode {
            //jp - get next two bytes and jump to addr
            0xc3 => {
                let byte1 = read_byte_from_rom(self.reg_pc + 1) as u16;
                let byte2 = (read_byte_from_rom(self.reg_pc + 2) as u16) << 8;
                self.reg_pc = byte2 + byte1;
            },
            //xor a
            0xaf => {
                self.a = 0;
                self.reg_pc += 1;
            },
            //ld hl
            0x21 => {
                let byte1 = read_byte_from_rom(self.reg_pc + 1);
                let byte2 = read_byte_from_rom(self.reg_pc + 2);
                self.h = byte2;
                self.l = byte1;
                self.reg_pc += 3;
            },
            //ld c
            0xe => {
                let byte = read_byte_from_rom(self.reg_pc +1);
                self.c = byte;
                self.reg_pc += 2;
            },
            //ld b
            0x6 => {
                let byte = read_byte_from_rom(self.reg_pc +1);
                self.b = byte;
                self.reg_pc += 2;
            },
            //ld hl, a
            0x32 => {
                //Save A at (HL) and decrement HL
                self.h = 0;
                self.l = self.a;
                let hl = (self.l as u16).wrapping_sub(1);
                self.h = (hl >> 8) as u8;
                self.l = hl as u8;
                print!("HL: {:#x}", hl);
                self.reg_pc += 1;
            },
            //dec b
            0x5 => {
                self.b = self.b.wrapping_sub(1);
                self.reg_pc += 1;
            }
            _=> {
                println!("{:?}", self);
                panic!("Unrecognized opcode: {:#x}", opcode);
            }
        }
    }
    
}

#[derive(Default, Debug)]
struct MemoryMap {
   memory: Box<[u16]> //0x0000-0x3fff    
}

impl MemoryMap {
    fn new() -> MemoryMap {
        const MEMORY_SIZE: usize = 0x3fff - 0x0000;
        MemoryMap {
            memory:  vec![0; MEMORY_SIZE].into_boxed_slice()
        }
    }
    fn read(&self, addr: u16) {
        println!("address: {:#x}{:#x}", &self.memory[addr as usize], &self.memory[(addr + 1) as usize]);
    }
    fn write(&mut self, addr: u16, val: u16) {
        self.memory[addr as usize] = val;
    }
}
