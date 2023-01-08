use crate::keypad::Keypad;
use crate::ram::{Ram, FONTSET_ADDR};
use crate::video::Video;
use std::hint::unreachable_unchecked;

pub struct Cpu {
    pc: u16,
    v: [u8; 16],
    i: u16,
    sp: u8,
    stack: [u16; 16],
    delay: u8,
    opcode: u16,
    ram: Ram,
    pub video: Video,
    pub keypad: Keypad,
}

impl Cpu {
    pub const START_ADDR: u16 = 0x200;

    pub fn new() -> Self {
        Self {
            pc: Self::START_ADDR,
            v: [0; 16],
            i: 0,
            sp: 0,
            stack: [0; 16],
            delay: 0,
            opcode: 0x0000,
            ram: Ram::new(),
            video: Video::new(),
            keypad: Keypad::new(),
        }
    }

    pub fn with_rom(rom: &[u8]) -> Self {
        let mut cpu = Self::new();
        cpu.load_rom(rom);
        cpu
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.ram.load(Self::START_ADDR, rom);
    }

    pub fn cycle(&mut self) {
        self.fetch();
        self.pc += 2;
        self.execute();
        self.delay = self.delay.saturating_sub(1);
    }

    fn fetch(&mut self) {
        self.opcode = u16::from_be_bytes([self.ram[self.pc], self.ram[self.pc + 1]]);
    }

    fn execute(&mut self) {
        match self.opcode & 0xF000 {
            0x0000 => self.op_0xxx(),
            0x1000 => self.op_1nnn(),
            0x2000 => self.op_2nnn(),
            0x3000 => self.op_3xkk(),
            0x4000 => self.op_4xkk(),
            0x5000 => self.op_5xy0(),
            0x6000 => self.op_6xkk(),
            0x7000 => self.op_7xkk(),
            0x8000 => self.op_8xxx(),
            0x9000 => self.op_9xy0(),
            0xA000 => self.op_annn(),
            0xB000 => self.op_bnnn(),
            0xC000 => self.op_cxkk(),
            0xD000 => self.op_dxyn(),
            0xE000 => self.op_exxx(),
            0xF000 => self.op_fxxx(),
            _ => unsafe { unreachable_unchecked() },
        }
    }

    fn op_0xxx(&mut self) {
        match self.opcode {
            0x00E0 => self.op_00e0(),
            0x00EE => self.op_00ee(),
            _ => {}
        }
    }

    fn op_8xxx(&mut self) {
        match self.opcode & 0x000F {
            0x0 => self.op_8xy0(),
            0x1 => self.op_8xy1(),
            0x2 => self.op_8xy2(),
            0x3 => self.op_8xy3(),
            0x4 => self.op_8xy4(),
            0x5 => self.op_8xy5(),
            0x6 => self.op_8xy6(),
            0x7 => self.op_8xy7(),
            0xE => self.op_8xye(),
            _ => {}
        }
    }

    fn op_exxx(&mut self) {
        match self.opcode & 0x00FF {
            0x9E => self.op_ex9e(),
            0xA1 => self.op_exa1(),
            _ => {}
        }
    }

    fn op_fxxx(&mut self) {
        match self.opcode & 0x00FF {
            0x07 => self.op_fx07(),
            0x0A => self.op_fx0a(),
            0x15 => self.op_fx15(),
            0x1E => self.op_fx1e(),
            0x29 => self.op_fx29(),
            0x33 => self.op_fx33(),
            0x55 => self.op_fx55(),
            0x65 => self.op_fx65(),
            _ => {}
        }
    }

    fn op_00e0(&mut self) {
        self.video.clear();
    }

    fn op_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    fn op_1nnn(&mut self) {
        self.pc = self.nnn();
    }

