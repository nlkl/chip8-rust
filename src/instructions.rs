#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    /// 00E0 - Clear the screen.
    ClearScreen,
    /// 00EE - Return from subroutine.
    Return,
    /// 0NNN - Execute system subroutine (ignored).
    SysCall { address: u16 },
    /// 1NNN - Jump to address NNN.
    Jump { address: u16 },
    /// BNNN - Jump to address NNN + V0.
    JumpWithOffset { address: u16 },
    /// 2NNN - Call subroutine at address NNN.
    Call { address: u16 },
    /// 5XY0 - Skip next instruction if the value in register VX equals the value in register VY.
    SkipIfEqual { register: u8, comparand_register: u8 },
    /// 9XY0 - Skip next instruction if the value in register VX does not equal the value in register VY.
    SkipIfNotEqual { register: u8, comparand_register: u8 },
    /// 3XNN - Skip next instruction if the value in register VX equals NN.
    SkipIfValue { register: u8, comparand_value: u8 },
    /// 4XNN - Skip next instruction if the value in register VX does not equal NN.
    SkipIfNotValue { register: u8, comparand_value: u8 },
    /// 6XNN - Load NN into register VX.
    LoadValue { register: u8, value: u8 },
    /// 7XNN - Add NN to register VX.
    AddValue { register: u8, value: u8 },
    /// 8XY0 - Load the value in register VY into register VX.
    Load { register: u8, from_register: u8 },
    /// 8XY1 - Set register VX to VX OR VY.
    Or { register: u8, or_register: u8 },
    /// 8XY2 - Set register VX to VX AND VY.
    And { register: u8, and_register: u8 },
    /// 8XY3 - Set register VX to VX XOR VY.
    Xor { register: u8, xor_register: u8 },
    /// 8XY4 - Set register VX to VX + VY.
    Add { register: u8, add_register: u8 },
    /// 8XY5 - Set register VX to VX - VY.
    Subtract { register: u8, subtract_register: u8 },
    /// 8XY7 - Set register VX to VY - VX.
    SubtractFrom { register: u8, subtract_from_register: u8 },
    /// 8XY6 - Set register VX to VY >> 1.
    ShiftRight { register: u8, source_register: u8 },
    /// 8XYE - Set register VX to VY << 1.
    ShiftLeft { register: u8, source_register: u8 },
    /// CXNN - Set register VX to a random number with a mask of NN.
    Random { register: u8, mask: u8 },
    /// DXYN - Draw sprite at position (VX, VY). The sprite consists of N bytes starting from the address in I.
    DrawSprite { register_x: u8, register_y: u8, length: u8 },
    /// EX9E - Skip next instruction if the key corresponding to the value in register VX is pressed.
    SkipIfKeyDown { register: u8 },
    /// EXA1 - Skip next instruction if the key corresponding to the value in register VX is not pressed.
    SkipIfKeyUp { register: u8 },
    /// FX0A - Wait for a keypress and store the result in register VX.
    WaitForKeyDown { register: u8 },
    /// FX07 - Load the current value of the delay timer into register VX.
    LoadDelayTimer { register: u8 },
    /// FX15 - Set the delay timer to the value in register VX.
    SetDelayTimer { register: u8 },
    /// FX18 - Set the sound timer to the value in register VX.
    SetSoundTimer { register: u8 },
    /// ANNN - Load address NNN into I.
    LoadAddress { address: u16 },
    /// FX1E - Add the value in register VX to I.
    AddToAddress { register: u8 },
    /// FX29 - Load the address of the sprite corresponding to the value stored in register VX into I.
    LoadDigitSpriteAddress { register: u8 },
    /// FX33 - Write the value in register VX as a binary-coded decimal into memory starting at the address in I.
    WriteMemoryFromBinaryCodedDecimal { register: u8 },
    /// FX55 - Write the values in registers V0 - VX into memory starting at the address in I.
    WriteMemory { end_register: u8 },
    /// FX65 - Read memory starting at the address in I into registers V0 - VX.
    ReadMemory { end_register: u8 },
    /// Unknown / unsupported instruction.
    Unknown { instruction: u16 },
}

