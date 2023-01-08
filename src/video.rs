use std::ops::{Index, IndexMut};

pub struct Video {
    buf: [bool; 2048],
}

impl Video {
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 32;

    pub fn new() -> Self {
        Self { buf: [false; 2048] }
    }

    pub fn draw(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        let x = x as usize % Video::WIDTH;
        let y = y as usize % Video::HEIGHT;
        let h = sprite.len();
        let mut flipped = false;
        
        for row in 0..h {
            let line = sprite[row];
            for col in 0..8 {
                if (line & (0b1000_0000 >> col)) != 0 {
                    let i = (x + col) % Self::WIDTH;
                    let j = (y + row) % Self::HEIGHT;
                    let index = i + Self::WIDTH * j;
    
                    flipped = flipped || self.buf[index] as bool;
                    self.buf[index] ^= true;
                }
            }
        }

        flipped
    }

    pub fn clear(&mut self) {
        self.buf.fill(false);
    }
}

impl Index<usize> for Video {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    }
}

impl IndexMut<usize> for Video {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buf[index]
    }
}
