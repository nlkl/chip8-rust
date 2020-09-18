extern crate piston_window;

mod display;

use piston_window::*;

fn main() {
    let black = [0.0, 0.0, 0.0, 0.0];
    let white = [1.0, 1.0, 1.0, 1.0];

    let mut window: PistonWindow = WindowSettings::new("Chip-8", [640, 320])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut display = display::new_display();
    display.set(1, 1, true);
    display.apply_sprite(10, 10, vec![0xFF, 0xE7, 0xE7, 0xFF]);

    while let Some(event) = window.next() {

        let window_size = window.size();
        let window_width = window_size.width;
        let window_height = window_size.height;

        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |context, graphics, _device| {
                clear(black, graphics);

                for x in 0..display::WIDTH {
                    for y in 0..display::HEIGHT {
                        if display.is_active(x, y) {
                            let x_physical = (x as f64) * window_width / (display::WIDTH as f64);
                            let y_physical = (y as f64) * window_height / (display::HEIGHT as f64);
                            let width_physical = window_width / (display::WIDTH as f64);
                            let height_physical =  window_height / (display::HEIGHT as f64);
                            rectangle(white, [x_physical, y_physical, width_physical, height_physical], context.transform, graphics);
                        }
                    }
                }
            });
        }
    }
}