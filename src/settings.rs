#[derive(Clone, Copy)]
pub struct Settings {
    /// Clock speed in Hz.
    pub frame_rate: u16,
    /// Frame rate in Hz.
    pub clock_speed: u16,
    /// The memory address at which programs start.
    pub program_start_address: u16,
    /// The memory size in bytes.
    pub memory_size: u16,
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

impl Default for Settings {
    fn default() -> Self { 
        Self {
            frame_rate: 60,
            clock_speed: 500,
            program_start_address: 0x200,
            memory_size: 0x1000,
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