# Yet another CHIP-8 emulator written in Rust

A simple and modular CHIP-8 emulator written in Rust.

![chip8-rust](https://raw.githubusercontent.com/nlkl/chip8-rust/master/img/screenshot.png)

SDL2 is used for rendering and IO, but the emulator is built in a way that makes it rather trivial to use swap out the frontend.

Although it currently requires recompilation, the emulator supports most common CHIP-8 quirks. See `settings.rs` for further details.

Currently unsupported features:
* Sound

## Build

To build the emulator, simply run:

```
cargo build
```

To run the test suite:

```
cargo test
```

## Usage

To execute, specify a path to the ROM you wish to execute:

```
chip8 path/to/rom.ch8
```
