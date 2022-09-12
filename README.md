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

```
    Keyboard                    CHIP-8 Keypad
 ---------------               ---------------
| 1 | 2 | 3 | 4 |             | 1 | 2 | 3 | C |
|---|---|---|---|             |---|---|---|---|
| Q | W | E | R |             | 4 | 5 | 6 | D |
|---|---|---|---|    ====>    |---|---|---|---|
| A | S | D | F |             | 7 | 8 | 9 | E |
|---|---|---|---|             |---|---|---|---|
| Z | X | C | V |             | A | 0 | B | F |
 ---------------               ---------------
```
