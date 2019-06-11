// register name are going to be broken
// function containing instruction will also trigger a warning
#![allow(non_snake_case)]

use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

const MEMORY_SIZE: usize = 0x1000;

const FONTSET: [u8; 5 * 16] = [
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

pub struct Chip8 {
    opcode: u16,               // all the instruction are on two bytes
    memory: [u8; MEMORY_SIZE], // 4ko for the chip8
    V: [u8; 16],               // 15 register + one carry flag
    I: u16,                    // index register
    pc: usize,                 // program counter
    stack: [usize; 16],
    sp: usize, // stack pointer

    delay_timer: u8, // timers -> goto zero
    sound_timer: u8, // when zero buzzer is triggered

    pub key: [bool; 16],   // which key are pressed
    screen: [u8; 64 * 32], // pixel array

    draw: bool, // indicate if we should draw the screen
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut chip = Chip8 {
            opcode: 0,
            memory: [0; MEMORY_SIZE],
            V: [0; 16],
            I: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,

            delay_timer: 0,
            sound_timer: 0,

            key: [false; 16],
            screen: [0; 64 * 32],

            draw: true,
        };

        for (i, v) in FONTSET.iter().enumerate() {
            chip.memory[i] = *v
        }

        chip
    }

    /// Load the game into the chip-8 memory from 0x200 to the end of memory
    pub fn load_game(&mut self, file: &str) -> std::io::Result<()> {
        let mut file = File::open(file)?;
        file.read(&mut self.memory[0x200..])?;
        Ok(())
    }

    pub fn cycle(&mut self) {
        self.handle_opcode();

        self.delay_timer = self.delay_timer.checked_sub(1).unwrap_or(0);
        self.sound_timer = self.sound_timer.checked_sub(1).unwrap_or(0);
    }

    pub fn sound(&self) -> bool {
        self.sound_timer == 1
    }

    pub fn update(&mut self) -> Option<&[u8]> {
        let draw = self.draw;
        self.draw = false;
        match draw {
            true => Some(&self.screen),
            false => None,
        }
    }

    fn handle_opcode(&mut self) {
        let opcode = ((self.memory[self.pc] as u16) << 8) | (self.memory[self.pc + 1] as u16);
        self.opcode = opcode.clone();

        match opcode {
            _ if opcode == 0x00EE => self.opcode_00EE(),
            _ if opcode == 0x00E0 => self.opcode_00E0(),
            _ if (opcode & 0xF000) == 0x0000 => self.opcode_0NNN(),
            _ if (opcode & 0xF000) == 0x1000 => self.opcode_1NNN(),
            _ if (opcode & 0xF000) == 0x2000 => self.opcode_2NNN(),
            _ if (opcode & 0xF000) == 0x3000 => self.opcode_3XNN(),
            _ if (opcode & 0xF000) == 0x4000 => self.opcode_4XNN(),
            _ if (opcode & 0xF00F) == 0x5000 => self.opcode_5XY0(),
            _ if (opcode & 0xF000) == 0x6000 => self.opcode_6XNN(),
            _ if (opcode & 0xF000) == 0x7000 => self.opcode_7XNN(),
            _ if (opcode & 0xF00F) == 0x8000 => self.opcode_8XY0(),
            _ if (opcode & 0xF00F) == 0x8001 => self.opcode_8XY1(),
            _ if (opcode & 0xF00F) == 0x8002 => self.opcode_8XY2(),
            _ if (opcode & 0xF00F) == 0x8003 => self.opcode_8XY3(),
            _ if (opcode & 0xF00F) == 0x8004 => self.opcode_8XY4(),
            _ if (opcode & 0xF00F) == 0x8005 => self.opcode_8XY5(),
            _ if (opcode & 0xF00F) == 0x8006 => self.opcode_8XY6(),
            _ if (opcode & 0xF00F) == 0x8007 => self.opcode_8XY7(),
            _ if (opcode & 0xF00F) == 0x800E => self.opcode_8XYE(),
            _ if (opcode & 0xF00F) == 0x9000 => self.opcode_9XY0(),
            _ if (opcode & 0xF000) == 0xA000 => self.opcode_ANNN(),
            _ if (opcode & 0xF000) == 0xB000 => self.opcode_BNNN(),
            _ if (opcode & 0xF000) == 0xC000 => self.opcode_CXNN(),
            _ if (opcode & 0xF000) == 0xD000 => self.opcode_DXYN(),
            _ if (opcode & 0xF0FF) == 0xE09E => self.opcode_EX9E(),
            _ if (opcode & 0xF0FF) == 0xE0A1 => self.opcode_EXA1(),
            _ if (opcode & 0xF0FF) == 0xF007 => self.opcode_FX07(),
            _ if (opcode & 0xF0FF) == 0xF00A => self.opcode_FX0A(),
            _ if (opcode & 0xF0FF) == 0xF015 => self.opcode_FX15(),
            _ if (opcode & 0xF0FF) == 0xF018 => self.opcode_FX18(),
            _ if (opcode & 0xF0FF) == 0xF01E => self.opcode_FX1E(),
            _ if (opcode & 0xF0FF) == 0xF029 => self.opcode_FX29(),
            _ if (opcode & 0xF0FF) == 0xF033 => self.opcode_FX33(),
            _ if (opcode & 0xF0FF) == 0xF055 => self.opcode_FX55(),
            _ if (opcode & 0xF0FF) == 0xF065 => self.opcode_FX65(),
            _ => println!("Unknown opcode: 0x{:4X}", opcode),
        }
    }

    fn NNN(&self) -> u16 {
        (self.opcode & 0x0FFF).into()
    }

    fn NN(&self) -> u8 {
        (self.opcode & 0x00FF) as u8
    }

    fn N(&self) -> u8 {
        (self.opcode & 0x000F) as u8
    }

    fn X(&self) -> usize {
        ((self.opcode & 0x0F00) >> 8).into()
    }

    fn Y(&self) -> usize {
        ((self.opcode & 0x00F0) >> 4).into()
    }

    fn VX(&self) -> u8 {
        self.V[self.X()]
    }

    fn VY(&self) -> u8 {
        self.V[self.Y()]
    }

    /// ## INSTRUCTION

    /// Clears the screen.
    fn opcode_00E0(&mut self) {
        for p in self.screen.iter_mut() {
            *p = 0
        }
        self.pc += 2;
        self.draw = true;
    }

    /// Returns from a subroutine.
    fn opcode_00EE(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp];
        self.pc += 2;
    }

    /// Calls RCA 1802 program at address NNN. Not necessary for most ROMs.
    /// !!! **NOT IMPLEMENTED**  !!!
    fn opcode_0NNN(&mut self) {
        self.pc += 2;
    }

    /// Jumps to address NNN.
    fn opcode_1NNN(&mut self) {
        self.pc = self.NNN().into();
    }

    /// Calls subroutine at NNN.
    fn opcode_2NNN(&mut self) {
        self.stack[self.sp] = self.pc;
        self.sp += 1;
        self.pc = self.NNN().into();
    }

    /// Skips the next instruction if VX equals NN.
    /// (Usually the next instruction is a jump to skip a code block)
    fn opcode_3XNN(&mut self) {
        match self.VX() == self.NN() {
            true => self.pc += 4,
            false => self.pc += 2,
        }
    }

    /// Skips the next instruction if VX doesn't equal NN.
    /// (Usually the next instruction is a jump to skip a code block)
    fn opcode_4XNN(&mut self) {
        match self.VX() != self.NN() {
            true => self.pc += 4,
            false => self.pc += 2,
        }
    }

    /// Skips the next instruction if VX equals VY.
    /// (Usually the next instruction is a jump to skip a code block)
    fn opcode_5XY0(&mut self) {
        match self.VX() == self.VY() {
            true => self.pc += 4,
            false => self.pc += 2,
        }
    }

    /// Sets VX to NN.
    fn opcode_6XNN(&mut self) {
        self.V[self.X()] = self.NN();
        self.pc += 2;
    }

    /// Adds NN to VX. (Carry flag is not changed)
    fn opcode_7XNN(&mut self) {
        let X = self.X();

        self.V[X] = self.V[X].wrapping_add(self.NN());
        self.pc += 2;
    }

    /// Sets VX to the value of VY.
    fn opcode_8XY0(&mut self) {
        self.V[self.X()] = self.VY();
        self.pc += 2;
    }

    /// Sets VX to VX or VY. (Bitwise OR operation)
    fn opcode_8XY1(&mut self) {
        self.V[self.X()] |= self.VY();
        self.pc += 2;
    }

    /// Sets VX to VX and VY. (Bitwise AND operation)
    fn opcode_8XY2(&mut self) {
        self.V[self.X()] &= self.VY();
        self.pc += 2;
    }

    /// Sets VX to VX xor VY.
    fn opcode_8XY3(&mut self) {
        self.V[self.X()] ^= self.VY();
        self.pc += 2;
    }

    /// Adds VY to VX.
    /// VF is set to 1 when there's a carry, and to 0 when there isn't.
    fn opcode_8XY4(&mut self) {
        let X = self.X();

        let (res, carry) = self.V[X].overflowing_add(self.VY());
        self.V[X] = res;
        self.V[0xF] = carry.into();
        self.pc += 2;
    }

    /// VY is subtracted from VX. VF is set to 0 when there's a borrow,
    /// and 1 when there isn't.
    fn opcode_8XY5(&mut self) {
        let X = self.X();

        let (res, carry) = self.V[X].overflowing_sub(self.VY());
        self.V[X] = res;
        self.V[0xF] = (!carry).into();
        self.pc += 2;
    }

    /// Stores the least significant bit of VX in VF and then shifts VX
    /// to the right by 1.
    fn opcode_8XY6(&mut self) {
        let X = self.X();

        self.V[0xF] = self.V[X] & 0x1;
        self.V[X] >>= 1;
        self.pc += 2;
    }

    /// Sets VX to VY minus VX. VF is set to 0 when there's a borrow,
    /// and 1 when there isn't.
    fn opcode_8XY7(&mut self) {
        let X = self.X();
        let (res, carry) = self.VY().overflowing_sub(self.V[X]);

        self.V[X] = res;
        self.V[0xF] = (!carry).into();
        self.pc += 2;
    }

    /// Stores the most significant bit of VX in VF and then shifts
    /// VX to the left by 1.
    fn opcode_8XYE(&mut self) {
        let X = self.X();

        self.V[0xF] = self.V[X] >> 7;
        self.V[X] <<= 1;
        self.pc += 2;
    }

    /// Skips the next instruction if VX doesn't equal VY.
    /// (Usually the next instruction is a jump to skip a code block)
    fn opcode_9XY0(&mut self) {
        match self.VX() != self.VY() {
            true => self.pc += 4,
            false => self.pc += 2,
        }
    }

    /// Sets I to the address NNN
    fn opcode_ANNN(&mut self) {
        self.I = self.NNN();
        self.pc += 2;
    }

    /// Jumps to the address NNN plus V0.
    fn opcode_BNNN(&mut self) {
        self.pc = self.NNN() as usize + self.V[0] as usize;
    }

    /// Sets VX to the result of a bitwise and operation on
    /// a random number (Typically: 0 to 255) and NN.
    fn opcode_CXNN(&mut self) {
        let rand: u8 = rand::thread_rng().gen();

        self.V[self.X()] = rand & self.NN();
        self.pc += 2;
    }

    /// Draws a sprite at coordinate (VX, VY) that has a width
    /// of 8 pixels and a height of N pixels. Each row of 8 pixels is read
    /// as bit-coded starting from memory location I;
    /// I value doesn’t change after the execution of this instruction.
    /// As described above, VF is set to 1 if any screen pixels are flipped
    /// from set to unset when the sprite is drawn, and to 0 if that doesn’t happen.
    fn opcode_DXYN(&mut self) {
        let X = self.VX() as usize;
        let Y = self.VY() as usize;
        let N = self.N() as usize; // heigth

        self.V[0xF] = 0;
        for y in 0..N {
            let pixel = self.memory[self.I as usize + y];
            for x in 0..8 {
                if (pixel & (0x80 >> x)) != 0 {
                    let pos = (X + x + ((Y + y) * 64)) % 2048;
                    let pos = pos as usize;

                    if self.screen[pos] != 0 {
                        self.V[0xF] = 1
                    }
                    self.screen[pos] = !self.screen[pos];
                }
            }
        }
        self.pc += 2;
        self.draw = true;
    }

    /// Skips the next instruction if the key stored in VX is pressed.
    /// (Usually the next instruction is a jump to skip a code block)
    fn opcode_EX9E(&mut self) {
        match self.key[self.VX() as usize] {
            true => self.pc += 4,
            false => self.pc += 2,
        }
    }

    /// Skips the next instruction if the key stored in VX isn't pressed.
    /// (Usually the next instruction is a jump to skip a code block)
    fn opcode_EXA1(&mut self) {
        match self.key[self.VX() as usize] {
            true => self.pc += 2,
            false => self.pc += 4,
        }
    }

    /// Sets VX to the value of the delay timer.
    fn opcode_FX07(&mut self) {
        self.V[self.X()] = self.delay_timer;
        self.pc += 2;
    }

    /// A key press is awaited, and then stored in VX.
    /// (Blocking Operation. All instruction halted until next key event)
    /// This is implemented by NOT incrementing the program counter (pc)
    /// so when getting the new opcode we'll re-execute this instruction
    fn opcode_FX0A(&mut self) {
        for (idx, key) in self.key.iter().enumerate() {
            if *key {
                self.V[self.X()] = idx as u8;
                self.pc += 2;
                return;
            }
        }
        // If we didn't received a keypress, skip this cycle and try again.
    }

    /// Sets the delay timer to VX.
    fn opcode_FX15(&mut self) {
        self.delay_timer = self.VX();
        self.pc += 2;
    }

    /// Sets the sound timer to VX.
    fn opcode_FX18(&mut self) {
        self.sound_timer = self.VX();
        self.pc += 2;
    }

    /// Adds VX to I.
    fn opcode_FX1E(&mut self) {
        let (res, carry) = self.I.overflowing_add(self.VX() as u16);
        self.I = res;
        self.V[0xF] = carry.into();
        self.pc += 2;
    }

    /// Sets I to the location of the sprite for the character in VX.
    /// Characters 0-F (in hexadecimal) are represented by a 4x5 font.
    fn opcode_FX29(&mut self) {
        self.I = self.VX().wrapping_mul(0x5).into();
        self.pc += 2;
    }

    /// Stores the binary-coded decimal representation of VX, with
    /// the most significant of three digits at the address in I, the middle
    /// digit at I plus 1, and the least significant digit at I plus 2.
    /// -------------------
    /// In other words, take the decimal representation of VX:
    /// * Place the hundreds digit in memory at location in I
    /// * The tens digit at location I+1
    /// * And the ones digit at location I+2)
    fn opcode_FX33(&mut self) {
        let VX = self.VX();

        self.memory[(self.I) as usize] = VX / 100;
        self.memory[(self.I + 1) as usize] = (VX / 10) % 10;
        self.memory[(self.I + 2) as usize] = VX % 100;
        self.pc += 2;
    }

    /// Stores V0 to VX (including VX) in memory starting at address I.
    /// The offset from I is increased by 1 for each value written,
    /// but I itself is left unmodified.
    fn opcode_FX55(&mut self) {
        for i in 0..=self.X() {
            self.memory[self.I as usize + i] = self.V[i];
        }

        self.pc += 2;
    }

    /// Fills V0 to VX (including VX) with values from memory starting
    /// at address I. The offset from I is increased by 1 for each
    /// value written, but I itself is left unmodified.
    fn opcode_FX65(&mut self) {
        for i in 0..=self.X() {
            self.V[i as usize] = self.memory[self.I as usize + i];
        }

        self.pc += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() -> Chip8 {
        Chip8::new()
    }

    #[test]
    fn test_init() {
        let _c = init(); // it could segfault ¯\_(ツ)_/¯
    }

    #[test]
    fn opcode_00E0() {
        let mut c = init();
        c.opcode_00E0();
        assert_eq!(c.pc, 0x202);
        for p in c.screen.iter() {
            assert_eq!(*p, 0);
        }
        assert_eq!(c.draw, true);
    }

    #[test]
    fn opcode_00EE() {
        let mut c = init();
        c.sp = 1;
        c.stack[0] = 40;
        c.opcode_00EE();
        assert_eq!(c.sp, 0);
        assert_eq!(c.pc, 42);
    }

    #[test]
    fn opcode_0NNN() {
        let mut c = init();
        c.opcode_0NNN();
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_1NNN() {
        let mut c = init();
        c.opcode = 0x1B0B;
        c.opcode_1NNN();
        assert_eq!(c.pc, 0xB0B);
    }

    #[test]
    fn opcode_2NNN() {
        let mut c = init();
        c.opcode = 0x2B0B;
        c.opcode_2NNN();
        assert_eq!(
            c.stack,
            [0x200, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(c.sp, 1);
        assert_eq!(c.pc, 0xB0B);
    }

    #[test]
    fn opcode_3XNN_false() {
        let mut c = init();
        c.opcode = 0x3ABB;
        c.V[0xA] = 0xAA;
        c.opcode_3XNN();
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_3XNN_true() {
        let mut c = init();
        c.opcode = 0x3ABB;
        c.V[0x0A] = 0xBB;
        c.opcode_3XNN();
        assert_eq!(c.pc, 0x204);
    }

    #[test]
    fn opcode_4XNN_false() {
        let mut c = init();
        c.opcode = 0x4ABB;
        c.V[0xA] = 0xAA;
        c.opcode_4XNN();
        assert_eq!(c.pc, 0x204);
    }

    #[test]
    fn opcode_4XNN_true() {
        let mut c = init();
        c.opcode = 0x4ABB;
        c.V[0x0A] = 0xBB;
        c.opcode_4XNN();
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_5XY0_false() {
        let mut c = init();
        c.opcode = 0x5AB0;
        c.V[0xA] = 0xAA;
        c.V[0xB] = 0xBB;
        c.opcode_5XY0();
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_5XY0_true() {
        let mut c = init();
        c.opcode = 0x5AB0;
        c.V[0xA] = 0xBB;
        c.V[0xB] = 0xBB;
        c.opcode_5XY0();
        assert_eq!(c.pc, 0x204);
    }

    #[test]
    fn opcode_6XNN() {
        let mut c = init();
        c.opcode = 0x6ABB;
        c.opcode_6XNN();
        assert_eq!(c.V[0xA], 0xBB);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_7XNN() {
        let mut c = init();
        c.opcode = 0x7ABB;
        c.V[0xA] = 0x11;
        c.opcode_7XNN();
        assert_eq!(c.V[0xA], 0xCC);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_7XNN_overflow() {
        let mut c = init();
        c.opcode = 0x7AEE;
        c.V[0xA] = 0x12;
        c.opcode_7XNN();
        assert_eq!(c.V[0xA], 0x00);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_8XY0() {
        let mut c = init();
        c.opcode = 0x8AB0;
        c.V[0xA] = 0xAA;
        c.V[0xB] = 0xBB;
        c.opcode_8XY0();
        assert_eq!(c.V[0xA], 0xBB);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_8XY1() {
        let mut c = init();
        c.opcode = 0x8AB1;
        c.V[0xA] = 0xAA;
        c.V[0xB] = 0xBB;
        c.opcode_8XY1();
        assert_eq!(c.V[0xA], 0xAA | 0xBB);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_8XY2() {
        let mut c = init();
        c.opcode = 0x8AB2;
        c.V[0xA] = 0xEE;
        c.V[0xB] = 0x55;
        c.opcode_8XY2();
        assert_eq!(c.V[0xA], 0xEE & 0x55);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_8XY3() {
        let mut c = init();
        c.opcode = 0x8AB3;
        c.V[0xA] = 0xEE;
        c.V[0xB] = 0x55;
        c.opcode_8XY3();
        assert_eq!(c.V[0xA], 0xEE ^ 0x55);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_8XY4() {
        let mut c = init();
        c.opcode = 0x8AB4;
        c.V[0xA] = 0x11;
        c.V[0xB] = 0xAA;
        c.opcode_8XY4();
        assert_eq!(c.V[0xA], 0xBB);
        assert_eq!(c.V[0xF], 0x00);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_8XY4_with_carry() {
        let mut c = init();
        c.opcode = 0x8AB4;
        c.V[0xA] = 0x12;
        c.V[0xB] = 0xFF;
        c.opcode_8XY4();
        assert_eq!(c.V[0xA], 0x11);
        assert_eq!(c.V[0xF], 0x01);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_8XY4_with_and_without_carry() {
        let mut c = init();
        c.opcode = 0x8AB4;
        c.V[0xA] = 0x12;
        c.V[0xB] = 0xFF;
        c.opcode_8XY4();
        assert_eq!(c.V[0xA], 0x11);
        assert_eq!(c.V[0xF], 0x01);
        assert_eq!(c.pc, 0x202);

        c.V[0xB] = 0x22;
        c.opcode_8XY4();
        assert_eq!(c.V[0xA], 0x33);
        assert_eq!(c.V[0xF], 0x00);
        assert_eq!(c.pc, 0x204);
    }

    #[test]
    fn opcode_8XY5() {
        let mut c = init();
        c.opcode = 0x8AB5;
        c.V[0xA] = 0xAA;
        c.V[0xB] = 0x11;
        c.opcode_8XY5();
        assert_eq!(c.V[0xA], 0x99);
        assert_eq!(c.V[0xF], 0x01);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_8XY5_with_carry() {
        let mut c = init();
        c.opcode = 0x8AB5;
        c.V[0xA] = 0x10;
        c.V[0xB] = 0x22;
        c.opcode_8XY5();
        assert_eq!(c.V[0xA], 0xEE);
        assert_eq!(c.V[0xF], 0x00);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_8XY5_with_and_without_carry() {
        let mut c = init();
        c.opcode = 0x8AB5;
        c.V[0xA] = 0x10;
        c.V[0xB] = 0x22;
        c.opcode_8XY5();
        assert_eq!(c.V[0xA], 0xEE);
        assert_eq!(c.V[0xF], 0x00);
        assert_eq!(c.pc, 0x202);

        c.V[0xB] = 0x22;
        c.opcode_8XY5();
        assert_eq!(c.V[0xA], 0xCC);
        assert_eq!(c.V[0xF], 0x01);
        assert_eq!(c.pc, 0x204);
    }

    #[test]
    fn opcode_8XY6() {
        let mut c = init();
        c.opcode = 0x8AB6;
        c.V[0xA] = 0xFF;
        c.opcode_8XY6();
        assert_eq!(c.V[0xA], 0xFF >> 1);
        assert_eq!(c.V[0xF], 0x01);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_8XY7() {
        let mut c = init();
        c.opcode = 0x8AB7;
        c.V[0xA] = 0x11;
        c.V[0xB] = 0xAA;
        c.opcode_8XY7();
        assert_eq!(c.V[0xA], 0x99);
        assert_eq!(c.V[0xF], 0x01);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_8XY7_with_carry() {
        let mut c = init();
        c.opcode = 0x8AB7;
        c.V[0xA] = 0x22;
        c.V[0xB] = 0x10;
        c.opcode_8XY7();
        assert_eq!(c.V[0xA], 0xEE);
        assert_eq!(c.V[0xF], 0x00);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_8XY7_with_and_without_carry() {
        let mut c = init();
        c.opcode = 0x8AB7;
        c.V[0xA] = 0x22;
        c.V[0xB] = 0x10;
        c.opcode_8XY7();
        assert_eq!(c.V[0xA], 0xEE);
        assert_eq!(c.V[0xF], 0x00);
        assert_eq!(c.pc, 0x202);

        c.V[0xB] = 0xFF;
        c.opcode_8XY7();
        assert_eq!(c.V[0xA], 0x11);
        assert_eq!(c.V[0xF], 0x01);
        assert_eq!(c.pc, 0x204);
    }

    #[test]
    fn opcode_8XYE() {
        let mut c = init();
        c.opcode = 0x8ABE;
        c.V[0xA] = 0xEF;
        c.opcode_8XYE();
        assert_eq!(c.V[0xA], 0xEF << 1);
        assert_eq!(c.V[0xF], 0x01);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_9XY0_not_equal() {
        let mut c = init();
        c.opcode = 0x9AB0;
        c.V[0xA] = 0xAA;
        c.V[0xB] = 0xBB;
        c.opcode_9XY0();
        assert_eq!(c.pc, 0x204);
    }

    #[test]
    fn opcode_9XY0_equal() {
        let mut c = init();
        c.opcode = 0x9AB0;
        c.V[0xA] = 0xAA;
        c.V[0xB] = 0xAA;
        c.opcode_9XY0();
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_ANNN() {
        let mut c = init();
        c.opcode = 0xA777;
        c.opcode_ANNN();
        assert_eq!(c.I, 0x777);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_BNNN() {
        let mut c = init();
        c.opcode = 0xB777;
        c.V[0x0] = 0x11;
        c.opcode_BNNN();
        assert_eq!(c.pc, 0x788);
    }

    #[test]
    fn opcode_CXNN() {
        let mut c = init();
        c.opcode = 0xC7F0; // the right part should be to zero event after the and
        c.opcode_CXNN();
        assert_eq!(c.V[0x7] & 0x0F, 0x0);
        // can't test a lot more because of random
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_DXYN() {
        let mut c = init();
        c.opcode = 0xDABC;
        c.opcode_DXYN();
        assert_eq!(c.pc, 0x202);
        assert_eq!(c.draw, true);
        // TODO test more things TODO
    }

    #[test]
    fn opcode_EX9E_pressed() {
        let mut c = init();
        c.opcode = 0xEA9E;
        c.V[0xA] = 0x07;
        c.key[0x7] = true;
        c.opcode_EX9E();
        assert_eq!(c.pc, 0x204);
    }

    #[test]
    fn opcode_EX9E_not_pressed() {
        let mut c = init();
        c.opcode = 0xEA9E;
        c.V[0xA] = 0x07;
        c.key[0x7] = false;
        c.opcode_EX9E();
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_EXA1_pressed() {
        let mut c = init();
        c.opcode = 0xEA9E;
        c.V[0xA] = 0x07;
        c.key[0x7] = true;
        c.opcode_EXA1();
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_EXA1_not_pressed() {
        let mut c = init();
        c.opcode = 0xEA9E; // the right part should be to zero event after the and
        c.V[0xA] = 0x07;
        c.key[0x7] = false;
        c.opcode_EXA1();
        assert_eq!(c.pc, 0x204);
    }

    #[test]
    fn opcode_FX07() {
        let mut c = init();
        c.opcode = 0xFA07; // the right part should be to zero event after the and
        c.V[0xA] = 0x07;
        c.delay_timer = 0x12;
        c.opcode_FX07();
        assert_eq!(c.V[0xA], 0x12);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_FX0A() {
        let mut c = init();
        c.opcode = 0xEA9E; // the right part should be to zero event after the and
        c.V[0xA] = 0x07;
        c.opcode_FX0A();
        assert_eq!(c.V[0xA], 0x07);
        assert_eq!(c.pc, 0x200);

        c.key[0x2] = true;
        c.opcode_FX0A();
        assert_eq!(c.V[0xA], 0x02);
        assert_eq!(c.pc, 0x202);

        c.key[0x2] = false;
        c.key[0x8] = true;
        c.opcode_FX0A();
        assert_eq!(c.V[0xA], 0x08);
        assert_eq!(c.pc, 0x204);
    }

    #[test]
    fn opcode_FX15() {
        let mut c = init();
        c.opcode = 0xFA15;
        c.V[0xA] = 0x77;
        c.opcode_FX15();
        assert_eq!(c.delay_timer, 0x77);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_FX18() {
        let mut c = init();
        c.opcode = 0xFA18;
        c.V[0xA] = 0x77;
        c.opcode_FX18();
        assert_eq!(c.sound_timer, 0x77);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_FX1E_without_carry() {
        let mut c = init();
        c.opcode = 0xFA1E;
        c.V[0xA] = 0x11;
        c.I = 0xAA;
        c.opcode_FX1E();
        assert_eq!(c.I, 0xBB);
        assert_eq!(c.V[0xF], 0x0);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_FX1E_with_carry() {
        let mut c = init();
        c.opcode = 0xFA1E;
        c.V[0xA] = 0x23;
        c.I = 0xFFEE;
        c.opcode_FX1E();
        assert_eq!(c.I, 0x11);
        assert_eq!(c.V[0xF], 0x1);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_FX29() {
        let mut c = init();
        c.opcode = 0xFA29;
        c.V[0xA] = 0x11;
        c.I = 0xAA;
        c.opcode_FX29();
        assert_eq!(c.I, 0x55);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_FX33() {
        let mut c = init();
        c.opcode = 0xFA33;
        c.opcode_FX33();
        assert_eq!(c.pc, 0x202);
        // TODO do something
    }

    #[test]
    fn opcode_FX55() {
        let mut c = init();
        c.opcode = 0xF333;
        c.V[0] = 0x00;
        c.V[1] = 0x11;
        c.V[2] = 0x22;
        c.V[3] = 0x33;
        c.I = 0xAA;
        c.memory[0xAA + 0x4] = 0xFF;

        c.opcode_FX55();

        assert_eq!(c.memory[0xAA + 0], 0x00);
        assert_eq!(c.memory[0xAA + 1], 0x11);
        assert_eq!(c.memory[0xAA + 2], 0x22);
        assert_eq!(c.memory[0xAA + 3], 0x33);
        assert_eq!(c.memory[0xAA + 4], 0xFF);
        assert_eq!(c.I, 0xAA);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn opcode_FX65() {
        let mut c = init();
        c.opcode = 0xF333;
        c.I = 0xAA;
        c.memory[c.I as usize + 0] = 0x00;
        c.memory[c.I as usize + 1] = 0x11;
        c.memory[c.I as usize + 2] = 0x22;
        c.memory[c.I as usize + 3] = 0x33;
        c.V[0x4] = 0xFF;

        c.opcode_FX65();

        assert_eq!(c.V[0], 0x00);
        assert_eq!(c.V[1], 0x11);
        assert_eq!(c.V[2], 0x22);
        assert_eq!(c.V[3], 0x33);
        assert_eq!(c.V[4], 0xFF);
        assert_eq!(c.I, 0xAA);
        assert_eq!(c.pc, 0x202);
    }

    #[test]
    fn return_after_call() {
        let mut c = init();
        c.opcode = 0x2B0B;
        c.opcode_2NNN();
        c.opcode_00EE();
        assert_eq!(
            c.stack, // we don't clear the stack after returning
            [0x200, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(c.sp, 0);
        assert_eq!(c.pc, 0x202);
    }
}
