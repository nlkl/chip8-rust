use crate::instructions::Instruction;
use crate::settings::Settings;
use crate::state::State;

pub enum CpuCycleResult {
    Continue,
    Wait,
    Done,
}

pub struct Cpu {
    pub settings: Settings,
}

impl Cpu {
    pub fn cycle(&self, state: &mut State) -> CpuCycleResult {
        if state.program_counter() > (self.settings.memory_size - 2)  {
            return CpuCycleResult::Done;
        }

        let instruction_bytes = state.read_memory(state.program_counter(), 2);
        let instruction = Instruction::decode((instruction_bytes[0] as u16) << 8 | instruction_bytes[1] as u16);
        state.increment_program_counter(); // TODO: Will fail at end

        match instruction {
            Instruction::ClearScreen => {
                state.display.clear();
            },
            Instruction::Return => {
                let return_address = state.pop_return_address();
                state.set_program_counter(return_address);
            },
            Instruction::Jump { address } => {
                state.set_program_counter(address);
            },
            Instruction::JumpWithOffset { address } => {
                let offset = if self.settings.use_flexible_jump_offset {
                    let register = (address & 0xF00) >> 8;
                    state.register(register as u8)
                } else {
                    state.register(0x0)
                };
                state.set_program_counter(address + offset as u16);
            },
            Instruction::Call { address } => {
                state.push_return_address(state.program_counter());
                state.set_program_counter(address);
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
            },
            Instruction::And { register, and_register } => {
                let value = state.register(register) & state.register(and_register);
                state.set_register(register, value);
            },
            Instruction::Xor { register, xor_register } => {
                let value = state.register(register) ^ state.register(xor_register);
                state.set_register(register, value);
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
                state.set_address_register(address + end_register as u16 + 1);
            },
            Instruction::ReadMemory { end_register } => {
                let address = state.address_register();
                let data = state.read_memory(address, end_register as u16 + 1).to_vec();
                state.write_registers(&data);
                state.set_address_register(address + end_register as u16 + 1);
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