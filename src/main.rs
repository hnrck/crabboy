use std::fs;

use clap::Parser;

#[derive(Clone)]
struct Memory {
    data: Vec<u8>,
}

trait MemoryTrait {
    fn load(&mut self, path: &str) -> Result<(), std::io::Error>;
}

impl MemoryTrait for Memory {
    fn load(&mut self, path: &str) -> Result<(), std::io::Error> {
        let rom_data = fs::read(path)?;
        self.data = rom_data;
        Ok(())
    }
}

struct CartridgeHeader {
    title: String,
    manufacturer_code: Option<String>,
    cgb_flag: Option<CGBFlag>,
    new_licensee_code: String,
    sgb_flag: SGBFlag,
    cartridge_type: CartridgeType,
    rom_size: RomSize,
    ram_size: RamSize,
    destination_code: DestinationCode,
    old_licensee_code: u8,
    rom_version: u8,
    header_checksum: u8,
    global_checksum: u16,
}

#[derive(Debug)]
enum CGBFlag {
    Supported = 0x80,
    Required = 0xc0,
}

impl CGBFlag {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x80 => Some(CGBFlag::Supported),
            0xC0 => Some(CGBFlag::Required),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum SGBFlag {
    NotSupported = 0x00,
    Supported = 0x03,
}

impl SGBFlag {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(SGBFlag::NotSupported),
            0x03 => Some(SGBFlag::Supported),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum CartridgeType {
    RomOnly = 0x00,
    Mbc1 = 0x01,
    Mbc1Ram = 0x02,
    Mbc1RamBattery = 0x03,
    Mbc2 = 0x05,
    Mbc2Battery = 0x06,
    RomRam = 0x08,
    RomRamBattery = 0x09,
    Mmm01 = 0x0B,
    Mbc5RumbleRam = 0x1D,
    Mmm01Ram = 0x0C,
    Mmm01RamBattery = 0x0D,
    Mbc3TimerBattery = 0x0F,
    Mbc3TimerRamBattery = 0x10,
    Mbc3 = 0x11,
    Mbc3Ram = 0x12,
    Mbc3RamBattery = 0x13,
    Mbc4 = 0x15,
    Mbc4Ram = 0x16,
    Mbc4RamBattery = 0x17,
    Mbc5 = 0x19,
    Mbc5Ram = 0x1A,
    Mbc5RamBattery = 0x1B,
    Mbc5Rumble = 0x1C,
    Mbc5RumbleRamBattery = 0x1E,
    PocketCamera = 0xFC,
    BandaiTama5 = 0xFD,
    Huc3 = 0xFE,
    Huc1RamBattery = 0xFF,
}

impl CartridgeType {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(CartridgeType::RomOnly),
            0x01 => Some(CartridgeType::Mbc1),
            0x02 => Some(CartridgeType::Mbc1Ram),
            0x03 => Some(CartridgeType::Mbc1RamBattery),
            0x05 => Some(CartridgeType::Mbc2),
            0x06 => Some(CartridgeType::Mbc2Battery),
            0x08 => Some(CartridgeType::RomRam),
            0x09 => Some(CartridgeType::RomRamBattery),
            0x0B => Some(CartridgeType::Mmm01),
            0x1D => Some(CartridgeType::Mbc5RumbleRam),
            0x0C => Some(CartridgeType::Mmm01Ram),
            0x0D => Some(CartridgeType::Mmm01RamBattery),
            0x0F => Some(CartridgeType::Mbc3TimerBattery),
            0x10 => Some(CartridgeType::Mbc3TimerRamBattery),
            0x11 => Some(CartridgeType::Mbc3),
            0x12 => Some(CartridgeType::Mbc3Ram),
            0x13 => Some(CartridgeType::Mbc3RamBattery),
            0x15 => Some(CartridgeType::Mbc4),
            0x16 => Some(CartridgeType::Mbc4Ram),
            0x17 => Some(CartridgeType::Mbc4RamBattery),
            0x19 => Some(CartridgeType::Mbc5),
            0x1A => Some(CartridgeType::Mbc5Ram),
            0x1B => Some(CartridgeType::Mbc5RamBattery),
            0x1C => Some(CartridgeType::Mbc5Rumble),
            0x1E => Some(CartridgeType::Mbc5RumbleRamBattery),
            0xFC => Some(CartridgeType::PocketCamera),
            0xFD => Some(CartridgeType::BandaiTama5),
            0xFE => Some(CartridgeType::Huc3),
            0xFF => Some(CartridgeType::Huc1RamBattery),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum RomSize {
    Kb32 = 0x00,
    Kb64 = 0x01,
    Kb128 = 0x02,
    Kb256 = 0x03,
    Kb512 = 0x04,
    Mb1 = 0x05,
    Mb2 = 0x06,
    Mb4 = 0x07,
    Mb1_1 = 0x52,
    Mb1_2 = 0x53,
    Mb1_5 = 0x54,
}

impl RomSize {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(RomSize::Kb32),
            0x01 => Some(RomSize::Kb64),
            0x02 => Some(RomSize::Kb128),
            0x03 => Some(RomSize::Kb256),
            0x04 => Some(RomSize::Kb512),
            0x05 => Some(RomSize::Mb1),
            0x06 => Some(RomSize::Mb2),
            0x07 => Some(RomSize::Mb4),
            0x52 => Some(RomSize::Mb1_1),
            0x53 => Some(RomSize::Mb1_2),
            0x54 => Some(RomSize::Mb1_5),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
enum RamSize {
    None = 0x00,
    Kb2 = 0x01,
    Kb8 = 0x02,
    Kb32 = 0x03,
}

impl RamSize {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(RamSize::None),
            0x01 => Some(RamSize::Kb2),
            0x02 => Some(RamSize::Kb8),
            0x03 => Some(RamSize::Kb32),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum DestinationCode {
    Japanese = 0x00,
    NonJapanese = 0x01,
}

impl DestinationCode {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(DestinationCode::Japanese),
            0x01 => Some(DestinationCode::NonJapanese),
            _ => None,
        }
    }
}

impl CartridgeHeader {
    fn print_info(&self) {
        println!("ROM Header Information:");
        println!("-----------------------");
        println!("Title: {}", self.title);
        if let Some(code) = &self.manufacturer_code {
            println!("Manufacturer Code: {}", code);
        }
        println!("CGB Support: {:?}", self.cgb_flag);
        println!("New Licensee Code: {}", self.new_licensee_code);
        println!("SGB Support: {:?}", self.sgb_flag);
        println!("Cartridge Type: {:?}", self.cartridge_type);
        println!("ROM Size: {:?}", self.rom_size);
        println!("RAM Size: {:?}", self.ram_size);
        println!("Destination Code: {:?}", self.destination_code);
        println!("Old Licensee Code: {}", self.old_licensee_code);
        println!("ROM Version: {}", self.rom_version);
        println!("Header Checksum: {:02X}", self.header_checksum);
        println!("Global Checksum: {:04X}", self.global_checksum);
    }
}

struct Rom {
    memory: Memory,
    header: CartridgeHeader,
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
        let memory = Memory { data: fs::read(path)? };
        let rom = Rom {
            memory: memory.clone(),
            header: Rom::extract_header(&memory).expect("Cannot parse header"),
        };
        Ok(rom)
    }