impl Instruction {
    pub fn decode(instruction: u16) -> Self {
        let n0 = ((instruction & 0xF000) >> 12) as u8;
        let n1 = ((instruction & 0x0F00) >> 8) as u8;
        let n2 = ((instruction & 0x00F0) >> 4) as u8;
        let n3 = (instruction & 0x000F) as u8;
        match (n0, n1, n2, n3) {
           (0x0, 0x0, 0xE, 0x0) => Self::ClearScreen,
           (0x0, 0x0, 0xE, 0xE) => Self::Return,
           (0x0,   _,   _,   _) => Self::SysCall { address: instruction },
           (0x1,   _,   _,   _) => Self::Jump { address: instruction & 0x0FFF },
           (0x2,   _,   _,   _) => Self::Call { address: instruction & 0x0FFF },
           (0x3,   _,   _,   _) => Self::SkipIfValue { register: n1, comparand_value: n2 << 4 | n3 },
           (0x4,   _,   _,   _) => Self::SkipIfNotValue { register: n1, comparand_value: n2 << 4 | n3 },
           (0x5,   _,   _, 0x0) => Self::SkipIfEqual { register: n1, comparand_register: n2 },
           (0x6,   _,   _,   _) => Self::LoadValue { register: n1, value: n2 << 4 | n3 },
           (0x7,   _,   _,   _) => Self::AddValue { register: n1, value: n2 << 4 | n3 },
           (0x8,   _,   _, 0x0) => Self::Load { register: n1, from_register: n2 },
           (0x8,   _,   _, 0x1) => Self::Or { register: n1, or_register: n2 },
           (0x8,   _,   _, 0x2) => Self::And { register: n1, and_register: n2 },
           (0x8,   _,   _, 0x3) => Self::Xor { register: n1, xor_register: n2 },
           (0x8,   _,   _, 0x4) => Self::Add { register: n1, add_register: n2 },
           (0x8,   _,   _, 0x5) => Self::Subtract { register: n1, subtract_register: n2 },
           (0x8,   _,   _, 0x6) => Self::ShiftRight { register: n1, source_register: n2 },
           (0x8,   _,   _, 0x7) => Self::SubtractFrom { register: n1, subtract_from_register: n2 },
           (0x8,   _,   _, 0xE) => Self::ShiftLeft { register: n1, source_register: n2 },
           (0x9,   _,   _, 0x0) => Self::SkipIfNotEqual { register: n1, comparand_register: n2 },
           (0xA,   _,   _,   _) => Self::LoadAddress { address: instruction & 0x0FFF },
           (0xB,   _,   _,   _) => Self::JumpWithOffset { address: instruction & 0x0FFF },
           (0xC,   _,   _,   _) => Self::Random { register: n1, mask: n2 << 4 | n3 },
           (0xD,   _,   _,   _) => Self::DrawSprite { register_x: n1, register_y: n2, length: n3 },
           (0xE,   _, 0x9, 0xE) => Self::SkipIfKeyDown { register: n1 },
           (0xE,   _, 0xA, 0x1) => Self::SkipIfKeyUp { register: n1 },
           (0xF,   _, 0x0, 0x7) => Self::LoadDelayTimer { register: n1 },
           (0xF,   _, 0x0, 0xA) => Self::WaitForKeyDown { register: n1 },
           (0xF,   _, 0x1, 0x5) => Self::SetDelayTimer { register: n1 },
           (0xF,   _, 0x1, 0x8) => Self::SetSoundTimer { register: n1 },
           (0xF,   _, 0x1, 0xE) => Self::AddToAddress { register: n1 },
           (0xF,   _, 0x2, 0x9) => Self::LoadDigitSpriteAddress { register: n1 },
           (0xF,   _, 0x3, 0x3) => Self::WriteMemoryFromBinaryCodedDecimal { register: n1 },
           (0xF,   _, 0x5, 0x5) => Self::WriteMemory { end_register: n1 },
           (0xF,   _, 0x6, 0x5) => Self::ReadMemory { end_register: n1 },
           _                    => Self::Unknown { instruction }
        }
    }

