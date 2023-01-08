use std::ops::{Index, IndexMut, Range};

#[derive(Debug)]
pub struct Ram {
    buf: [u8; 4096],
}

impl Default for Ram {
    fn default() -> Self {
        Self { buf: [0; 4096] }
    }
}

impl Ram {
    pub fn new() -> Self {
        let mut ram = Self::default();
        ram.load(FONTSET_ADDR, &FONTSET);
        ram
    }

    pub fn load(&mut self, addr: u16, src: &[u8]) {
        let addr = addr as usize;
        self.buf[addr..addr + src.len()].copy_from_slice(src);
    }
}

impl Index<u16> for Ram {
    type Output = u8;
    fn index(&self, index: u16) -> &Self::Output {
        &self.buf[index as usize]
    }
}

impl IndexMut<u16> for Ram {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.buf[index as usize]
    }
}

impl Index<Range<u16>> for Ram {
    type Output = [u8];
    fn index(&self, index: Range<u16>) -> &Self::Output {
        &self.buf[index.start as usize..index.end as usize]
    }
}

pub const FONTSET_ADDR: u16 = 0x50;
pub const FONTSET: [u8; 80] = [
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
