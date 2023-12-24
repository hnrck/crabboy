use clap::Parser;

use crate::cpu::CPU;
use crate::mmu::MMU;
use crate::rom::Rom;

mod mmu;
mod types;
mod cartridge;
mod rom;
mod cpu;

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
    let mut mmu = MMU::new().with_rom(rom);
    let mut cpu = CPU::new();
    loop {
        cpu.step(&mut mmu);
    }
}
