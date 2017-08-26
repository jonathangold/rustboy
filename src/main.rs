use std::env;
use std::fs;
use std::io::Read;

fn main() {
   let mut cpu: Cpu = Default::default();
   let mut memory_map = MemoryMap::new();
   memory_map.read(0x0000);
   cpu.process();
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
    fn process(&mut self) {
        let byte = read_byte_from_rom(self.reg_pc);
        println!("{:#x}", byte);
        match byte {
            //jp - get next two bytes and jump to addr
            0xc3 => {
                let byte1 = read_byte_from_rom(self.reg_pc + 1) as u16;
                let byte2 = (read_byte_from_rom(self.reg_pc + 2) as u16) << 8;
                self.reg_pc = byte2 + byte1;
                self.process();
            },
            //xor a
            0xaf => {
                self.a = 0;
                self.reg_pc += 1;
                self.process();
            },
            //ld hl
            0x21 => {
                let byte1 = read_byte_from_rom(self.reg_pc + 1);
                let byte2 = read_byte_from_rom(self.reg_pc + 2);
                self.h = byte2;
                self.l = byte1;
                self.reg_pc += 3;
                self.process();
            },
            //ld c
            0xe => {
                let byte = read_byte_from_rom(self.reg_pc +1);
                self.c = byte;
                self.reg_pc += 2;
                self.process();
            },
            //ld b
            0x6 => {
                let byte = read_byte_from_rom(self.reg_pc +1);
                self.b = byte;
                self.reg_pc += 2;
                self.process();
            },
            //ld
            0x32 => {
                //load a into ff00+C
                let byte1 = read_byte_from_rom(self.reg_pc + 1) as u16;
                let byte2 = (read_byte_from_rom(self.reg_pc + 2) as u16) << 8;
                let addr = byte2 + byte1;
                //TODO: implement memory to run this
                self.reg_pc += 3;
                self.process();
            }
            _=> {
                println!("{:?}", self);
                panic!("{:#x}", byte);
            }
        }
    }
}

#[derive(Default, Debug)]
struct MemoryMap {
   rom_bank_0: Box<[u16]> //0x0000-0x3fff    
}

impl MemoryMap {
    fn new() -> MemoryMap {
        const rom_bank_0_size: usize = 0x3fff - 0x0000;
        MemoryMap {
            rom_bank_0:  vec![0; rom_bank_0_size].into_boxed_slice()
        }
    }
    fn read(&self, addr: u16) {
        println!("{:#x}", &self.rom_bank_0[addr as usize]);
    }
}
