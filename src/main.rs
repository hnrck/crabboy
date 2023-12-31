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

/// Retrieve the value of the `CARGO_PKG_VERSION` environment variable.
const OPT_SETUP: &'static str = env!("CARGO_PKG_VERSION");

/// Retrieve the value of the `CARGO_PKG_AUTHORS` environment variable.
const AUTHOR_SETUP: &'static str = env!("CARGO_PKG_AUTHORS");

/// Retrieve the value of the `CARGO_PKG_DESCRIPTION` environment variable.
const ABOUT_SETUP: &'static str = env!("CARGO_PKG_DESCRIPTION");

/// Command-line options
#[derive(Parser)]
#[clap(version = OPT_SETUP, author = AUTHOR_SETUP, about = ABOUT_SETUP)]
struct Opt {
    /// The command line argument for specifying the path to a ROM file.
    #[clap(short = 'r', long = "rom", required = true)]
    rom: String,
}

/// Initializes the logger with debug level filtering if the `debug_assertions` feature is enabled.
///
/// This function should be called once at the start of the application to initialize the logger
/// and set the desired log level.
#[cfg(debug_assertions)]
fn init_logger() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
}

/// Initializes the logger based on the current configuration.
///
/// The logger is initialized with a log level of `Info` when the program is not compiled with assertions enabled.
#[cfg(not(debug_assertions))]
fn init_logger() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
}

/// Runs the ROM on the emulator.
///
/// # Arguments
///
/// * `rom` - The ROM to be loaded and executed.
///
/// # Examples
///
/// ```rust
/// let rom = Rom::new("game.rom");
/// run_rom(rom);
/// ```
///
/// # Panics
///
/// This function will panic if there is an error loading or validating the ROM.
///
/// # Safety
///
/// This function assumes that the ROM has been loaded and validated successfully.
fn run_rom(rom: Rom) {
    debug!("ROM loaded and validated successfully");
    rom.print_info();
    let mut mmu = MMU::new().with_rom(rom);
    let mut cpu = CPU::new();
    loop {
        // TODO(henrick) Step error handling
        cpu.step(&mut mmu);
    }
}

/// Main
fn main() {
    init_logger();
    let opt = Opt::parse();
    match Rom::from_path(&opt.rom) {
        Ok(rom) => {
            if rom.validate() {
                run_rom(rom);
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