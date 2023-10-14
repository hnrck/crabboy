mod mmu;
mod types;
mod cartridge;
mod rom;
mod cpu;

use clap::Parser;
use crate::rom::Rom;

#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Opt {
    #[clap(short = 'r', long = "rom", required = true)]
    rom: String,
}

fn main() {
    let opt = Opt::parse();
    let rom_path: &String = &opt.rom;

    let rom: Rom = Rom::from_path(&rom_path).expect("Failed to load ROM");

    if !rom.validate() {
        panic!("Invalid ROM!");
    }
    rom.print_info();
}
