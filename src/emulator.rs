use std::time::{Duration, Instant};
use crate::cpu::{Cpu, CpuCycleResult};
use crate::display::Display;
use crate::keypad::Keypad;
use crate::settings::Settings;
use crate::state::State;

const SPRITE_DATA: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

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
}

pub struct Emulator {
    settings: Settings,
    state: State,
    cpu: Cpu,
}

impl Emulator {
    pub fn new(settings: Settings, program: Vec<u8>) -> Emulator {
        let display = Display::new(settings.display_width, settings.display_height, settings.use_sprite_wrapping);
        let keypad = Keypad::new();
        let mut state = State::new(settings.memory_size, display, keypad);
        let program_counter = settings.program_start_address;
        state.set_program_counter(program_counter);
        state.write_memory(program_counter, &program);
        state.write_memory(settings.sprite_start_address, &SPRITE_DATA);

        Self {
            settings: settings,
            state: state,
            cpu: Cpu { settings },
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

            let output = EmulatorOutput { display: self.state.display.clone() };
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