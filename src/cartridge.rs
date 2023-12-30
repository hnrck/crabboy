use log::info;

pub struct CartridgeHeader {
    pub(crate) title: String,
    pub(crate) manufacturer_code: Option<String>,
    pub(crate) cgb_flag: Option<CGBFlag>,
    pub(crate) new_licensee_code: String,
    pub(crate) sgb_flag: SGBFlag,
    pub(crate) cartridge_type: CartridgeType,
    pub(crate) rom_size: RomSize,
    pub(crate) ram_size: RamSize,
    pub(crate) destination_code: DestinationCode,
    pub(crate) old_licensee_code: u8,
    pub(crate) rom_version: u8,
    pub(crate) header_checksum: u8,
    pub(crate) global_checksum: u16,
}

#[derive(Debug)]
pub(crate) enum CGBFlag {
    Supported = 0x80,
    Required = 0xc0,
}

impl CGBFlag {
    pub(crate) fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x80 => Some(CGBFlag::Supported),
            0xC0 => Some(CGBFlag::Required),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum SGBFlag {
    NotSupported = 0x00,
    Supported = 0x03,
}

impl SGBFlag {
    pub(crate) fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(SGBFlag::NotSupported),
            0x03 => Some(SGBFlag::Supported),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum CartridgeType {
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
    pub(crate) fn from_u8(value: u8) -> Option<Self> {
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
pub(crate) enum RomSize {
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
    pub(crate) fn from_u8(value: u8) -> Option<Self> {
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
pub(crate) enum RamSize {
    None = 0x00,
    Kb2 = 0x01,
    Kb8 = 0x02,
    Kb32 = 0x03,
}

impl RamSize {
    pub(crate) fn from_u8(value: u8) -> Option<Self> {
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
pub(crate) enum DestinationCode {
    Japanese = 0x00,
    NonJapanese = 0x01,
}

impl DestinationCode {
    pub(crate) fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(DestinationCode::Japanese),
            0x01 => Some(DestinationCode::NonJapanese),
            _ => None,
        }
    }
}

impl CartridgeHeader {
    pub(crate) fn print_info(&self) {
        info!("ROM Header Information:");
        info!("-----------------------");
        info!("Title: {}", self.title);
        if let Some(code) = &self.manufacturer_code {
            info!("Manufacturer Code: {}", code);
        }
        info!("CGB Support: {:?}", self.cgb_flag);
        info!("New Licensee Code: {}", self.new_licensee_code);
        info!("SGB Support: {:?}", self.sgb_flag);
        info!("Cartridge Type: {:?}", self.cartridge_type);
        info!("ROM Size: {:?}", self.rom_size);
        info!("RAM Size: {:?}", self.ram_size);
        info!("Destination Code: {:?}", self.destination_code);
        info!("Old Licensee Code: {}", self.old_licensee_code);
        info!("ROM Version: {}", self.rom_version);
        info!("Header Checksum: {:02X}", self.header_checksum);
        info!("Global Checksum: {:04X}", self.global_checksum);
    }
}