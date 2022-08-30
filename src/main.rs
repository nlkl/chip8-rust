extern crate sdl2;

mod display;
mod emulator;

use emulator::{Emulator, EmulatorSettings, EmulatorInput};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::env;
use std::fs;
use std::time::Duration;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 320;

fn main() {
    let mut args = env::args();
    let path = args.nth(1).expect("Please provide a path to a valid program.");
    let program = fs::read(path).expect("Could not load program.");

    let emulator_settings = EmulatorSettings::default();
    let mut emulator = Emulator::new(emulator_settings, program);

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

    emulator.execute(|output| {
        let mut key_pressed = None;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return EmulatorInput { quit: true, key_pressed: None };
                },
                Event::KeyDown { keycode: Some(code), .. } => {
                    key_pressed = match code {
                        Keycode::Num1 => Some(0x1),
                        Keycode::Num2 => Some(0x2),
                        Keycode::Num3 => Some(0x3),
                        Keycode::Num4 => Some(0xC),
                        Keycode::Q => Some(0x4),
                        Keycode::W => Some(0x5),
                        Keycode::E => Some(0x6),
                        Keycode::R => Some(0xD),
                        Keycode::A => Some(0x7),
                        Keycode::S => Some(0x8),
                        Keycode::D => Some(0x9),
                        Keycode::F => Some(0xE),
                        Keycode::Z => Some(0xA),
                        Keycode::X => Some(0x0),
                        Keycode::C => Some(0xB),
                        Keycode::V => Some(0xF),
                        _ => None
                    };
                    if key_pressed.is_some()  {
                        break;
                    }
                },
                _ => {}
            }
        }

        let (window_width, window_height) = canvas.output_size().expect("Could not retrieve canvas output size.");
        let width_scale = window_width / u32::from(output.display_width);
        let heigh_scale = window_height / u32::from(output.display_height);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for (x, y) in output.displayed_pixels {
            let x_canvas = (x as u32 * width_scale) as i32;
            let y_canvas = (y as u32 * heigh_scale) as i32;
            canvas.fill_rect(Rect::new(x_canvas, y_canvas, width_scale, heigh_scale)).expect("Draw failed.");
        }

        canvas.present();
        return EmulatorInput { quit: false, key_pressed: key_pressed };
    });

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return;
                },
                _ => {}
            }
        }
    }
}