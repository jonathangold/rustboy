use std::env;
use std::fs;
use std::io::Read;

fn main() {
    let mut cpu:Cpu = Default::default();
    cpu.pc = 0x0100;
    cpu.sp = 0xFFFE;
    loop {
        cpu.process();
    }
}

#[derive(Debug, Default)]
struct RegF {
    z: bool,
    n: bool,
    h: bool,
    c: bool
}


#[derive(Debug, Default)]
struct DoubleReg {
    bit_hi: u8,
    bit_lo: u8
}

#[derive(Debug, Default)]
struct RegAF {
    a: u8,
    f: RegF
}

#[derive(Debug, Default)]
struct Cpu {
    pc: u16,
    sp: u16,

    af: RegAF,
    bc: DoubleReg,
    de: DoubleReg,
    hl: DoubleReg,
}

impl Cpu {
    fn process(&mut self) {
        let opcode = read_address(self.pc);
        println!("{:#x}", opcode);
        match opcode {
            //inc a
            //Z 0 H -
            0x3c => {
                self.af.a += 1;
                self.pc += 1;
            }
            //jp nz
            0xc3 => {
                if self.af.f.z == false {
                    let addr = read_rom_16(self.pc + 1);
                    self.pc = addr;
                } else {
                    self.pc += 2;        
                }
            }
            //ret
            //- - - -
           _ => {panic!("unrecognized opcode: {:#x}", opcode)}
        }       
    }
}


fn read_address(input:u16) -> usize {
    match input {
        0x0100...0x3FFF => {read_rom(input - 0x100) as usize}
        //0xFF80...0xFFFE => {//TODO: impliment ram}
        _ => {panic!("Unrecognized Address: {:#x}", input)}
    }
}

fn read_rom(counter:u16) -> u8 {
    let filename = env::args().nth(1).unwrap();
    let mut file = fs::File::open(&filename).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf[counter as usize]
}

fn read_rom_16(addr: u16) -> u16 {
    let bit_lo = read_address(addr) as u16;
    let bit_hi = (read_address(addr + 1) as u16) << 8;
    bit_hi + bit_lo
}
