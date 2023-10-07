use std::fs;

#[derive(Clone)]
pub struct Memory {
    pub(crate) data: Vec<u8>,
}

pub(crate) trait MemoryTrait {
    fn load(&mut self, path: &str) -> Result<(), std::io::Error>;
}

impl MemoryTrait for Memory {
    fn load(&mut self, path: &str) -> Result<(), std::io::Error> {
        let rom_data = fs::read(path)?;
        self.data = rom_data;
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub enum MemorySection {
    Rom,
    VRam,
    ExternalRam,
    InternalRam,
    Oam,
    IoPorts,
    HRam,
}

impl MemorySection {
    pub fn size(&self) -> usize {
        match self {
            MemorySection::Rom => 0x8000,
            MemorySection::VRam => 0x2000,
            MemorySection::ExternalRam => 0x2000,
            MemorySection::InternalRam => 0x2000,
            MemorySection::Oam => 0xA0,
            MemorySection::IoPorts => 0x80,
            MemorySection::HRam => 0x80,
        }
    }

    pub fn range(&self) -> (u16, u16) {
        let start = match self {
            MemorySection::Rom => 0x0000,
            MemorySection::VRam => 0x8000,
            MemorySection::ExternalRam => 0xA000,
            MemorySection::InternalRam => 0xC000,
            MemorySection::Oam => 0xFE00,
            MemorySection::IoPorts => 0xFF00,
            MemorySection::HRam => 0xFF80,
        };

        let end = start + (self.size() as u16 - 1);
        (start, end)
    }

    pub fn contains(&self, addr: u16) -> bool {
        let (start, end) = self.range();
        addr >= start && addr <= end
    }
}