use crate::register::*;

enum Instruction {
    SysCall { address: u16 },
    ClearScreen,
    Return,
    Jump { address: u16 },
    JumpWithOffset { address: u16 },
    Call { address: u16 },
    SkipIfEqual { register: u8, other_register: u8 },
    SkipIfNotEqual { register: u8, other_register: u8 },
    SkipIfValue { register: u8, value: u8 },
    SkipIfNotValue { register: u8, value: u8 },
    LoadValue { register: u8, value: u8 },
    AddValue { register: u8, value: u8 },
    Load { destination_register: u8, source_register: u8 },
    Or { destination_register: u8, or_register: u8 },
    And { destination_register: u8, and_register: u8 },
    Xor { destination_register: RegisterIndex, xor_register: u8 },
    Add { destination_register: u8, add_register: u8 },
    Subtract { destination_register: u8, subtract_register: u8 },
    SubtractFrom { destination_register: u8, subtract_register: u8 },
    ShiftLeft { destination_register: u8, shift_register: u8 },
    ShiftRight { destination_register: u8, shift_register: u8 },
    Random { register: u8, mask: u8 },
    DrawSprite { register_x: u8, register_y: u8, size: usize },
    SkipIfKeyDown { register: u8 },
    SkipIfKeyUp { register: u8 },
    WaitForKeyDown { register: u8 },
    LoadDelayTimer { register: u8 },
    SetDelayTimer { register: u8 },
    SetSoundTimer { register: u8 },
    LoadMemoryLocation { register: u8 },
    AddToMemoryLocation { register: u8 },
    LoadDigitSpriteMemoryLocation { register: u8 },
    WriteMemoryFromBinaryCodedDecimal { register: u8 },
    WriteMemory { register: u8 },
    ReadMemory { register: u8 },
}

struct Cpu {

}