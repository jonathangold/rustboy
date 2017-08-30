use memory;

use std::fmt;

#[derive(Default)]
pub struct Cpu {
    pc: u16,
    sp: u16,

    a: u8,
    f: RegF,

    b: u8,
    c: u8,

    d: u8,
    e: u8,

    h: u8,
    l: u8
}

impl Cpu {
    pub fn process(&mut self, memory: &mut memory::Memory) {
            if memory.read_address(0xFFFF) != 0 {
                panic!("interrupt: {:#x}", memory.read_address(0xFFFF))
            }
        let opcode = memory.read_address(self.pc);
        println!("{:#x}: {:#x}", self.pc, opcode);
        match opcode {
            //jp a16
            //- - - -
            0xc3 => {
                    let addr = memory.read_16(self.pc + 1);
                    self.pc = addr;
            }
            //pop (hl)
            //pop to hl
            0xe1 => {
                let data = memory.read_16(self.sp);
                self.write_hl(data);
                self.sp += 2;
                self.pc += 1
            }
            //inc a
            //Z 0 H -
            0x3c => {
                self.f.z = self.zero(self.a, 1);
                self.f.n = false;
                self.f.h = self.half_carry(self.a, 1);

                self.a += 1;
                self.pc += 1;
            }
            //ld (bc),a
            //- - - -
            0x2 => {
                let addr = self.read_reg_16(self.b, self.c);
                self.a = memory.read_address(addr);
                self.pc += 3;
            }
            //cp a
            0xbf => {
                self.f.z = true;
                self.f.n = true;
                self.f.h = false;
                self.f.h = false;

                self.pc += 1;
            }
            
            //reti
            //- - - -
            0xd9 => {
                self.pc = memory.read_16(self.sp - 2);
                self.sp += 2;
                //TODO: set intterupts
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
                self.f.z = self.zero(self.a, self.b);
                self.f.n = false;
                self.f.h = self.half_carry(self.a, self.b);
                self.f.c = self.carry(self.a, self.b);

                self.a = self.a + self.b;
                self.pc += 1;
            }

            //or b
            //Z 0 0 0
            0xB0 => {
                let data = memory.contents[(self.pc + 1) as usize];
                self.b = self.b | data;
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
    pub fn init(&mut self) {
        self.pc = 0x0100;
        self.sp = 0xFFFE;
        self.a = 0x01;
        self.f.c = true;
        self.f.h = true;
        self.f.z = true;
        self.c = 0x13;
        self.h = 0x01;
        self.l = 0x4d;
    }

    fn read_reg_16(&self, reg_hi:u8, reg_lo:u8) -> u16 {
        ((reg_hi as u16) << 8) + reg_lo as u16
    }

    fn write_hl(&mut self, data:u16) {
        self.h = ((data & 0xFF00) >> 8) as u8;
        self.l = data as u8;
    }

    fn half_carry(&self, lhs:u8, rhs:u8) -> bool {
        ((lhs & 0x0F) + (rhs & 0x0F) & 0x10) == 0x10
    }

    fn carry(&self, lhs:u8, rhs:u8) -> bool {
        lhs as u16 + rhs as u16 > 255
    }

    fn zero(&self, lhs:u8, rhs:u8) -> bool { 
        lhs + rhs == 0 
    }
}
impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "
               PC: {:#x} SP: {:#x}
               A: {:#x} F: {:#x}
               B: {:#x} C: {:#x}
               D: {:#x} E: {:#x}
               H: {:#x} L: {:#x}", 
               self.pc, self.sp, 
               self.a, self.f.read(),
               self.b, self.c,
               self.d, self.e,
               self.h, self.l
               )
    }
}
#[derive(Default)]
struct RegF {
    z: bool,
    n: bool,
    h: bool,
    c: bool
}
impl fmt::Debug for RegF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.read())
    }
}
impl RegF {
    fn write(&mut self, data: u8) {
        self.z = (data & 0b1000_0000) != 0;
        self.n = (data & 0b0100_0000) != 0;
        self.h = (data & 0b0010_0000) != 0;
        self.c = (data & 0b0001_0000) != 0;
    }
    fn read(&self) -> u8 {
        let mut flags = 0x00;
        if self.z == true {flags = flags + 0b1000_0000};
        if self.n == true {flags = flags + 0b0100_0000};
        if self.h == true {flags = flags + 0b0010_0000};
        if self.c == true {flags = flags + 0b0001_0000};
        flags
    }
}
