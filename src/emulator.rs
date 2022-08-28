use std::time::{Duration, Instant};
use crate::display::Display;

pub struct EmulatorSettings {
    /// Clock speed in Hz.
    pub frame_rate: u16,
    /// Frame rate in Hz.
    pub clock_speed: u16,
}

impl Default for EmulatorSettings {
    fn default() -> EmulatorSettings { 
        EmulatorSettings {
            frame_rate: 60,
            clock_speed: 500,
        }
    }
}

pub struct Emulator {
    pub settings: EmulatorSettings,
    pub display: Display,
}

impl Emulator {
    pub fn new(settings: EmulatorSettings) -> Emulator {
        Emulator {
            settings: settings,
            display: Display::new(),
        }
    }

    pub fn execute<F>(&mut self, mut render: F)
    where
        F: FnMut(&Display) -> bool
    {
        let cycle_duration = Duration::from_secs_f64(1.0 / self.settings.clock_speed as f64);
        let frame_duration = Duration::from_secs_f64(1.0 / self.settings.frame_rate as f64);
        let cycles_per_frame = (frame_duration.as_secs_f64() / cycle_duration.as_secs_f64()) as i64;

        println!("Cycles per frame: {}", cycles_per_frame);

        loop {
            let frame_clock = Instant::now();

            for _ in 0..cycles_per_frame {
                self.cycle();
            };

            let continue_execution = render(&self.display);

            let frame_elapsed_duration = frame_clock.elapsed();
            if frame_elapsed_duration < frame_duration {
                std::thread::sleep(frame_duration - frame_elapsed_duration);
            }

            self.decrement_timers();

            println!("FPS: {}", 1.0 / frame_clock.elapsed().as_secs_f64());

            if !continue_execution {
                break;
            }
        }
    }

    fn cycle(&mut self) {
        // Dummy work
        std::thread::sleep(Duration::from_secs_f64(1.0 / 1500.0));
    }

    fn decrement_timers(&mut self) {
        // TODO
    }
}