    fn op_2nnn(&mut self) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = self.nnn();
    }

    fn op_3xkk(&mut self) {
        if self.v[self.x()] == self.kk() {
            self.pc += 2;
        }
    }

    fn op_4xkk(&mut self) {
        if self.v[self.x()] != self.kk() {
            self.pc += 2;
        }
    }

    fn op_5xy0(&mut self) {
        if self.v[self.x()] == self.v[self.y()] {
            self.pc += 2;
        }
    }

    fn op_6xkk(&mut self) {
        self.v[self.x()] = self.kk();
    }

    fn op_7xkk(&mut self) {
        let x = self.x();
        self.v[x] = self.v[x].wrapping_add(self.kk());
    }

    fn op_8xy0(&mut self) {
        self.v[self.x()] = self.v[self.y()];
    }

    fn op_8xy1(&mut self) {
        self.v[self.x()] |= self.v[self.y()];
    }

    fn op_8xy2(&mut self) {
        self.v[self.x()] &= self.v[self.y()];
    }

    fn op_8xy3(&mut self) {
        self.v[self.x()] ^= self.v[self.y()];
    }

    fn op_8xy4(&mut self) {
        let x = self.x();
        let y = self.y();
        let overflow;

        (self.v[x], overflow) = self.v[x].overflowing_add(self.v[y]);
        self.v[0xF] = overflow as u8;
    }

    fn op_8xy5(&mut self) {
        let x = self.x();
        let y = self.y();
        let overflow;

        (self.v[x], overflow) = self.v[x].overflowing_sub(self.v[y]);
        self.v[0xF] = if overflow { 1 } else { 0 };
    }

    fn op_8xy6(&mut self) {
        let x = self.x();
        let overflow;

        (self.v[x], overflow) = self.v[x].overflowing_shr(1);
        self.v[0xF] = if overflow { 1 } else { 0 };
    }

    fn op_8xy7(&mut self) {
        let x = self.x();
        let y = self.y();
        let overflow;

        (self.v[x], overflow) = self.v[y].overflowing_sub(self.v[x]);
        self.v[0xF] = if overflow { 1 } else { 0 };
    }

    fn op_8xye(&mut self) {
        let x = self.x();
        let overflow;

        (self.v[x], overflow) = self.v[x].overflowing_shl(1);
        self.v[0xF] = if overflow { 1 } else { 0 };
    }

    fn op_9xy0(&mut self) {
        if self.v[self.x()] != self.v[self.y()] {
            self.pc += 2;
        }
    }

    fn op_annn(&mut self) {
        self.i = self.nnn();
    }

    fn op_bnnn(&mut self) {
        self.pc = self.v[0] as u16 + self.nnn();
    }

    fn op_cxkk(&mut self) {
        self.v[self.x()] = rand::random::<u8>() & self.kk();
    }

    fn op_dxyn(&mut self) {
        let x = self.v[self.x()];
        let y = self.v[self.y()];
        let h = self.opcode & 0x000F;

        self.v[0xF] = self.video.draw(x, y, &self.ram[self.i..self.i + h]) as u8;
    }

    fn op_ex9e(&mut self) {
        let key = self.v[self.x()];
        if self.keypad.is_pressed(key) {
            self.pc += 2;
        }
    }

    fn op_exa1(&mut self) {
        let key = self.v[self.x()];
        if !self.keypad.is_pressed(key) {
            self.pc += 2;
        }
    }

    fn op_fx07(&mut self) {
        self.v[self.x()] = self.delay;
    }

    fn op_fx0a(&mut self) {
        let x = self.x();

        for i in 0..15 {
            if self.keypad.is_pressed(i) {
                self.v[x] = i;
                return;
            }
        }

        self.pc -= 2;
    }

    fn op_fx15(&mut self) {
        self.delay = self.v[self.x()];
    }

    fn op_fx1e(&mut self) {
        self.i += self.v[self.x()] as u16;
    }

    fn op_fx29(&mut self) {
        self.i = FONTSET_ADDR + 5 * self.v[self.x()] as u16;
    }

    fn op_fx33(&mut self) {
        let vx = self.v[self.x()];
        self.ram[self.i + 2] = vx % 10;
        self.ram[self.i + 1] = (vx / 10) % 10;
        self.ram[self.i] = (vx / 100) % 10;
    }

    fn op_fx55(&mut self) {
        for offset in 0..=self.x() as u16 {
            self.ram[self.i + offset] = self.v[offset as usize];
        }
    }

    fn op_fx65(&mut self) {
        for offset in 0..=self.x() as u16 {
            self.v[offset as usize] = self.ram[self.i + offset];
        }
    }

    #[inline(always)]
    fn x(&self) -> usize {
        ((self.opcode & 0x0F00) >> 8) as usize
    }

    #[inline(always)]
    fn y(&self) -> usize {
        ((self.opcode & 0x00F0) >> 4) as usize
    }

    #[inline(always)]
    fn kk(&self) -> u8 {
        (self.opcode & 0x00FF) as u8
    }

    #[inline(always)]
    fn nnn(&self) -> u16 {
        self.opcode & 0x0FFF
    }
}
