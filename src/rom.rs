use std::fs;

use log::{error, info};

use crate::cartridge::{CartridgeHeader, RomSize};
use crate::cartridge::CartridgeType;
use crate::cartridge::CGBFlag;
use crate::cartridge::DestinationCode;
use crate::cartridge::RamSize;
use crate::cartridge::SGBFlag;
use crate::types::Memory;
use crate::types::MemoryTrait;

pub struct Rom {
    pub(crate) memory: Memory,
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
        match Rom::extract_header(&memory) {
            Some(header) => Ok(Rom { memory: memory.clone(), header }),
            None => Err(*Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Unable to parse ROM header")))
        }
    }

    fn extract_header(memory: &Memory) -> Option<CartridgeHeader> {
        // TODO(henrick) handle title size:
        // "Title of the game in UPPER CASE ASCII. If it is less than 16 characters then the
        //  remaining bytes are filled with 00's. When inventing the CGB, Nintendo has reduced the
        //  length of this area to 15 characters, and some months later they had the fantastic idea
        //  to reduce it to 11 characters only. The new meaning of the ex-title bytes is described
        //  below."
        let sgb_flag = match SGBFlag::from_u8(memory.data[0x0146]) {
            Some(sgb_flag) => sgb_flag,
            None => {
                error!("Cannot parse SGB flag");
                return None;
            }
        };
        let cartridge_type = match CartridgeType::from_u8(memory.data[0x0147]) {
            Some(cartridge_type) => cartridge_type,
            None => {
                error!("Unable to extract cartridge type");
                return None;
            }
        };
        let rom_size = match RomSize::from_u8(memory.data[0x0148]) {
            Some(rom_size) => rom_size,
            None => {
                error!("Cannot extract ROM size");
                return None;
            }
        };
        let ram_size = match RamSize::from_u8(memory.data[0x0149]) {
            Some(ram_size) => ram_size,
            None => {
                error!("Cannot extract RAM size");
                return None;
            }
        };
        if let CartridgeType::Mbc2 = cartridge_type {
            if ram_size != RamSize::None {
                error!("RAM size for MBC2 type cartridge is not None");
                return None;
            }
        }
        let destination_code = match DestinationCode::from_u8(memory.data[0x014a]) {
            Some(destination_code) => destination_code,
            None => {
                error!("Cannot extract destination code");
                return None;
            }
        };
        Some(CartridgeHeader {
            title: String::from_utf8_lossy(&memory.data[0x0134..0x0143]).trim_end_matches('\0').to_string(),
            manufacturer_code: String::from_utf8(memory.data[0x013F..=0x0142].to_vec()).ok(),
            cgb_flag: CGBFlag::from_u8(memory.data[0x0143]),
            new_licensee_code: String::from_utf8(memory.data[0x0144..0x0145].to_vec()).unwrap_or_else(|_| String::from("")),
            sgb_flag,
            cartridge_type,
            rom_size,
            ram_size,
            destination_code,
            old_licensee_code: memory.data[0x014b],
            rom_version: memory.data[0x014c],
            header_checksum: memory.data[0x014d],
            global_checksum: ((memory.data[0x014e] as u16) << 8) | (memory.data[0x014f] as u16),
        })
    }

    pub(crate) fn validate(&self) -> bool {
        self.memory.data[0x104..0x134] == Self::NINTENDO_LOGO
    }

    pub fn print_info(&self) {
        info!("ROM Information:");
        info!("----------------");
        self.header.print_info();
        info!("----------------");
    }
}