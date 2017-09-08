extern crate sdl2;

use self::sdl2::pixels;
use self::sdl2::pixels::Color;
use sdl2::rect::{Point};
use self::sdl2::keyboard::Keycode;

use self::sdl2::gfx::primitives::DrawRenderer;

const SCREEN_WIDTH: u32 = 160;
const SCREEN_HEIGHT: u32 = 144;

pub struct Display {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump
}

impl Display {
    pub fn new() -> Display {
        let sdl_context = sdl2::init().unwrap();
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys.window("rustboy", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        canvas.clear();
        canvas.present();
        Display {
            canvas: canvas,
            event_pump: event_pump
        }
    }
    pub fn update(&mut self) {
        self.canvas.set_draw_color(Color::RGB(128, 128, 128));
        self.canvas.draw_point(Point::new(60 as i32, 60 as i32)).unwrap();
        self.canvas.clear();
        self.canvas.present();
    }
}
