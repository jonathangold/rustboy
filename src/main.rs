use std::env;
use std::fs;
use std::io::Read;

fn main() {
   let mut cpu: Cpu = Default::default();
   cpu.process();
}

fn read_byte_from_rom(counter:u16) -> u8 {
    let filename = env::args().nth(1).unwrap();
    let mut file = fs::File::open(&filename).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf[counter as usize]
    
}

#[derive(Default)]
struct Cpu {
    //registers
    reg_pc: u16,
    a:u8
}

impl Cpu {
    fn process(&mut self) {
        let byte = read_byte_from_rom(self.reg_pc);
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
            _=> {panic!("{:#x}", byte)}
        }
    }
}

struct Memory {
    
}
