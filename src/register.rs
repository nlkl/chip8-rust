pub struct RegisterIndex {
    index: usize
}

impl RegisterIndex {
    pub fn new(index: usize) -> RegisterIndex {
        if index > 0xF {
            panic!("Register index must be between 0x0 and 0xF.");
        }

        RegisterIndex { index: index }
    }
}

impl Into<usize> for RegisterIndex {
    fn into(self) -> usize {
        self.index
    }
}