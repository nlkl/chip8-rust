use std::ops::{Index, IndexMut, Range};

const REGISTER_COUNT: usize = 16;

pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new(size: u16) -> Memory {
        Memory {
            data: vec![0x0; size as usize]
        }
    }

    pub fn size(&self) -> u16 {
        self.data.len() as u16
    }
}

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        let index = index as usize;
        assert!(index < self.data.len(), "Index out bounds. Index: {}, memory size in bytes: {}.", index, self.data.len());
        &self.data[index]
    }
}

impl Index<Range<u16>> for Memory {
    type Output = [u8];

    fn index(&self, index: Range<u16>) -> &Self::Output {
        let range = Range { start: index.start as usize, end: index.end as usize };
        assert!(range.end < self.data.len(), "Index out bounds. Range end: {}, memory size in bytes: {}.", range.end, self.data.len());
        &self.data[range]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        let index = index as usize;
        assert!(index < self.data.len(), "Index out bounds. Index: {}, memory size in bytes: {}.", index, self.data.len());
        &mut self.data[index]
    }
}

pub struct Registers {
    data: [u8; REGISTER_COUNT],
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            data: [0x0; REGISTER_COUNT]
        }
    }
}

impl Index<u8> for Registers {
    type Output = u8;

    fn index(&self, index: u8) -> &Self::Output {
        let index = index as usize;
        assert!(index < REGISTER_COUNT, "Index out bounds. Index: {}, register count: {}.", index, REGISTER_COUNT);
        &self.data[index]
    }
}

impl IndexMut<u8> for Registers {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        let index = index as usize;
        assert!(index < REGISTER_COUNT, "Index out bounds. Index: {}, register count: {}.", index, REGISTER_COUNT);
        &mut self.data[index]
    }
}