    fn extract_header(memory: &Memory) -> Option<CartridgeHeader> {
        // TODO(henrick) handle title size:
        // "Title of the game in UPPER CASE ASCII. If it is less than 16 characters then the
        //  remaining bytes are filled with 00's. When inventing the CGB, Nintendo has reduced the
        //  length of this area to 15 characters, and some months later they had the fantastic idea
        //  to reduce it to 11 characters only. The new meaning of the ex-title bytes is described
        //  below."
        let cartridge_type = CartridgeType::from_u8(memory.data[0x0147]).expect("Unable to extract cartridge type");
        let ram_size = RamSize::from_u8(memory.data[0x0149]).expect("Cannot extract RAM size");
        if let CartridgeType::Mbc2 = cartridge_type {
            if ram_size != RamSize::None {
                return None;
            }
        }
        Some(CartridgeHeader {
            title: String::from_utf8_lossy(&memory.data[0x0134..0x0143]).trim_end_matches('\0').to_string(),
            manufacturer_code: String::from_utf8(memory.data[0x013F..=0x0142].to_vec()).ok(),
            cgb_flag: CGBFlag::from_u8(memory.data[0x0143]),
            new_licensee_code: String::from_utf8(memory.data[0x0144..0x0145].to_vec()).unwrap_or_else(|_| String::from("")),
            sgb_flag: SGBFlag::from_u8(memory.data[0x0146]).expect("Cannot parse SGB flag"),
            cartridge_type: CartridgeType::from_u8(memory.data[0x0147]).expect("Unable to extract cartridge type"),
            rom_size: RomSize::from_u8(memory.data[0x0148]).expect("Cannot extract ROM size"),
            ram_size: RamSize::from_u8(memory.data[0x0149]).expect("Cannot extract RAM size"),
            destination_code: DestinationCode::from_u8(memory.data[0x014a]).expect("Cannot extract destination code"),
            old_licensee_code: memory.data[0x014b],
            rom_version: memory.data[0x014c],
            header_checksum: memory.data[0x014d],
            global_checksum: ((memory.data[0x014e] as u16) << 8) | (memory.data[0x014f] as u16),
        })
    }

    fn validate(&self) -> bool {
        self.memory.data[0x104..0x134] == Self::NINTENDO_LOGO
    }

    pub fn print_info(&self) {
        println!("ROM Information:");
        println!("----------------");
        self.header.print_info();
        println!("----------------");
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
    rom.print_info();
}
