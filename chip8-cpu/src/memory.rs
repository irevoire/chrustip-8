use std::fs::File;
use std::io::Read;

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

pub struct Memory {
    raw: [u8; MEMORY_SIZE], // 4ko
}

/// just a little Helper so I don't have to pub Memory
pub fn new() -> Memory {
    Memory::new()
}

impl Memory {
    /// initialize the memory of the chip-8
    /// create an empty array of 4ko (MEMORY_SIZE)
    /// Then load the fontset from the start
    pub fn new() -> Self {
        let mut memory = Memory {
            raw: [0; MEMORY_SIZE],
        };
        for (i, v) in FONTSET.iter().enumerate() {
            memory[i] = *v
        }
        memory
    }

    /// Load the game into the chip-8 memory from 0x200 to the end of memory
    pub fn load_game(&mut self, file: &str) -> std::io::Result<()> {
        let mut file = File::open(file)?;
        file.read(&mut self.raw[0x200..])?;
        Ok(())
    }
}

/// Just implementing some easy access to the memory
/// Nothing interesting to see after this point
use std::ops::Index;
use std::ops::IndexMut;

impl Index<usize> for Memory {
    type Output = u8;
    fn index(&self, i: usize) -> &Self::Output {
        &self.raw[i]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut<'b>(&'b mut self, i: usize) -> &'b mut Self::Output {
        &mut self.raw[i]
    }
}
