use rand;
use std::time::{Duration, Instant};
use crate::display::Display;
use crate::instructions::Instruction;

const MEMORY_LENGTH: usize = 4096;
const REGISTER_COUNT: usize = 16;

enum CycleResult {
    Continue,
    Wait,
    Done,
}

#[derive(Clone, Copy)]
pub struct EmulatorInput {
    pub quit: bool,
    pressed_keys: [bool; 0xF+1],
    released_keys: [bool; 0xF+1],
}

impl Default for EmulatorInput {
    fn default() -> Self {
        Self {
            quit: false,
            pressed_keys: [false; 0xF+1],
            released_keys: [false; 0xF+1],
        }
    }
}

impl EmulatorInput {
    pub fn quit() -> Self {
        Self { quit: true, ..Default::default() }
    }

    pub fn set_key_pressed(&mut self, key: u8) {
        assert!(key <= 0xF, "Key out of range.");
        self.pressed_keys[key as usize] = true;
    }

    pub fn set_key_released(&mut self, key: u8) {
        assert!(key <= 0xF, "Key out of range.");
        self.released_keys[key as usize] = true;
    }

    pub fn key_pressed(&self, key: u8) -> bool {
        self.pressed_keys[key as usize]
    }

    pub fn pressed_keys(&self) -> Vec<u8> {
        let mut keys = vec![];
        for key in 0..0xFu8 {
            if self.pressed_keys[key as usize] {
                keys.push(key);
            }
        }
        keys
    }

    pub fn released_key(&self) -> Option<u8> {
        for key in 0..0xFu8 {
            if self.released_keys[key as usize] {
                return Some(key);
            }
        }
        None
    }
}

pub struct EmulatorOutput {
    pub display_width: u8,
    pub display_height: u8,
    pub visible_pixels: Vec<(u8, u8)>,
}

#[derive(Clone, Copy)]
pub struct EmulatorSettings {
    /// Clock speed in Hz.
    pub frame_rate: u16,
    /// Frame rate in Hz.
    pub clock_speed: u16,
    /// The memory address at which programs start.
    pub program_start_address: u16,
    /// The width of the virtual display in px.
    pub display_width: u8,
    /// The height of the virtual display in px.
    pub display_height: u8,
    /// Shift right (8XY6) and left (8XYE) in-place on register VX rather than from VY.
    pub use_in_place_shift: bool,
    /// Jumping with offset (BNNN) uses a specified register for the offset (VX in BXNN).
    pub use_flexible_jump_offset: bool,
    /// TODO: Implement
    /// The memory read (FX65) and write (FX55) operations auto-increment the address register I.
    pub use_auto_address_increments: bool,
    /// TODO: Implement
    /// The logic operations OR (8XY1), AND (8XY2), and XOR (8XY3), reset the flag register VF to 0.
    pub use_flag_reset_on_logic_ops: bool,
    /// Sprites partially outside the display are wrapped instead of clipped.
    pub use_sprite_wrapping: bool,
    /// Sprites are only applied at the beginning of next frame.
    pub use_sprite_draw_delay: bool,
}

impl Default for EmulatorSettings {
    fn default() -> EmulatorSettings { 
        EmulatorSettings {
            frame_rate: 60,
            clock_speed: 500,
            program_start_address: 0x200,
            display_width: 64,
            display_height: 32,
            use_in_place_shift: false,
            use_flexible_jump_offset: false,
            use_auto_address_increments: true,
            use_flag_reset_on_logic_ops: false,
            use_sprite_wrapping: false,
            use_sprite_draw_delay: false,
        }
    }
}

pub struct Emulator {
    settings: EmulatorSettings,
    display: Display,
    memory: [u8; MEMORY_LENGTH],
    registers: [u8; REGISTER_COUNT],
    program_counter: u16,
    address_register: u16,
    delay_register: u8,
    sound_register: u8,
    stack: Vec<u16>
}

