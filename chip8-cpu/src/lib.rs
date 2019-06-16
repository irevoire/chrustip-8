// register name are going to be broken
// function containing instruction will also trigger a warning
#![allow(non_snake_case)]

pub mod cpu;
mod memory;

/// return a new chip8 cpu
pub fn new() -> cpu::Cpu {
    cpu::Cpu::new()
}
