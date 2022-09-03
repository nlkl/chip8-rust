use crate::display::Display;
use crate::keypad::Keypad;
use crate::settings::Settings;

const REGISTER_COUNT: usize = 16;
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

pub struct State {
    memory: Vec<u8>,
    registers: [u8; REGISTER_COUNT],
    stack: Vec<u16>,
    address_register: u16,
    pub program_counter: u16,
    pub delay_register: u8,
    pub sound_register: u8,
    pub display: Display,
    pub keypad: Keypad,
}

impl State {
    pub fn new(settings: Settings, program: Vec<u8>) -> Self {
        let mut state = Self {
            memory: vec![0x0; settings.memory_size as usize],
            registers: [0x0; REGISTER_COUNT],
            stack: vec![],
            address_register: 0,
            program_counter: settings.program_start_address,
            delay_register: 0,
            sound_register: 0,
            display: Display::new(settings.display_width, settings.display_height, settings.use_sprite_wrapping),
            keypad: Keypad::new(),
        };

        state.write_memory(settings.program_start_address, &program);
        state.write_memory(settings.sprite_start_address, &SPRITE_DATA);

        state
    }

    pub fn read_memory(&self, address: u16, size: u16) -> &[u8] {
        let address_range_end = address as usize + size as usize;
        assert!(address_range_end <= self.memory.len(), "Address range out of bounds. Start address: {}, end address: {}.", address, address_range_end - 1);
        &self.memory[address as usize .. address_range_end]
    }

    pub fn write_memory(&mut self, address: u16, data: &[u8]) {
        let address_range_end = address as usize + data.len();
        assert!(address_range_end <= self.memory.len(), "Address range out of bounds. Start address: {}, end address: {}.", address, address_range_end - 1);
        self.memory[address as usize .. address_range_end].copy_from_slice(data);
    }

    pub fn read_registers(&self, end_register: u8) -> &[u8] {
        &self.registers[0 .. end_register as usize + 1]
    }

    pub fn write_registers(&mut self, data: &[u8]) {
        assert!(data.len() <= REGISTER_COUNT, "Data exceeds register count. Data length: {}.", data.len());
        self.registers[0 .. data.len()].copy_from_slice(data);
    }

    pub fn register(&self, register: u8) -> u8 {
        assert!(register < REGISTER_COUNT as u8, "Invalid register: {}.", register);
        self.registers[register as usize]
    }

    pub fn set_register(&mut self, register: u8, value: u8) {
        assert!(register < REGISTER_COUNT as u8, "Invalid register: {}.", register);
        self.registers[register as usize] = value;
    }

    pub fn program_terminated(&self) -> bool {
        (self.program_counter as usize) >= self.memory.len()
    }

    pub fn increment_program_counter(&mut self) {
        self.program_counter += 2;
    }

    pub fn decrement_program_counter(&mut self) {
        self.program_counter -= 2;
    }

    pub fn address_register(&self) -> u16 {
        self.address_register
    }

    pub fn set_address_register(&mut self, address: u16) {
        assert!((address as usize) < self.memory.len(), "Address out of bounds. Address: {}.", address);
        self.address_register = address;
    }

    pub fn pop_return_address(&mut self) -> u16 {
        self.stack.pop().expect("Cannot pop return address as stack is empty.")
    }

    pub fn push_return_address(&mut self, address: u16) {
        assert!((address as usize) < self.memory.len(), "Address out of bounds. Address: {}.", address);
        self.stack.push(address);
    }

    pub fn decrement_delay_register(&mut self) {
        if self.delay_register > 0 {
            self.delay_register -= 1;
        }
    }

    pub fn decrement_sound_register(&mut self) {
        if self.sound_register > 0 {
            self.sound_register -= 1;
        }
    }
}