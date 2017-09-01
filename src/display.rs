extern crate sdl2;

use self::sdl2::event::Event;
use self::sdl2::pixels;
use self::sdl2::gfx::primitives::DrawRenderer;

const SCREEN_WIDTH:u32 = 160;
const SCREEN_HEIGHT:u32 = 144;

pub fn init(){
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys.window("Rustboy", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut lastx = 0;
    let mut lasty = 0;

    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            canvas.pixel(x as i16, y as i16, 0xFF000FFu32).unwrap();
        }
    }
    canvas.present();
}
