extern crate minifb;

mod display;

use minifb::{Key, Window, WindowOptions};

const WINDOW_WIDTH: usize = 640;
const WINDOW_HEIGHT: usize = 320;
const BLACK: u32 = 0x000000;
const WHITE: u32 = 0xffffff;

fn main() {
    let mut display = display::new_display();
    display.set(1, 1, true);
    display.apply_sprite(10, 10, vec![0xFF, 0xE7, 0xE7, 0xFF]);

    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = Window::new("Chip-8", WINDOW_WIDTH, WINDOW_HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for x in 0..WINDOW_WIDTH {
            for y in 0..WINDOW_HEIGHT {
                let x_display = x * display::WIDTH / WINDOW_WIDTH;
                let y_display = y * display::HEIGHT / WINDOW_HEIGHT;

                let i = y * WINDOW_WIDTH + x;
                buffer[i] =
                    if display.is_active(x_display, y_display) {
                        WHITE
                    } else {
                        BLACK
                    }
            }
        }

        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}