use memory;

#[derive(Debug, Default)]
pub struct Cpu {
    pub pc: u16,
    pub sp: u16,

    pub af: RegAF,
    pub bc: RegBC,
    pub de: RegDE,
    pub hl: RegHL,
}

impl Cpu {
    pub fn process(&mut self, memory: &mut memory::Memory) {
        let opcode = memory.read_address(self.pc);
        println!("{:#x}", opcode);
        match opcode {
            //inc a
            //Z 0 H -
            0x3c => {
                self.af.f.z = self.zero(self.af.a, 1);
                self.af.f.n = false;
                self.af.f.h = self.halfCarry(self.af.a, 1);

                self.af.a += 1;
                self.pc += 1;
            }
            //jp nz
            0xc3 => {
                if self.af.f.z == false {
                    let addr = memory.read_16(self.pc + 1);
                    self.pc = addr;
                } else {
                    self.pc += 2;        
                }
            }
            //ret
            //- - - -
            0xc9 => {
                self.pc = memory.read_16(self.sp - 2);
                self.sp += 2;
            }
            //add a, b
            //Z 0 H C
            0x80 => {
                self.af.f.z = self.zero(self.af.a, self.bc.b);
                self.af.f.n = false;
                self.af.f.h = self.halfCarry(self.af.a, self.bc.b);
                self.af.f.c = self.carry(self.af.a, self.bc.b);

                self.af.a = self.af.a + self.bc.b;
                self.pc += 1;
            }

            //or b
            //Z 0 0 0
            0xB0 => {
                let data = memory.contents[(self.pc + 1) as usize];
                self.bc.b = self.bc.b | data;
                self.pc += 2;
            }
            //nop
            0x0 => {
                self.pc +=1;
            }

           _ => {
               println!("{:#?}", self);
               panic!("unrecognized opcode: {:#x}", opcode)}
        }       
    }

    fn halfCarry(&self, lhs:u8, rhs:u8) -> bool {
        ((lhs & 0x0F) + (rhs & 0x0F) & 0x10) == 0x10
    }

    fn carry(&self, lhs:u8, rhs:u8) -> bool {
        lhs as u16 + rhs as u16 > 255
    }

    fn zero(&self, lhs:u8, rhs:u8) -> bool { 
        lhs + rhs == 0 
    }
}

#[derive(Debug, Default)]
pub struct RegF {
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool
}

#[derive(Debug, Default)]
pub struct RegBC {
    pub b: u8,
    pub c: u8
}

#[derive(Debug, Default)]
pub struct RegDE {
    pub d: u8,
    pub e: u8
}

#[derive(Debug, Default)]
pub struct RegHL {
    pub h: u8,
    pub l: u8
}

#[derive(Debug, Default)]
pub struct RegAF {
    pub a: u8,
    pub f: RegF
}


