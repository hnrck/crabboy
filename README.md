# CrabBoy

Simple and stupid GB emulator in Rust.

## Usage

### Build

```bash
cargo build --release --package crabboy --bin crabboy
```

### Run

```bash
cargo run --release --package crabboy --bin crabboy -- --rom <path/to/a/homemade/rom.gb>
```

## TODO

A lot:

* Debug/Release
* Logs
* IO
* hardware simu
* time simu
* Display

## Sources

* [bgb documentation](http://bgb.bircd.org/pandocs.txt) found in http://bgb.bircd.org
* [Gameboy CPU (LR35902) instruction set](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
* [Game Boy™ CPU Manual (pdf)](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf)
* [Length of instruction LD A,(C) in gameboy ~Z80 processor](https://stackoverflow.com/questions/41353869/length-of-instruction-ld-a-c-in-gameboy-z80-processor)