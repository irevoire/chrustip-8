# Chrustip 8

A chip 8 emulator in rust.

## Requirement
* rust
* cargo

The easiest way to get all the rust environment is to install [rustup](http://rustup.rs/).

## Compile
```
cargo build --release
```

## Execute
```
cargo run --release [path to game]
```

## Run unit tests
```
cargo test
```

## Generate documentation
There is not so much documentation.
The only usefull thing here is the documentation of the instructions
```
cargo rustdoc --open -- --document-private-items
```
