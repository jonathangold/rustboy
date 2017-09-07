use memory;

use std;
use std::fmt;

#[derive(Default)]
pub struct Cpu {

    pub clock: u8,

    pub pc: u16,
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

macro_rules! ld_16 {
    ($self:expr, $hi:ident, $lo:ident, $memory:expr) => {
        {
            $self.$hi = $memory.read_address($self.pc + 3);
            $self.$lo = $memory.read_address($self.pc + 2);
            $self.pc += 1;
        }
    }
}
macro_rules! ld_nn_n {
    ($self:expr, $hi:ident, $lo:ident, $reg:ident, $memory:expr) => {
        {
            let addr = $self.read_reg_16($self.$hi, $self.$lo);
            $self.$reg = $memory.read_address(addr);
            $self.pc += 3;
        }
    }
}

macro_rules! inc_16 {
    ($self:expr, $hi:ident, $lo:ident) => {
        {
            let result = $self.read_reg_16($self.$hi, $self.$lo) + 1;
            $self.$hi = ((result & 0xFF00) >> 8) as u8;
            $self.$lo = result as u8;
        }
    }
}

macro_rules! inc {
    ($self:expr, $reg:ident) => {
        {
            $self.$reg += 1;
            $self.f.z = $self.zero($self.$reg + 1);
            $self.f.n = false;
            $self.f.h = self.half_carry_addition($self.$reg, 1);
        }
    }
}

macro_rules! dec {
    ($self:expr, $var:ident) => {
        {
            $self.$var -= 1;
            $self.f.z = $self.zero($self.$var);
            $self.f.n = true;
            $self.f.h = $self.half_carry_subtraction($self.$var + 1, $self.$var);
            $self.pc += 1;
        }
    }
}

impl Cpu {
    pub fn process(&mut self, memory: &mut memory::Memory) {
        let opcode = memory.read_address(self.pc);
        // println!("{:#?}", self);
        // println!("{:#x}: {:#x}", self.pc, opcode);
        match opcode {
            //LD BC, d16
            //- - - -
            0x1 => { ld_16!(self, b, c, memory) }
            //ld (bc),a
            //- - - -
            0x2 => { ld_nn_n!(self, b, c, a, memory) }
            //INC BC
            0x3 => { inc_16!(self, b, c) }
            //INC B
            //Z 0 H -
            0x4 => {
                self.f.z = self.zero(self.b + 1);
                self.f.n = false;
                self.f.h = self.half_carry_addition(self.b, 1);

                self.b += 1;
                self.pc += 1;
            }
            //DEC B
            //Z 1 H -
            0x5 => { dec!(self, b) }
            //DEC BC
            //- - - -
            0xb => {
                let data = self.read_reg_16(self.b, self.c);
                self.write_bc(data - 1);
                self.pc += 1;
            }
            //LD A,(HL+)
            //- - - -
            0x2a => {
                let addr = self.read_reg_16(self.h, self.l);
                self.a = memory.read_address(addr);
                self.write_hl(addr + 1);
                self.pc += 1;
            }
            //CPL
            //- 1 1 -
            0x2f => {
                let result = !self.a;
                self.a = result;
                self.f.n = true;
                self.f.h = true;
                self.pc += 1;
            }
            //LD SP, d16
            //- - - -
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
            //LD (HL),d8
            //- - - -
            0x36 => {
                let addr = self.read_reg_16(self.h, self.l);
                let data = memory.read_address(self.pc +1);
                memory.contents[addr as usize] = data;
                self.pc += 2;
            }
            //JR NZ r8
            //- - - -
            0x20 => {
                if !self.f.z {
                    let offset = memory.read_address(self.pc +1) as i8;
                    let target = ((self.pc as i32) + offset as i32) as u16;
                    self.pc = 2 + target;
                } else { 
                    self.pc += 2;
                }
            }
            //LD A,d8
            //- - - -
            0x3e => {
                self.a = memory.read_address(self.pc + 1);
                self.pc += 2;
            }
            //LD C,d8
            //- - - -
            0xe => {
                self.c = memory.read_address(self.pc + 1);
                self.pc += 2;
            }
            //LD D,d8
            //- - - -
            0x16 => {
                self.d = memory.read_address(self.pc + 1);
                self.pc += 2;
            }
            //LD L,d8
            //- - - -
            0x2e => {
                self.l = memory.read_address(self.pc + 1);
                self.pc += 2;
            }
            //LD (C), A
            //- - - -
            0xe2 => {
                let addr = 0xFF00 + self.c as u16;
                memory.contents[addr as usize] = self.a;
                self.pc += 1;
            }
            //INC C
            //Z 0 H -
            0xc => {
                self.f.z = self.zero(self.c + 1);
                self.f.n = false;
                self.f.h = self.half_carry_addition(self.c, 1);

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
                memory.write_16(self.sp, self.pc + 3);
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
                self.f.n = false;
                self.f.h = false;
                if (val >> 7) == 1 {
                    self.f.c = true;
                } else {
                    self.f.c = false;
                }
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

            //DEC D
            //Z 1 H -
            0x15 => {
                self.d -= 1;
                self.f.n = true;
                self.f.z = self.zero(self.d);
                self.f.h = self.half_carry_subtraction(self.d + 1, self.d);
                self.pc += 1
            }
            //LD (HL+),A
            //- - - -
            0x22 => {
                let addr = self.read_reg_16(self.h, self.l);
                memory.contents[addr as usize] = self.a;
                let result = self.read_reg_16(self.h, self.l) + 1;
                self.write_hl(result);
                self.pc += 1;
            }
            //INC HL
            //- - - -
            0x23 => {
                let result = self.read_reg_16(self.h, self.l) + 1;
                self.write_hl(result);
                self.pc += 1;
            }
            //RET
            //- - - -
            0xc9 => {
                self.pc = memory.read_16(self.sp);
                self.sp += 2;
            }
            //INC DE
            //- - - -
            0x13 => {
                let result = self.read_reg_16(self.d, self.e) + 1;
                self.write_de(result);
                self.pc += 1;
            }
            //LD A, B
            //- - - -
            0x78 => {
                self.a = self.b;
                self.pc += 1;
            }
            //LD A, E
            //- - - -
            0x7b => {
                self.a = self.e;
                self.pc += 1;
            }
            //LD A, L
            //- - - -
            0x7d => {
                self.a = self.l;
                self.pc += 1;
            }
            //CP (HL)
            //Z 1 H C
            0xbe => {
                let addr = self.read_reg_16(self.h, self.l);
                let data = memory.read_address(addr);
                if (self.a - data) == 0 {
                    self.f.z = true;
                } else {
                    self.f.z = false;
                }
                self.f.n = true;
                self.f.h = self.half_carry_subtraction(self.a, data);
                self.f.c = self.carry_subtraction(self.a, data);
                self.pc += 1;

            }
            //CP d8
            //Z 1 H C
            0xfe => {
                let data = memory.read_address(self.pc + 1);
                if (self.a - data) == 0 {
                    self.f.z = true;
                } else {
                    self.f.z = false;
                }
                self.f.n = true;
                self.f.h = self.half_carry_subtraction(self.a, data);
                self.f.c = self.carry_subtraction(self.a, data);
                self.pc += 2;
            }
            //XOR d8
            //Z 0 0 0
            0xee => {
                let data = memory.read_address(self.pc + 1);
                self.a = data ^ self.a;
                self.f.write(0);
                if self.a == 0 {self.f.z = true}
                self.pc += 1;
            }
            //LD B,E
            //- - - -
            0x34 => {
                self.b = self.e;
                self.pc += 1;
            }
            //LD (a16), A
            //- - - -
            0xea => {
                self.a = memory.read_address(self.pc + 1);
                self.pc += 3;
            }
            //DEC A
            //Z 1 H -
            0x3d => {
                self.a -= 1;
                self.f.n = true;
                self.f.z = self.zero(self.a);
                self.f.h = self.half_carry_subtraction(self.a + 1, self.a);
                self.pc += 1

            }
            //JR Z r8
            //- - - -
            0x28 => {
                if self.f.z {
                    let offset = memory.read_address(self.pc +1) as i8;
                    let target = ((self.pc as i32) + offset as i32) as u16;
                    self.pc = 2 + target;
                } else { 
                    self.pc += 2;
                }
            }
            //JR r8
            //- - - -
            0x18 => {
                let offset = memory.read_address(self.pc +1) as i8;
                let target = ((self.pc as i32) + offset as i32) as u16;
                self.pc = 2 + target;
            }
            //DEC C
            //Z 1 H -
            0xd => {
                self.c -= 1;
                self.f.n = true;
                self.f.z = self.zero(self.c);
                self.f.h = self.half_carry_subtraction(self.c + 1, self.c);
                self.pc += 1

            }
            //DEC E
            //Z 1 H -
            0x1d => {
                self.e -= 1;
                self.f.n = true;
                self.f.z = self.zero(self.e);
                self.f.h = self.half_carry_subtraction(self.e + 1, self.e);
                self.pc += 1

            }
            //LD H,A
            //- - - -
            0x67 => {
                self.h = self.a;
                self.pc += 1;
            }
            //LD A,H
            //- - - -
            0x7c => {
                self.a - self.h;
                self.pc += 1;
            }
            //LD D,A
            //- - - -
            0x57 => {
                self.d = self.a;
                self.pc += 1;
            }
            //INC H
            //Z 0 H -
            0x24 => {
                self.f.z = self.zero(self.h + 1);
                self.f.n = false;
                self.f.h = self.half_carry_addition(self.h, 1);

                self.h += 1;
                self.pc += 1;
            }

            //LD E, d8
            //- - - -
            0x1e => {
                self.e = memory.read_address(self.pc + 1);
                self.pc += 2;
            }
            //LDH a, (a8)
            //- - - -
            0xf0 => {
                let addr = 0xFF00 + memory.read_address(self.pc + 1) as u16;
                self.a = memory.read_address(addr);
                self.pc += 2;
            }
            //SUB B
            //Z 1 H C
            0x90 => {
                if (self.a - self.b) == 0 {
                    self.f.z = true;
                } else {
                    self.f.z = false;
                }
                self.f.n = true;
                self.f.h = self.half_carry_subtraction(self.a, self.b);
                self.f.c = self.carry_subtraction(self.a, self.b);
                self.a = self.a - self.b;
                self.pc += 1
            }
            //ADD A, (HL)
            //Z 0 H C
            0x86 => {
                let addr = self.read_reg_16(self.h, self.l);
                let val = memory.read_address(addr);
                self.f.z = self.zero(self.a + val);
                self.f.n = false;
                self.f.h = self.half_carry_addition(self.a, val);
                self.f.c = self.carry(self.a, val);

                self.a = self.a + val;
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
                self.f.z = self.zero(self.a + 1);
                self.f.n = false;
                self.f.h = self.half_carry_addition(self.a, 1);

                self.a += 1;
                self.pc += 1;
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
                self.pc = memory.read_16(self.sp);
                self.sp += 2;
                //set intterupts
                memory.contents[0xFFFF] = 0xFF;
            }
            //EI
            //- - - -
            0xfb => {
                memory.contents[0xFFFF] = 0xFF;
                self.pc += 1;
            }
            //DI
            //- - - -
            0xf3 => {
                memory.contents[0xFFFF] = 0x00;
                self.pc += 1;
            }
            //add a, b
            //Z 0 H C
            0x80 => {
                self.f.z = self.zero(self.a + self.b);
                self.f.n = false;
                self.f.h = self.half_carry_addition(self.a, self.b);
                self.f.c = self.carry(self.a, self.b);

                self.a = self.a + self.b;
                self.pc += 1;
            }
            //OR C
            //Z 0 0 0
            0xb1 => {
                let data = self.a | self.c;
                self.a = data;
                self.f.write(0x00);
                self.f.z = self.zero(self.a);
                self.pc += 1;
            }
            //OR B
            //Z 0 0 0
            0xb0 => {
                let data = self.a | self.b;
                self.a = data;
                self.f.write(0x00);
                self.f.z = self.zero(self.a);
                self.pc += 1;
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
                        self.f.z = self.zero(self.c);
                        self.f.n = false;
                        self.f.h = false;
                        if (val >> 7) == 1 {
                            self.f.c = true;
                        } else {
                            self.f.c = false;
                        }
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

fn half_carry_addition(&self, lhs:u8, rhs:u8) -> bool {
    ((lhs & 0x0F) + (rhs & 0x0F) & 0x10) == 0x10
}

fn half_carry_subtraction(&self, lhs:u8, rhs:u8) -> bool {
    ((lhs & 0x0F) - (rhs & 0x0F) & 0b0000_1000) == 0b0000_1000
}

fn carry(&self, lhs:u8, rhs:u8) -> bool {
    lhs as u16 + rhs as u16 > 255
}
fn carry_subtraction(&self, lhs:u8, rhs:u8) -> bool {
    lhs >= 128 && rhs < 128
}

fn zero(&self, val:u8) -> bool { 
    val == 0 
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
