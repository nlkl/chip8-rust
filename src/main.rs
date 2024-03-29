extern crate sdl2;

mod speaker;
mod cpu;
mod display;
mod emulator;
mod instructions;
mod keypad;
mod settings;
mod state;

use emulator::{Emulator, EmulatorInput};
use settings::Settings;
use speaker::{SpeakerSettings, Speaker};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
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

    let emulator_settings = Settings::default();
    let mut emulator = Emulator::new(emulator_settings, program);

    let sdl_context = sdl2::init().expect("Could not initialize SDL2.");

    let audio_subsystem = sdl_context.audio().expect("Could not initialize audio subsystem.");
    let speaker_settings = SpeakerSettings::default();
    let mut speaker = Speaker::new(audio_subsystem, speaker_settings);

    let video_subsystem = sdl_context.video().expect("Could not initialize video subsystem.");
    let window = video_subsystem
        .window("Chip-8", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .expect("Could not build window.");
    let mut canvas = window.into_canvas().build().expect("Could not build canvas.");
    let mut event_pump = sdl_context.event_pump().expect("Could not obtain event pump.");

    let mut input = EmulatorInput::new();

    emulator.execute(|output| {
        if output.sound_playing {
            speaker.play()
        } else {
            speaker.pause()
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return EmulatorInput::quit();
                },
                Event::KeyDown { keycode: Some(Keycode::M), .. } => {
                    if speaker.volume() > 0 {
                        speaker.set_volume(0);
                    } else {
                        speaker.set_volume(speaker_settings.volume);
                    }
                },
                _ => {}
            }
        }

        input.keypad.release_all_keys();
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
                input.keypad.set_key_pressed(key);
            }
        }

        let (window_width, window_height) = canvas.output_size().expect("Could not retrieve canvas output size.");
        let width_scale = window_width / u32::from(output.display.width);
        let heigh_scale = window_height / u32::from(output.display.height);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for (x, y) in output.display.visible_pixels() {
            let x_canvas = (x as u32 * width_scale) as i32;
            let y_canvas = (y as u32 * heigh_scale) as i32;
            canvas.fill_rect(Rect::new(x_canvas, y_canvas, width_scale, heigh_scale)).expect("Draw failed.");
        }

        canvas.present();
        return input.clone();
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