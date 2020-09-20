use crate::display::Display;

const MEMORY_LOCATION_COUNT: usize = 4096;
const REGISTER_COUNT: usize = 0xF;

pub struct System {
    display: Display,
    memory: [u8; MEMORY_LOCATION_COUNT],
    registers: [u8; REGISTER_COUNT],
    memory_register: u16,
    stack: Vec<u16>
}

impl System {
    pub fn new() -> System {
        System {
            display: Display::new(),
            memory: [0; MEMORY_LOCATION_COUNT],
            registers: [0; REGISTER_COUNT],
            memory_register: 0,
            stack: vec![]
        }
    }
}