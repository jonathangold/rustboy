use memory;

#[derive(Debug, Default)]
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
        let opcode = memory.read_address(self.pc);
        println!("{:#x}: {:#x}", self.pc, opcode);
        match opcode {
            //inc a
            //Z 0 H -
            0x3c => {
                self.f.z = self.zero(self.a, 1);
                self.f.n = false;
                self.f.h = self.halfCarry(self.a, 1);

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
            //jp a16
            //- - - -
            0xc3 => {
                    let addr = memory.read_16(self.pc + 1);
                    self.pc = addr;
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
                self.f.h = self.halfCarry(self.a, self.b);
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
struct RegF {
    z: bool,
    n: bool,
    h: bool,
    c: bool
}
