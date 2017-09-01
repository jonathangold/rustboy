mod memory;
mod cpu;
mod display;

fn main() {
    let mut cycle = 0;
    let mut memory = memory::Memory::new();
    let mut cpu:cpu::Cpu = Default::default();
    display::init();
    loop {
        if cpu.pc > 0x100 {
            println!("{:?}", cpu);
        }
        if cycle % 456 == 0 {
            fake_screen(&mut memory);
        }
        cpu.process(&mut memory);
        cycle += 1
    }
}

fn fake_screen(memory: &mut memory::Memory) {
    if memory.contents[0xFF44] < 154 {
        memory.contents[0xFF44] += 1;
    } else {
        memory.contents[0xFF44] = 0;
    }
}
