use std::time::{Duration, Instant};
use crate::cpu::{Cpu, CpuCycleResult};
use crate::display::Display;
use crate::keypad::Keypad;
use crate::settings::Settings;
use crate::state::State;

#[derive(Clone)]
pub struct EmulatorInput {
    pub quit: bool,
    pub keypad: Keypad,
}

impl EmulatorInput {
    pub fn new() -> Self {
        Self {
            quit: false,
            keypad: Keypad::new(),
        }
    }

    pub fn quit() -> Self {
        Self { quit: true, ..Self::new() }
    }
}

pub struct EmulatorOutput {
    pub display: Display,
    pub sound_playing: bool,
}

pub struct Emulator {
    settings: Settings,
    state: State,
    cpu: Cpu,
}

impl Emulator {
    pub fn new(settings: Settings, program: Vec<u8>) -> Self {
        Self {
            settings: settings,
            state: State::new(settings, program),
            cpu: Cpu::new(settings),
        }
    }

    pub fn execute<F>(&mut self, mut render: F)
    where
        F: FnMut(EmulatorOutput) -> EmulatorInput,
    {
        let cycle_duration = Duration::from_secs_f64(1.0 / self.settings.clock_speed as f64);
        let frame_duration = Duration::from_secs_f64(1.0 / self.settings.frame_rate as f64);
        let cycles_per_frame = (frame_duration.as_secs_f64() / cycle_duration.as_secs_f64()) as i64;

        loop {
            let frame_clock = Instant::now();

            self.state.decrement_delay_register();
            self.state.decrement_sound_register();

            let output = EmulatorOutput { 
                display: self.state.display.clone(),
                sound_playing: self.state.sound_playing(),
            };
            let input = render(output);
            self.state.keypad = input.keypad;

            if input.quit {
                break;
            }

            for _ in 0..cycles_per_frame {
                let cycle_result = self.cpu.cycle(&mut self.state);

                match cycle_result {
                    CpuCycleResult::Wait => {
                        break;
                    },
                    CpuCycleResult::Done => {
                        return;
                    },
                    _ => {}
                }
            };

            let frame_elapsed_duration = frame_clock.elapsed();
            if frame_elapsed_duration < frame_duration {
                std::thread::sleep(frame_duration - frame_elapsed_duration);
            }
        }
    }
}