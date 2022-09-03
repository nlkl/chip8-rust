use crate::display::Display;
use crate::keypad::Keypad;

const REGISTER_COUNT: usize = 16;

pub struct State {
    memory: Vec<u8>,
    registers: [u8; REGISTER_COUNT],
    stack: Vec<u16>,
    program_counter: u16,
    address_register: u16,
    pub delay_register: u8,
    pub sound_register: u8,
    pub display: Display,
    pub keypad: Keypad,
}

impl State {
    pub fn new(memory_size: u16, display: Display, keypad: Keypad) -> Self {
        Self {
            memory: vec![0x0; memory_size as usize],
            registers: [0x0; REGISTER_COUNT],
            stack: vec![],
            program_counter: 0,
            address_register: 0,
            delay_register: 0,
            sound_register: 0,
            display: display,
            keypad: keypad,
        }
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

    pub fn program_counter(&self) -> u16 {
        self.program_counter
    }

    pub fn set_program_counter(&mut self, address: u16) {
        assert!((address as usize) < self.memory.len(), "Address out of bounds. Address: {}.", address);
        self.program_counter = address;
    }

    pub fn increment_program_counter(&mut self) {
        self.set_program_counter(self.program_counter + 2);
    }

    pub fn decrement_program_counter(&mut self) {
        self.set_program_counter(self.program_counter - 2);
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