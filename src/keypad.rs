const KEY_COUNT: usize = 16;

#[derive(Clone, Copy, PartialEq, Eq)]
enum KeyPressState {
    None,
    Pressed,
    Released,
}

#[derive(Clone)]
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
            self.keys[key as usize] = match self.keys[key as usize] {
                KeyPressState::Pressed => KeyPressState::Released,
                KeyPressState::Released => KeyPressState::None,
                KeyPressState::None => KeyPressState::None
            };
        }
    }

    pub fn set_key_pressed(&mut self, key: u8) {
        assert!((key as usize) < KEY_COUNT, "Key out of range. Key: {}.", key);
        self.keys[key as usize] = KeyPressState::Pressed;
    }

    pub fn key_pressed(&self, key: u8) -> bool {
        assert!((key as usize) < KEY_COUNT, "Key out of range. Key: {}.", key);
        self.keys[key as usize] == KeyPressState::Pressed
    }

    pub fn key_released(&self, key: u8) -> bool {
        assert!((key as usize) < KEY_COUNT, "Key out of range. Key: {}.", key);
        self.keys[key as usize] == KeyPressState::Released
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