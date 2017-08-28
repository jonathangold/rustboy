mod memory;
mod cpu;

fn main() {
    let mut memory = memory::Memory::new();
    let mut cpu:cpu::Cpu = Default::default();
    cpu.pc = 0x0100;
    cpu.sp = 0xFFFE;
    loop {
        cpu.process(&mut memory);
    }
}
