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
            //LD SP, d16
            0x31 => { 
                self.sp = memory.read_16(self.pc + 1);
                self.pc += 3;
            }
            //XOR A
            //Z - - -
            0xaf => {
                self.a = 0x00;
                self.f.write(0b1000_0000);
                self.pc += 1;
            }
            //LD HL, d16
            //- - - -
            0x21 => {
                let data = memory.read_16(self.pc +1);
                self.write_hl(data);
                self.pc += 3;
            }
            //LD [HL-}, A
            //- - - -
            0x32 => {
                let addr = self.read_reg_16(self.h, self.l);
                memory.contents[addr as usize] = self.a;
                self.write_hl(addr - 1);
                self.pc += 1;
            }
            //JR NZ 16
            //
            0x20 => {
                if self.f.z == false {
                    let offset = memory.read_address(self.pc +1) as i32 + 0xFFFF_FF00;
                    let target = ((self.pc as i32) + offset) as u16;
                    self.pc = 2 + target;
                } else { 
                    self.pc += 2;
                }
            }
            //LD C,d8
            //- - - -
            0xe => {
                self.c = memory.read_address(self.pc + 1);
                self.pc += 2;
            }
            //LD A,d8
            //- - - -
            0x3e => {
                self.a = memory.read_address(self.pc + 1);
                self.pc += 2;
            }
            //LD (C), A
            //- - - -
            0xe2 => {
                let addr = 0xFF00 + self.c;
                memory.contents[addr as usize] = self.a;
                self.pc += 1;
            }
            //INC C
            //Z 0 H -
            0xc => {
                self.f.z = self.zero(self.c, 1);
                self.f.n = false;
                self.f.h = self.half_carry(self.c, 1);

                self.c += 1;
                self.pc += 1;
            }
            //LD [HL], A
            //- - - -
            0x77 => {
                let addr = self.read_reg_16(self.h, self.l);
                memory.contents[addr as usize] = self.a;
                self.pc += 1;
            }
            //LDH (n), A
            //- - - -
            0xe0 => {
                let addr = memory.read_address(self.pc + 1);
                memory.contents[(0xFF00 + addr as u16) as usize] = self.a;
                self.pc += 2;
            }
            //LD DE, d16
            //- - - -
            0x11 => {
                let data = memory.read_16(self.pc +1);
                self.write_de(data);
                self.pc += 3;
            }
            //LD A, (DE)
            //- - - -
            0x1a => {
                let addr = self.read_reg_16(self.d, self.e);
                let data = memory.read_address(addr);
                self.a = data;
                self.pc += 1;
            }
            //CALL a16
            //- - - -
            0xcd => {
                let addr = memory.read_16(self.pc +1);
                self.sp -= 2;
                memory.write_16(self.sp, self.pc + 1);
                self.pc = addr;
            }
            //LD C, A
            //- - - -
            0x4f => {
                self.c = self.a;
                self.pc += 1;
            }
            //LD B,d8
            //- - - -
            0x6 => {
                let data = memory.read_address(self.pc +1);
                self.b = data;
                self.pc += 2;
            }
            //PUSH BC
            //- - - -
            0xc5 => {
                self.sp -= 2;
                memory.write_16(self.sp, self.read_reg_16(self.b, self.c));
                self.pc += 1;
            }
            //RLA
            //0 0 0 C
            0x17 => {
                let val = self.a;
                self.a = self.rotate_left(val);
                self.f.z = false;
                self.pc += 1;
            }
            //POP BC
            //- - - -
            0xc1 => {
                let data = memory.read_16(self.sp);
                self.write_bc(data);
                self.sp += 2;
                self.pc += 1;
            }
            //jp a16
            //- - - -
            0xc3 => {
                let addr = memory.read_16(self.pc + 1);
                self.pc = addr;
            }
            //pop (hl)
            //- - - -
            0xe1 => {
                let data = memory.read_16(self.sp);
                self.write_hl(data);
                self.sp += 2;
                self.pc += 1
            }
            //pop (af)
            //
            0xf1 => {
                let data = memory.read_16(self.sp);
                self.write_af(data);
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

            //PREFIX CB
            //
            0xcb => {
                let inst = memory.read_address(self.pc + 1);
                self.pc += 2;
                match inst {
                    0x7c => {
                        if 0b1000_0000 & self.h == 0b1000_0000 {
                            self.f.z = false;
                        } else {
                            self.f.z = true;
                        }
                    }
                    //RL C
                    //Z 0 0 C
                    0x11 => {
                        let val = self.c;
                        self.c = self.rotate_left(val);
                    }
                    _ => {panic!("Unknown CB instruction: {:#x}", inst)}
                }
            }
            _ => {
                println!("{:#?}", self);
                panic!("unrecognized opcode: {:#x}", opcode);
            }
    }
}
fn rotate_left(&mut self, value:u8) -> u8 {
    let mut result = value << 1;
    if self.f.c {result += 1}
    if value >> 7 == 1 {self.f.c = true} else {self.f.c = false}
    if value as u8 == 0 {self.f.z = true} else {self.f.z = false}
    self.f.n = false;
    self.f.h = false;
    result // & 0xFF;
}

fn read_reg_16(&self, reg_hi:u8, reg_lo:u8) -> u16 {
    ((reg_hi as u16) << 8) + reg_lo as u16
}

fn write_af(&mut self, data:u16) {
    self.a = ((data & 0xFF00) >> 8) as u8;
    self.f.write(data as u8);
}

fn write_bc(&mut self, data:u16) {
    self.b = ((data & 0xFF00) >> 8) as u8;
    self.c = data as u8;
}

fn write_de(&mut self, data:u16) {
    self.d = ((data & 0xFF00) >> 8) as u8;
    self.e = data as u8;
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
               A: {:#x} F: {:#b}
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
