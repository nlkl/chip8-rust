const KEY_COUNT: usize = 16;

#[derive(Clone, Copy, PartialEq, Eq)]
enum KeyPressState {
    Pressed,
    Released,
    None,
}

#[derive(Clone, Copy)]
pub struct Keypad {
    keys: [KeyPressState; KEY_COUNT],
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            keys: [KeyPressState::None; KEY_COUNT],
        }
    }

    pub fn release_all_keys(&mut self) {
        for key in 0..KEY_COUNT as u8 {
            if self.key_pressed(key) {
                self.keys[key as usize] = KeyPressState::Released;
            } else if self.key_released(key) {
                self.keys[key as usize] = KeyPressState::None;
            }
        }
    }

    pub fn set_key_pressed(&mut self, key: u8) {
        assert!((key as usize) < KEY_COUNT, "Key out of range. Key: {}.", key);
        self.keys[key as usize] = KeyPressState::Pressed;
    }

    pub fn set_key_released(&mut self, key: u8) {
        assert!((key as usize) < KEY_COUNT, "Key out of range. Key: {}.", key);
        self.keys[key as usize] = KeyPressState::Released;
    }

    pub fn key_pressed(&self, key: u8) -> bool {
        assert!((key as usize) < KEY_COUNT, "Key out of range. Key: {}.", key);
        self.keys[key as usize] == KeyPressState::Pressed
    }

    pub fn key_released(&self, key: u8) -> bool {
        assert!((key as usize) < KEY_COUNT, "Key out of range. Key: {}.", key);
        self.keys[key as usize] == KeyPressState::Released
    }

    pub fn pressed_keys(&self) -> Vec<u8> {
        let mut keys = vec![];
        for key in 0..KEY_COUNT as u8 {
            if self.key_pressed(key) {
                keys.push(key);
            }
        }
        keys
    }

    pub fn released_keys(&self) -> Vec<u8> {
        let mut keys = vec![];
        for key in 0..KEY_COUNT as u8 {
            if self.key_released(key) {
                keys.push(key);
            }
        }
        keys
    }
}