impl Emulator {
    pub fn new(settings: EmulatorSettings, program: Vec<u8>) -> Emulator {
        let program_counter = settings.program_start_address;
        let mut memory = [0; MEMORY_LENGTH];
        for i in 0..program.len() {
            memory[i + program_counter as usize] = program[i];
        }

        let sprites = vec![
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
        for i in 0..sprites.len() {
            memory[i] = sprites[i];
        }

        let display_width = settings.display_width;
        let display_height = settings.display_height;
        let wrap_sprites = settings.use_sprite_wrapping;
        let display = Display::new(display_width, display_height, wrap_sprites);

        Emulator {
            settings: settings,
            display: display,
            memory: memory,
            registers: [0; REGISTER_COUNT],
            program_counter: program_counter,
            address_register: 0,
            delay_register: 0,
            sound_register: 0,
            stack: vec![]
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

            self.decrement_timers();

            let output = EmulatorOutput {
                display_width: self.display.width,
                display_height: self.display.height,
                visible_pixels: self.display.visible_pixels()
            };

            let input = render(output);

            if input.quit {
                break;
            }

            for _ in 0..cycles_per_frame {
                let cycle_result = self.cycle(&input);

                match cycle_result {
                    CycleResult::Wait => {
                        break;
                    },
                    CycleResult::Done => {
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

    fn cycle(&mut self, input: &EmulatorInput) -> CycleResult {
        if self.program_counter as usize > (self.memory.len() - 2)  {
            return CycleResult::Done;
        }

        let instruction_bytes = (self.memory[self.program_counter as usize] as u16) << 8 | (self.memory[self.program_counter as usize + 1] as u16);
        let instruction = Instruction::decode(instruction_bytes);
        self.program_counter += 2;

        match instruction {
            Instruction::ClearScreen => {
                self.display.clear();
            },
            Instruction::Return => {
                let return_address = self.stack.pop().expect("Tried to return, but stack was empty.");
                self.program_counter = return_address;
            },
            Instruction::Jump { address } => {
                self.program_counter = address;
            },
            Instruction::JumpWithOffset { address } => {
                let offset = if self.settings.use_flexible_jump_offset {
                    let register = (address & 0xF00) >> 8;
                    self.registers[register as usize]
                } else {
                    self.registers[0x0]
                };
                println!("Adress: {} Offset: {}", address, offset);
                self.program_counter = address + offset as u16;
            },
            Instruction::Call { address } => {
                self.stack.push(self.program_counter);
                self.program_counter = address;
            },
            Instruction::SkipIfValue { register, comparand_value } => {
                let value = self.registers[register as usize];
                if value == comparand_value {
                    self.program_counter += 2;
                }
            },
            Instruction::SkipIfNotValue { register, comparand_value } => {
                let value = self.registers[register as usize];
                if value != comparand_value {
                    self.program_counter += 2;
                }
            },
            Instruction::SkipIfEqual { register, comparand_register } => {
                let value = self.registers[register as usize];
                let comparand_value = self.registers[comparand_register as usize];
                if value == comparand_value {
                    self.program_counter += 2;
                }
            },
            Instruction::SkipIfNotEqual { register, comparand_register } => {
                let value = self.registers[register as usize];
                let comparand_value = self.registers[comparand_register as usize];
                if value != comparand_value {
                    self.program_counter += 2;
                }
            },
            Instruction::LoadValue { register, value } => {
                self.registers[register as usize] = value;
            },
            Instruction::AddValue { register, value } => {
                let current_value = self.registers[register as usize] as u16;
                let result = current_value + value as u16;
                self.registers[register as usize] = (result & 0x00FF) as u8;
            },
            Instruction::Load { register, from_register } => {
                self.registers[register as usize] = self.registers[from_register as usize];
            },
            Instruction::Or { register, or_register } => {
                self.registers[register as usize] |= self.registers[or_register as usize];
            },
            Instruction::And { register, and_register } => {
                self.registers[register as usize] &= self.registers[and_register as usize];
            },
            Instruction::Xor { register, xor_register } => {
                self.registers[register as usize] ^= self.registers[xor_register as usize];
            },
            Instruction::Add { register, add_register } => {
                let value = self.registers[register as usize] as u16;
                let value_to_add = self.registers[add_register as usize] as u16;
                let result = value + value_to_add;
                self.registers[register as usize] = (result & 0x00FF) as u8;
                self.registers[0xF] =(result > 0xFF) as u8;
            },
            Instruction::Subtract { register, subtract_register } => {
                let value = self.registers[register as usize] as u16;
                let value_to_subtract = self.registers[subtract_register as usize] as u16;
                let result = 0x0100 + value - value_to_subtract;
                self.registers[register as usize] = (result & 0x00FF) as u8;
                self.registers[0xF] = (value_to_subtract <= value) as u8;
            },
            Instruction::SubtractFrom { register, subtract_from_register } => {
                let value_to_subtract = self.registers[register as usize] as u16;
                let value = self.registers[subtract_from_register as usize] as u16;
                let result = 0x0100 + value - value_to_subtract;
                self.registers[register as usize] = (result & 0x00FF) as u8;
                self.registers[0xF] = (value_to_subtract <= value) as u8;
            },
            Instruction::ShiftRight { register, source_register } => {
                let source_register = if self.settings.use_in_place_shift {
                    register
                } else {
                    source_register
                };
                let value = self.registers[source_register as usize];
                self.registers[register as usize] = value >> 1;
                self.registers[0xF] = value & 0x01;
            },
            Instruction::ShiftLeft { register, source_register } => {
                let source_register = if self.settings.use_in_place_shift {
                    register
                } else {
                    source_register
                };
                let value = self.registers[source_register as usize];
                self.registers[register as usize] = value << 1;
                self.registers[0xF] = (value & 0x80) >> 7;
            },
            Instruction::LoadAddress { address } => {
                self.address_register = address;
            },
            Instruction::Random { register, mask } => {
                let random_value: u8 = rand::random();
                self.registers[register as usize] = random_value & mask;
            },
            Instruction::SkipIfKeyDown { register } => {
                let value = self.registers[register as usize];
                if input.key_pressed(value) {
                    self.program_counter += 2;
                }
            },
            Instruction::SkipIfKeyUp { register } => {
                let value = self.registers[register as usize];
                if !input.key_pressed(value) {
                    self.program_counter += 2;
                }
            },
            Instruction::WaitForKeyDown { register } => {
                if let Some(key) = input.released_key() {
                    println!("Released");
                    self.registers[register as usize] = key;
                } else {
                    self.program_counter -= 2;
                    return CycleResult::Wait;
                }
            }
            Instruction::LoadDelayTimer { register } => {
                self.registers[register as usize] = self.delay_register;
            },
            Instruction::SetDelayTimer { register } => {
                self.delay_register = self.registers[register as usize];
            },
            Instruction::SetSoundTimer { register } => {
                self.sound_register = self.registers[register as usize];
            },
            Instruction::AddToAddress { register } => {
                let value_to_add = self.registers[register as usize];
                self.address_register += value_to_add as u16;
            },
            Instruction::WriteMemoryFromBinaryCodedDecimal { register } => {
                let value = self.registers[register as usize];
                self.memory[self.address_register as usize] = value / 100;
                self.memory[self.address_register as usize + 1] = (value % 100) / 10;
                self.memory[self.address_register as usize + 2] = value % 10;
            },
            Instruction::WriteMemory { end_register } => {
                for i in 0..end_register+1 {
                    self.memory[self.address_register as usize + i as usize] = self.registers[i as usize];
                }
                self.address_register += end_register as u16 + 1;
            },
            Instruction::ReadMemory { end_register } => {
                for i in 0..end_register+1 {
                    self.registers[i as usize] = self.memory[self.address_register as usize + i as usize];
                }
                self.address_register += end_register as u16 + 1;
            },
            Instruction::LoadDigitSpriteAddress { register } => {
                let digit = self.registers[register as usize];
                self.address_register = ((digit & 0x0F) * 5) as u16;
            },
            Instruction::DrawSprite { register_x, register_y, length } => {
                let x = self.registers[register_x as usize];
                let y = self.registers[register_y as usize];
                let sprite = self.memory[self.address_register as usize .. self.address_register as usize + length as usize].to_vec();
                let pixels_hidden = self.display.apply_sprite(x, y, sprite);
                self.registers[0xF] = pixels_hidden as u8;
                if self.settings.use_sprite_draw_delay {
                    return CycleResult::Wait;
                }
            },
            Instruction::SysCall { .. } | Instruction::Unknown { .. } => { }
        }

        return CycleResult::Continue;
    }

    fn decrement_timers(&mut self) {
        if self.delay_register > 0 {
            self.delay_register -= 1;
        }
        if self.sound_register > 0 {
            self.sound_register -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let program = vec![ 0x80, 0x14 ];
        let settings = EmulatorSettings::default();
        let mut emulator = Emulator::new(settings, program);
        emulator.registers[0x0] = 0x11;
        emulator.registers[0x1] = 0x10;
        let _ = emulator.cycle(&Default::default());
        assert_eq!(emulator.registers[0x0], 0x11 + 0x10);
        assert_eq!(emulator.registers[0xF], 0x00);
    }

    #[test]
    fn test_add_with_carry() {
        let program = vec![ 0x80, 0x14 ];
        let settings = EmulatorSettings::default();
        let mut emulator = Emulator::new(settings, program);
        emulator.registers[0x0] = 0xFF;
        emulator.registers[0x1] = 0x01;
        let _ = emulator.cycle(&Default::default());
        assert_eq!(emulator.registers[0x0], 0x00);
        assert_eq!(emulator.registers[0xF], 0x01);
    }

    #[test]
    fn test_subtract() {
        let program = vec![ 0x80, 0x15 ];
        let settings = EmulatorSettings::default();
        let mut emulator = Emulator::new(settings, program);
        emulator.registers[0x0] = 0x11;
        emulator.registers[0x1] = 0x10;
        let _ = emulator.cycle(&Default::default());
        assert_eq!(emulator.registers[0x0], 0x11 - 0x10);
        assert_eq!(emulator.registers[0xF], 0x01);
    }

    #[test]
    fn test_subtract_with_borrow() {
        let program = vec![ 0x80, 0x15 ];
        let settings = EmulatorSettings::default();
        let mut emulator = Emulator::new(settings, program);
        emulator.registers[0x0] = 0x10;
        emulator.registers[0x1] = 0x11;
        let _ = emulator.cycle(&Default::default());
        assert_eq!(emulator.registers[0x0], 0xFF);
        assert_eq!(emulator.registers[0xF], 0x00);
    }

    #[test]
    fn test_subtract_from() {
        let program = vec![ 0x80, 0x17 ];
        let settings = EmulatorSettings::default();
        let mut emulator = Emulator::new(settings, program);
        emulator.registers[0x0] = 0x10;
        emulator.registers[0x1] = 0x11;
        let _ = emulator.cycle(&Default::default());
        assert_eq!(emulator.registers[0x0], 0x11 - 0x10);
        assert_eq!(emulator.registers[0xF], 0x01);
    }

    #[test]
    fn test_subtract_from_with_borrow() {
        let program = vec![ 0x80, 0x17 ];
        let settings = EmulatorSettings::default();
        let mut emulator = Emulator::new(settings, program);
        emulator.registers[0x0] = 0x11;
        emulator.registers[0x1] = 0x10;
        let _ = emulator.cycle(&Default::default());
        assert_eq!(emulator.registers[0x0], 0xFF);
        assert_eq!(emulator.registers[0xF], 0x00);
    }

    #[test]
    fn test_binary_coded_decimal() {
        let program = vec![ 0xF0, 0x33 ];
        let settings = EmulatorSettings::default();
        let mut emulator = Emulator::new(settings, program);
        emulator.registers[0x0] = 123;
        emulator.address_register = 0x400;
        let _ = emulator.cycle(&Default::default());
        assert_eq!(emulator.memory[0x400], 1);
        assert_eq!(emulator.memory[0x401], 2);
        assert_eq!(emulator.memory[0x402], 3);
    }

    #[test]
    fn test_skip_if_value_skipped() {
        let program = vec![ 0x30, 0x11 ];
        let settings = EmulatorSettings::default();
        let mut emulator = Emulator::new(settings, program);
        emulator.registers[0x0] = 0x11;
        let _ = emulator.cycle(&Default::default());
        assert_eq!(emulator.program_counter, settings.program_start_address + 4);
    }

    #[test]
    fn test_skip_if_value_not_skipped() {
        let program = vec![ 0x30, 0x11 ];
        let settings = EmulatorSettings::default();
        let mut emulator = Emulator::new(settings, program);
        emulator.registers[0x0] = 0x10;
        let _ = emulator.cycle(&Default::default());
        assert_eq!(emulator.program_counter, settings.program_start_address + 2);
    }
}