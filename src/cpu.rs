use crate::instructions::Instruction;
use crate::settings::Settings;
use crate::state::State;

pub enum CpuCycleResult {
    Continue,
    Wait,
    Done,
}

pub struct Cpu {
    settings: Settings,
}

impl Cpu {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings: settings,
        }
    }

    pub fn cycle(&self, state: &mut State) -> CpuCycleResult {
        if state.program_terminated() {
            return CpuCycleResult::Done;
        }

        let instruction_bytes = state.read_memory(state.program_counter, 2);
        let instruction = Instruction::decode((instruction_bytes[0] as u16) << 8 | instruction_bytes[1] as u16);
        state.increment_program_counter();

        match instruction {
            Instruction::ClearScreen => {
                state.display.clear();
            },
            Instruction::Return => {
                let return_address = state.pop_return_address();
                state.program_counter = return_address;
            },
            Instruction::Jump { address } => {
                state.program_counter = address;
            },
            Instruction::JumpWithOffset { address } => {
                let offset = if self.settings.use_flexible_jump_offset {
                    let register = (address & 0xF00) >> 8;
                    state.register(register as u8)
                } else {
                    state.register(0x0)
                };
                state.program_counter = address + offset as u16;
            },
            Instruction::Call { address } => {
                state.push_return_address(state.program_counter);
                state.program_counter = address;
            },
            Instruction::SkipIfValue { register, comparand_value } => {
                let value = state.register(register);
                if value == comparand_value {
                    state.increment_program_counter()
                }
            },
            Instruction::SkipIfNotValue { register, comparand_value } => {
                let value = state.register(register);
                if value != comparand_value {
                    state.increment_program_counter();
                }
            },
            Instruction::SkipIfEqual { register, comparand_register } => {
                let value = state.register(register);
                let comparand_value = state.register(comparand_register);
                if value == comparand_value {
                    state.increment_program_counter();
                }
            },
            Instruction::SkipIfNotEqual { register, comparand_register } => {
                let value = state.register(register);
                let comparand_value = state.register(comparand_register);
                if value != comparand_value {
                    state.increment_program_counter();
                }
            },
            Instruction::LoadValue { register, value } => {
                state.set_register(register, value);
            },
            Instruction::AddValue { register, value } => {
                let current_value = state.register(register) as u16;
                let result = current_value + value as u16;
                state.set_register(register, (result & 0x00FF) as u8);
            },
            Instruction::Load { register, from_register } => {
                state.set_register(register, state.register(from_register));
            },
            Instruction::Or { register, or_register } => {
                let value = state.register(register) | state.register(or_register);
                state.set_register(register, value);
                if self.settings.use_flag_reset_on_logic_ops {
                    state.set_register(0xF, 0);
                }
            },
            Instruction::And { register, and_register } => {
                let value = state.register(register) & state.register(and_register);
                state.set_register(register, value);
                if self.settings.use_flag_reset_on_logic_ops {
                    state.set_register(0xF, 0);
                }
            },
            Instruction::Xor { register, xor_register } => {
                let value = state.register(register) ^ state.register(xor_register);
                state.set_register(register, value);
                if self.settings.use_flag_reset_on_logic_ops {
                    state.set_register(0xF, 0);
                }
            },
            Instruction::Add { register, add_register } => {
                let value = state.register(register) as u16;
                let value_to_add = state.register(add_register) as u16;
                let result = value + value_to_add;
                state.set_register(register, (result & 0x00FF) as u8);
                state.set_register(0xF, (result > 0xFF) as u8);
            },
            Instruction::Subtract { register, subtract_register } => {
                let value = state.register(register) as u16;
                let value_to_subtract = state.register(subtract_register) as u16;
                let result = 0x0100 + value - value_to_subtract;
                state.set_register(register, (result & 0x00FF) as u8);
                state.set_register(0xF, (value_to_subtract <= value) as u8);
            },
            Instruction::SubtractFrom { register, subtract_from_register } => {
                let value_to_subtract = state.register(register) as u16;
                let value = state.register(subtract_from_register) as u16;
                let result = 0x0100 + value - value_to_subtract;
                state.set_register(register, (result & 0x00FF) as u8);
                state.set_register(0xF, (value_to_subtract <= value) as u8);
            },
            Instruction::ShiftRight { register, source_register } => {
                let source_register = if self.settings.use_in_place_shift {
                    register
                } else {
                    source_register
                };
                let value = state.register(source_register);
                state.set_register(register, value >> 1);
                state.set_register(0xF, value & 0x01);
            },
            Instruction::ShiftLeft { register, source_register } => {
                let source_register = if self.settings.use_in_place_shift {
                    register
                } else {
                    source_register
                };
                let value = state.register(source_register);
                state.set_register(register, value << 1);
                state.set_register(0xF, (value & 0x80) >> 7);
            },
            Instruction::LoadAddress { address } => {
                state.set_address_register(address);
            },
            Instruction::Random { register, mask } => {
                let random_value: u8 = rand::random();
                state.set_register(register, random_value & mask);
            },
            Instruction::SkipIfKeyDown { register } => {
                let value = state.register(register);
                if state.keypad.key_pressed(value) {
                    state.increment_program_counter();
                }
            },
            Instruction::SkipIfKeyUp { register } => {
                let value = state.register(register);
                if !state.keypad.key_pressed(value) {
                    state.increment_program_counter();
                }
            },
            Instruction::WaitForKeyDown { register } => {
                if let Some(key) = state.keypad.released_keys().first() {
                    state.set_register(register, *key);
                } else {
                    state.decrement_program_counter();
                    return CpuCycleResult::Wait;
                }
            }
            Instruction::LoadDelayTimer { register } => {
                state.set_register(register, state.delay_register);
            },
            Instruction::SetDelayTimer { register } => {
                state.delay_register = state.register(register);
            },
            Instruction::SetSoundTimer { register } => {
                state.sound_register = state.register(register);
            },
            Instruction::AddToAddress { register } => {
                let value_to_add = state.register(register) as u16;
                let address = state.address_register() + value_to_add;
                state.set_address_register(address);
            },
            Instruction::WriteMemoryFromBinaryCodedDecimal { register } => {
                let value = state.register(register);
                let bcd_data = vec![value / 100, (value % 100) / 10, value % 10];
                state.write_memory(state.address_register(), &bcd_data)
            },
            Instruction::WriteMemory { end_register } => {
                let address = state.address_register();
                let data = state.read_registers(end_register).to_vec();
                state.write_memory(address, &data);
                if self.settings.use_auto_address_increments {
                    state.set_address_register(address + end_register as u16 + 1);
                }
            },
            Instruction::ReadMemory { end_register } => {
                let address = state.address_register();
                let data = state.read_memory(address, end_register as u16 + 1).to_vec();
                state.write_registers(&data);
                if self.settings.use_auto_address_increments {
                    state.set_address_register(address + end_register as u16 + 1);
                }
            },
            Instruction::LoadDigitSpriteAddress { register } => {
                let digit = state.register(register) as u16;
                state.set_address_register(self.settings.sprite_start_address + (digit & 0x0F) * 5);
            },
            Instruction::DrawSprite { register_x, register_y, length } => {
                let x = state.register(register_x);
                let y = state.register(register_y);
                let sprite = state.read_memory(state.address_register(), length as u16).to_vec();
                let pixels_hidden = state.display.apply_sprite(x, y, &sprite);
                state.set_register(0xF, pixels_hidden as u8);
                if self.settings.use_sprite_draw_delay {
                    return CpuCycleResult::Wait;
                }
            },
            Instruction::SysCall { .. } | Instruction::Unknown { .. } => { }
        }

        return CpuCycleResult::Continue;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(instructions: Vec<Instruction>) -> (Cpu, State, Settings) {
        let mut program = vec![];
        for instruction in instructions {
            let instruction_bytes = instruction.encode();
            program.push(((instruction_bytes & 0xFF00) >> 8) as u8);
            program.push((instruction_bytes & 0x00FF) as u8);
        }
        let settings = Settings::default();
        let cpu = Cpu::new(settings);
        let state = State::new(settings, program);
        (cpu, state, settings)
    }

    #[test]
    fn test_add() {
        let program = vec![ Instruction::Add { register: 0x0, add_register: 0x1 } ];
        let (cpu, mut state, _) = setup(program);
        state.set_register(0x0, 0x11);
        state.set_register(0x1, 0x10);
        let _ = cpu.cycle(&mut state);
        assert_eq!(state.register(0x0), 0x11 + 0x10);
        assert_eq!(state.register(0xF), 0x00);
    }

    #[test]
    fn test_add_with_carry() {
        let program = vec![ Instruction::Add { register: 0x0, add_register: 0x1 } ];
        let (cpu, mut state, _) = setup(program);
        state.set_register(0x0, 0xFF);
        state.set_register(0x1, 0x01);
        let _ = cpu.cycle(&mut state);
        assert_eq!(state.register(0x0), 0x00);
        assert_eq!(state.register(0xF), 0x01);
    }

    #[test]
    fn test_subtract() {
        let program = vec![ Instruction::Subtract { register: 0x0, subtract_register: 0x1 } ];
        let (cpu, mut state, _) = setup(program);
        state.set_register(0x0, 0x11);
        state.set_register(0x1, 0x10);
        let _ = cpu.cycle(&mut state);
        assert_eq!(state.register(0x0), 0x11 - 0x10);
        assert_eq!(state.register(0xF), 0x01);
    }

    #[test]
    fn test_subtract_with_borrow() {
        let program = vec![ Instruction::Subtract { register: 0x0, subtract_register: 0x1 } ];
        let (cpu, mut state, _) = setup(program);
        state.set_register(0x0, 0x10);
        state.set_register(0x1, 0x11);
        let _ = cpu.cycle(&mut state);
        assert_eq!(state.register(0x0), 0xFF);
        assert_eq!(state.register(0xF), 0x00);
    }

    #[test]
    fn test_subtract_from() {
        let program = vec![ Instruction::SubtractFrom { register: 0x0, subtract_from_register: 0x1 } ];
        let (cpu, mut state, _) = setup(program);
        state.set_register(0x0, 0x10);
        state.set_register(0x1, 0x11);
        let _ = cpu.cycle(&mut state);
        assert_eq!(state.register(0x0), 0x11 - 0x10);
        assert_eq!(state.register(0xF), 0x01);
    }

    #[test]
    fn test_subtract_from_with_borrow() {
        let program = vec![ Instruction::SubtractFrom { register: 0x0, subtract_from_register: 0x1 } ];
        let (cpu, mut state, _) = setup(program);
        state.set_register(0x0, 0x11);
        state.set_register(0x1, 0x10);
        let _ = cpu.cycle(&mut state);
        assert_eq!(state.register(0x0), 0xFF);
        assert_eq!(state.register(0xF), 0x00);
    }

    #[test]
    fn test_binary_coded_decimal() {
        let program = vec![ Instruction::WriteMemoryFromBinaryCodedDecimal { register: 0x0 } ];
        let (cpu, mut state, _) = setup(program);
        state.set_register(0x0, 123);
        state.set_address_register(0x0400);
        let _ = cpu.cycle(&mut state);
        assert_eq!(state.read_memory(0x0400, 3), &[1, 2, 3]);
    }

    #[test]
    fn test_skip_if_value_skipped() {
        let program = vec![ Instruction::SkipIfValue { register: 0x0, comparand_value: 0x11 } ];
        let (cpu, mut state, settings) = setup(program);
        state.set_register(0x0, 0x11);
        let _ = cpu.cycle(&mut state);
        assert_eq!(state.program_counter, settings.program_start_address + 4);
    }

    #[test]
    fn test_skip_if_value_not_skipped() {
        let program = vec![ Instruction::SkipIfValue { register: 0x0, comparand_value: 0x11 } ];
        let (cpu, mut state, settings) = setup(program);
        state.set_register(0x0, 0x10);
        let _ = cpu.cycle(&mut state);
        assert_eq!(state.program_counter, settings.program_start_address + 2);
    }
}