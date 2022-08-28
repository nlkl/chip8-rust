extern crate sdl2;

mod cpu;
mod display;
mod emulator;
mod register;
mod system;

use emulator::{Emulator, EmulatorSettings};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 320;

fn main() {
    let emulator_settings = EmulatorSettings::default();
    let mut emulator = Emulator::new(emulator_settings);
    emulator.display.set(1, 1, true);
    emulator.display.apply_sprite(10, 10, vec![0xFF, 0xE7, 0xE7, 0xFF]);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Chip-8", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    emulator.execute(|disp| {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return false;
                },
                _ => {}
            }
        }

        let (window_width, window_height) = canvas.output_size().expect("Could not retrieve canvas output size.");
        let width_scale = window_width / u32::from(display::WIDTH);
        let heigh_scale = window_height / u32::from(display::HEIGHT);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for (x, y) in disp.get_active() {
            let x_canvas = (x as u32 * width_scale) as i32;
            let y_canvas = (y as u32 * heigh_scale) as i32;
            canvas.fill_rect(Rect::new(x_canvas, y_canvas, width_scale, heigh_scale)).expect("Draw failed.");
        }

        canvas.present();
        return true;
    });
}