# Chrustip 8

A chip 8 emulator in rust.
I've made this project to learn more about rust and emulation.
Since I wanted to try multiple crate for handling the UI / graphics, most of my implemntation are shit but I still kept everything.

This repository is splitted in multiple part:
* **[chip8-cpu](chip8-cpu)**: Hold the code which simulate the chip-8 cpu with it's memory and registers
* **[chip8-minifb](chip8-minifb)**: My first implementation. It mostly works but there is a bug when a game wait for an input. You can see this for example when running the [KALEID](games/KALEID) game.
* **[chip8-ncurses](chip8-ncurses)**: My second implementation, it works in terminal. Getting any input is still shitty though.

## Requirement
* rust
* cargo

The easiest way to get all the rust environment is to install [rustup](http://rustup.rs/).

## Compile
### Compile all the binary
```
cargo build --release
```

### Compile only one of the binary
```
cargo build --release --bin [bin name] # chip8-minifb for example
```

## Execute
```
cargo run --release --bin [bin name] [path to game]
```

## Run unit tests
```
cargo test --lib
```
