use std::fs;
use crate::types::Memory;
use crate::types::MemoryTrait;

use crate::cartridge::CartridgeType;
use crate::cartridge::CartridgeHeader;
use crate::cartridge::RamSize;
use crate::cartridge::RomSize;
use crate::cartridge::CGBFlag;
use crate::cartridge::SGBFlag;
use crate::cartridge::DestinationCode;

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

    pub(crate) fn validate(&self) -> bool {
        self.memory.data[0x104..0x134] == Self::NINTENDO_LOGO
    }

    pub fn print_info(&self) {
        println!("ROM Information:");
        println!("----------------");
        self.header.print_info();
        println!("----------------");
    }
}