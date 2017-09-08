extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod memory;
mod cpu;
mod display;

fn main() {
    let mut cycle = 0;
    let mut memory = memory::Memory::new();
    let mut cpu:cpu::Cpu = Default::default();
    let mut display = display::Display::new();
    let mut running = true;
    while running {
        for event in display.event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                },
                _ => {}
            }
        }
        //display.update();
        if cpu.pc > 0x100{
            println!("{:?}", cpu);
        }
        if cycle % 456 == 0 {
            fake_screen(&mut memory);
        }
        cpu.process(&mut memory);
        cpu.clock += 1;
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
