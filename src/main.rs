use std::process;

use clap::Parser;
use log::{debug, error};

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

#[cfg(debug_assertions)]
fn init_logger() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
}

#[cfg(not(debug_assertions))]
fn init_logger() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
}

fn main() {
    init_logger();
    let opt = Opt::parse();
    let rom_path: &String = &opt.rom;
    match Rom::from_path(&rom_path) {
        Ok(rom) => {
            if rom.validate() {
                debug!("ROM loaded and validated successfully");
                rom.print_info();
                let mut mmu = MMU::new().with_rom(rom);
                let mut cpu = CPU::new();
                loop {
                    // TODO(henrick) Step error handling
                    cpu.step(&mut mmu);
                }
            } else {
                error!("Invalid ROM!");
                process::exit(1);
            }
        }
        Err(err) => {
            error!("Failed to load ROM: {}", err);
            process::exit(1);
        }
    }
}
