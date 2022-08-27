extern crate sdl2;

mod cpu;
mod display;
mod register;
mod system;

use display::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 320;

fn main() {
    let mut display = Display::new();
    display.set(1, 1, true);
    display.apply_sprite(10, 10, vec![0xFF, 0xE7, 0xE7, 0xFF]);

    // let rom = Rom.load("<path>");
    // let emulator = Emulator::new(rom);

    let cpu_duration = Duration::from_secs_f64(1.0 / 500.0);
    let frame_duration = Duration::from_secs_f64(1.0 / 60.0);
    let steps_per_frame = (frame_duration.as_secs_f64() / cpu_duration.as_secs_f64()) as i64;
    println!("Steps per frame: {}", steps_per_frame);

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

    'running: loop {
        let frame_clock = Instant::now();

        for _ in 0..steps_per_frame {
            // emulator.step();
            // Dummy work
            std::thread::sleep(Duration::from_secs_f64(1.0 / 1500.0));
        };

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
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
        for x in 0..display::WIDTH {
            for y in 0..display::HEIGHT {
                let x_canvas = (x as u32 * width_scale) as i32;
                let y_canvas = (y as u32 * heigh_scale) as i32;

                if display.is_active(x, y) {
                    canvas.fill_rect(Rect::new(x_canvas, y_canvas, width_scale, heigh_scale)).expect("Draw failed.");
                }
            }
        }

        canvas.present();

        let frame_elapsed_duration = frame_clock.elapsed();
        if frame_elapsed_duration < frame_duration {
            std::thread::sleep(frame_duration - frame_elapsed_duration);
        }

        println!("FPS: {}", 1.0 / frame_clock.elapsed().as_secs_f64());
    }
}