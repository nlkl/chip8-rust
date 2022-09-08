# Yet another CHIP-8 emulator written in Rust

A simple and modular CHIP-8 emulator written in Rust.

![chip8-rust](https://raw.githubusercontent.com/nlkl/chip8-rust/master/img/screenshot.png)

SDL2 is used for rendering and IO, but the emulator is built in a way that makes it rather trivial to use swap out the frontend.

Although it currently requires recompilation, the emulator supports most common CHIP-8 quirks. See `settings.rs` for further details.

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

## Key map

Toogle sound: m

Keypad:

| Keyboard key | Chip-8 key |
| - | - |
| 1 | 1 |
| 2 | 2 |
| 3 | 3 |
| 4 | c |
| q | 4 |
| w | 5 |
| e | 6 |
| r | d |
| a | 7 |
| s | 8 |
| d | 9 |
| f | e |
| z | a |
| x | 0 |
| c | b |
| v | f |