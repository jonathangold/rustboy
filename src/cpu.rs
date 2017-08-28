use memory;

#[derive(Debug, Default)]
pub struct RegF {
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool
}


#[derive(Debug, Default)]
pub struct DoubleReg {
    pub bit_hi: u8,
    pub bit_lo: u8
}

#[derive(Debug, Default)]
pub struct RegAF {
    pub a: u8,
    pub f: RegF
}

#[derive(Debug, Default)]
pub struct Cpu {
    pub pc: u16,
    pub sp: u16,

    pub af: RegAF,
    pub bc: DoubleReg,
    pub de: DoubleReg,
    pub hl: DoubleReg,
}

impl Cpu {
    pub fn process(&mut self, memory: &mut memory::Memory) {
        let opcode = memory.read_address(self.pc);
        println!("{:#x}", opcode);
        match opcode {
            //inc a
            //Z 0 H -
            0x3c => {
                self.af.a += 1;
                self.pc += 1;

                //set flags
                self.af.f.n = false;
                if self.af.a == 0 {self.af.f.z = true;}
                //check for half carry
                if (((self.af.a - 1) & 0x0F) + (1 & 0x0F)) == 0x10 {
                    self.af.f.h = true;
                }
            }
            //jp nz
            0xc3 => {
                if self.af.f.z == false {
                    let addr = memory.read_rom_16(self.pc + 1);
                    self.pc = addr;
                } else {
                    self.pc += 2;        
                }
            }
            //ret
            //- - - -
           _ => {
               println!("{:#?}", self);
               panic!("unrecognized opcode: {:#x}", opcode)}
        }       
    }
}
