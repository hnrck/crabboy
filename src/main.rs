use std::fs;

use clap::Parser;

struct Memory {
    data: [u8; 0xFFFF],
}

trait MemoryTrait {
    fn load(&mut self, path: &str) -> Result<(), std::io::Error>;
}

impl MemoryTrait for Memory {
    fn load(&mut self, path: &str) -> Result<(), std::io::Error> {
        let data = fs::read(path)?;
        let len = data.len().min(self.data.len());
        self.data[..len].copy_from_slice(&data[..len]);
        Ok(())
    }
}


struct Rom {
    memory: Memory,
}

impl MemoryTrait for Rom {
    fn load(&mut self, path: &str) -> Result<(), std::io::Error> {
        self.memory.load(path)
    }
}

impl Rom {
    const NINTENDO_LOGO: [u8; 48] = [
        0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
        0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
        0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
        0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
        0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
        0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E
    ];

    pub fn from_path(path: &str) -> Result<Self, std::io::Error> {
        let mut mem = Memory { data: [0; 0xFFFF] };
        mem.load(path)?;
        Ok(Rom { memory: mem })
    }

    fn validate(&self) -> bool {
        self.memory.data[0x104..0x134] == Self::NINTENDO_LOGO
    }
}


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
}
