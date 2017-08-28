mod memory;
mod cpu;

fn main() {
    let mut memory = memory::Memory::new();
    let mut cpu:cpu::Cpu = Default::default();
    cpu.pc = 0x0100;
    cpu.sp = 0xFFFE;
    cpu.af.a = 0x01;
    cpu.af.f.c = true;
    cpu.bc.c = 0x13;
    cpu.hl.h = 0x01;
    cpu.hl.l = 0x4d;

    memory.contents[0xFF10] = 0x80;
    memory.contents[0xFF11] = 0xBF;
    memory.contents[0xFF12] = 0xF3;
    memory.contents[0xFF14] = 0xBF;
    memory.contents[0xFF16] = 0x3F;
    memory.contents[0xFF19] = 0xBF;
    memory.contents[0xFF1A] = 0x7F;
    memory.contents[0xFF1B] = 0xFF;
    memory.contents[0xFF1C] = 0x9F;
    memory.contents[0xFF1E] = 0xBF;
    memory.contents[0xFF20] = 0xFF;
    memory.contents[0xFF23] = 0xBF;
    memory.contents[0xFF24] = 0x77;
    memory.contents[0xFF25] = 0xF3;
    memory.contents[0xFF26] = 0xF1;
    memory.contents[0xFF40] = 0xF1;
    memory.contents[0xFF47] = 0xFc;
    memory.contents[0xFF48] = 0xFF;
    memory.contents[0xFF49] = 0xFF;

    loop {
        cpu.process(&mut memory);
    }
}
