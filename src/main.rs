use std::env;
use std::fs;
use std::io::Read;

fn main() {
   let mut counter = 0;
   for i in 0..100{
       let mut cpu = Cpu {};
       let byte = read_byte_from_rom(i);
       counter += 1;
       cpu.process(byte)
   }
}

fn read_byte_from_rom(counter:i32) -> u8 {
    let filename = env::args().nth(1).unwrap();
    let mut file = fs::File::open(&filename).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf[counter as usize]
    
}

struct Cpu {
    //registers
}

impl Cpu {
    fn process(self, byte:u8) {
        match byte {
            //jp
            0xc3 => {println!("jp")},
            _=> {panic!("{:#x}", byte)}
        }
    }
}

struct Memory {
    
}