    #[cfg(test)]
    pub fn encode(self) -> u16 {
        fn concat(n0: u8, n1: u8, n2: u8, n3: u8) -> u16 {
            (n0 as u16) << 12 | (n1 as u16) << 8 | (n2 as u16) << 4 | n3 as u16
        }
        match self {
            Self::ClearScreen                                       => 0x00E0,
            Self::Return                                            => 0x00EE,
            Self::SysCall { address }                               => address,
            Self::Jump { address }                                  => 0x1000 | address,
            Self::Call { address }                                  => 0x2000 | address,
            Self::SkipIfValue { register, comparand_value }         => concat(0x3, register, 0, comparand_value),
            Self::SkipIfNotValue { register, comparand_value }      => concat(0x4, register, 0, comparand_value),
            Self::SkipIfEqual { register, comparand_register }      => concat(0x5, register, comparand_register, 0x0),
            Self::LoadValue { register, value }                     => concat(0x6, register, 0, value),
            Self::AddValue { register, value }                      => concat(0x7, register, 0, value),
            Self::Load { register, from_register }                  => concat(0x8, register, from_register, 0x0),
            Self::Or { register, or_register }                      => concat(0x8, register, or_register, 0x1),
            Self::And { register, and_register }                    => concat(0x8, register, and_register, 0x2),
            Self::Xor { register, xor_register }                    => concat(0x8, register, xor_register, 0x3),
            Self::Add { register, add_register }                    => concat(0x8, register, add_register, 0x4),
            Self::Subtract { register, subtract_register }          => concat(0x8, register, subtract_register, 0x5),
            Self::ShiftRight { register, source_register }          => concat(0x8, register, source_register, 0x6),
            Self::SubtractFrom { register, subtract_from_register } => concat(0x8, register, subtract_from_register, 0x7),
            Self::ShiftLeft { register, source_register }           => concat(0x8, register, source_register, 0xE),
            Self::SkipIfNotEqual { register, comparand_register }   => concat(0x9, register, comparand_register, 0x0),
            Self::LoadAddress { address }                           => 0xA000 | address,
            Self::JumpWithOffset { address }                        => 0xB000 | address,
            Self::Random { register, mask }                         => concat(0xC, register, 0, mask),
            Self::DrawSprite { register_x, register_y, length }     => concat(0xD, register_x, register_y, length),
            Self::SkipIfKeyDown { register }                        => concat(0xE, register, 0x9, 0xE),
            Self::SkipIfKeyUp { register }                          => concat(0xE, register, 0xA, 0x1),
            Self::LoadDelayTimer { register }                       => concat(0xF, register, 0x0, 0x7),
            Self::WaitForKeyDown { register }                       => concat(0xF, register, 0x0, 0xA),
            Self::SetDelayTimer { register }                        => concat(0xF, register, 0x1, 0x5),
            Self::SetSoundTimer { register }                        => concat(0xF, register, 0x1, 0x8),
            Self::AddToAddress { register }                         => concat(0xF, register, 0x1, 0xE),
            Self::LoadDigitSpriteAddress { register }               => concat(0xF, register, 0x2, 0x9),
            Self::WriteMemoryFromBinaryCodedDecimal { register }    => concat(0xF, register, 0x3, 0x3),
            Self::WriteMemory { end_register }                      => concat(0xF, end_register, 0x5, 0x5),
            Self::ReadMemory { end_register }                       => concat(0xF, end_register, 0x6, 0x5),
            Self::Unknown { instruction }                           => instruction
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_instructions() -> Vec<(u16, Instruction)> {
        vec![
            (0x00E0u16, Instruction::ClearScreen),
            (0x00EEu16, Instruction::Return),
            (0x0123u16, Instruction::SysCall { address: 0x123 }),
            (0x1123u16, Instruction::Jump { address: 0x123 }),
            (0x2123u16, Instruction::Call { address: 0x123 }),
            (0x3123u16, Instruction::SkipIfValue { register: 0x1, comparand_value: 0x23 }),
            (0x4123u16, Instruction::SkipIfNotValue { register: 0x1, comparand_value: 0x23 }),
            (0x5120u16, Instruction::SkipIfEqual { register: 0x1, comparand_register: 0x2 }),
            (0x6123u16, Instruction::LoadValue { register: 0x1, value: 0x23 }),
            (0x7123u16, Instruction::AddValue { register: 0x1, value: 0x23 }),
            (0x8120u16, Instruction::Load { register: 0x1, from_register: 0x2 }),
            (0x8121u16, Instruction::Or { register: 0x1, or_register: 0x2 }),
            (0x8122u16, Instruction::And { register: 0x1, and_register: 0x2 }),
            (0x8123u16, Instruction::Xor { register: 0x1, xor_register: 0x2 }),
            (0x8124u16, Instruction::Add { register: 0x1, add_register: 0x2 }),
            (0x8125u16, Instruction::Subtract { register: 0x1, subtract_register: 0x2 }),
            (0x8126u16, Instruction::ShiftRight { register: 0x1, source_register: 0x2 }),
            (0x8127u16, Instruction::SubtractFrom { register: 0x1, subtract_from_register: 0x2 }),
            (0x812Eu16, Instruction::ShiftLeft { register: 0x1, source_register: 0x2 }),
            (0x9120u16, Instruction::SkipIfNotEqual { register: 0x1, comparand_register: 0x2 }),
            (0xA123u16, Instruction::LoadAddress { address: 0x123 }),
            (0xB123u16, Instruction::JumpWithOffset { address: 0x123 }),
            (0xC123u16, Instruction::Random { register: 0x1, mask: 0x23 }),
            (0xD123u16, Instruction::DrawSprite { register_x: 0x1, register_y: 0x2, length: 0x3 }),
            (0xE19Eu16, Instruction::SkipIfKeyDown { register: 0x1 }),
            (0xE1A1u16, Instruction::SkipIfKeyUp { register: 0x1 }),
            (0xF107u16, Instruction::LoadDelayTimer { register: 0x1 }),
            (0xF10Au16, Instruction::WaitForKeyDown { register: 0x1 }),
            (0xF115u16, Instruction::SetDelayTimer { register: 0x1 }),
            (0xF118u16, Instruction::SetSoundTimer { register: 0x1 }),
            (0xF11Eu16, Instruction::AddToAddress { register: 0x1 }),
            (0xF129u16, Instruction::LoadDigitSpriteAddress { register: 0x1 }),
            (0xF133u16, Instruction::WriteMemoryFromBinaryCodedDecimal { register: 0x1 }),
            (0xF155u16, Instruction::WriteMemory { end_register: 0x1 }),
            (0xF165u16, Instruction::ReadMemory { end_register: 0x1 }),
            (0x9BCDu16, Instruction::Unknown { instruction: 0x9BCD }),
        ]
    }

    #[test]
    fn can_decode_instruction() {
        for (encoded, decoded) in generate_instructions() {
            assert_eq!(Instruction::decode(encoded), decoded);
        }
    }

    #[test]
    fn can_encode_instruction() {
        for (encoded, decoded) in generate_instructions() {
            assert_eq!(encoded, decoded.encode());
        }
    }
}