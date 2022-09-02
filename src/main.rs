extern crate sdl2;

mod display;
mod emulator;
mod instructions;

use emulator::{Emulator, EmulatorSettings, EmulatorInput};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::env;
use std::fs;

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

    let mut prev_frame_input = EmulatorInput::default();

    emulator.execute(|output| {
        let mut input = EmulatorInput::default();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return EmulatorInput::quit();
                },
                _ => {}
            }
        }

        for scancode in event_pump.keyboard_state().pressed_scancodes() {
            let key_pressed = match scancode {
                Scancode::Num1 => Some(0x1),
                Scancode::Num2 => Some(0x2),
                Scancode::Num3 => Some(0x3),
                Scancode::Num4 => Some(0xC),
                Scancode::Q => Some(0x4),
                Scancode::W => Some(0x5),
                Scancode::E => Some(0x6),
                Scancode::R => Some(0xD),
                Scancode::A => Some(0x7),
                Scancode::S => Some(0x8),
                Scancode::D => Some(0x9),
                Scancode::F => Some(0xE),
                Scancode::Z => Some(0xA),
                Scancode::X => Some(0x0),
                Scancode::C => Some(0xB),
                Scancode::V => Some(0xF),
                _ => None
            };

            if let Some(key) = key_pressed  {
                input.set_key_pressed(key);
            }
        }

        for key in prev_frame_input.pressed_keys() {
            if !input.key_pressed(key) {
                input.set_key_released(key);
            }
        }

        let (window_width, window_height) = canvas.output_size().expect("Could not retrieve canvas output size.");
        let width_scale = window_width / u32::from(output.display_width);
        let heigh_scale = window_height / u32::from(output.display_height);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for (x, y) in output.visible_pixels {
            let x_canvas = (x as u32 * width_scale) as i32;
            let y_canvas = (y as u32 * heigh_scale) as i32;
            canvas.fill_rect(Rect::new(x_canvas, y_canvas, width_scale, heigh_scale)).expect("Draw failed.");
        }

        canvas.present();
        prev_frame_input = input.clone();
        return input;
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
        canvas.present();
    }
}