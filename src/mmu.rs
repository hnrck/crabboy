use crate::rom::Rom;
use crate::types::Memory;
use crate::types::MemorySection;

pub struct MMU {
    rom: Memory,
    vram: Memory,
    external_ram: Memory,
    internal_ram: Memory,
    oam: Memory,
    io_ports: Memory,
    hram: Memory,
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            rom: Memory { data: vec![0; MemorySection::Rom.size()] },
            vram: Memory { data: vec![0; MemorySection::VRam.size()] },
            external_ram: Memory { data: vec![0; MemorySection::ExternalRam.size()] },
            internal_ram: Memory { data: vec![0; MemorySection::InternalRam.size()] },
            oam: Memory { data: vec![0; MemorySection::Oam.size()] },
            io_ports: Memory { data: vec![0; MemorySection::IoPorts.size()] },
            hram: Memory { data: vec![0; MemorySection::HRam.size()] },
        }
    }

    pub(crate) fn with_rom(self, rom: Rom) -> Self {
        MMU {
            rom: rom.memory,
            ..self
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            _ if MemorySection::Rom.contains(addr) => self.rom.data[addr as usize],
            _ if MemorySection::VRam.contains(addr) => self.vram.data[(addr - 0x8000) as usize],
            _ if MemorySection::ExternalRam.contains(addr) => self.external_ram.data[(addr - 0xA000) as usize],
            _ if MemorySection::InternalRam.contains(addr) => self.internal_ram.data[(addr - 0xC000) as usize],
            _ if MemorySection::Oam.contains(addr) => self.oam.data[(addr - 0xFE00) as usize],
            _ if MemorySection::IoPorts.contains(addr) => self.io_ports.data[(addr - 0xFF00) as usize],
            _ if MemorySection::HRam.contains(addr) => self.hram.data[(addr - 0xFF80) as usize],
            _ => 0,
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            _ if MemorySection::Rom.contains(addr) => self.rom.data[addr as usize] = value,
            _ if MemorySection::VRam.contains(addr) => self.vram.data[(addr - 0x8000) as usize] = value,
            _ if MemorySection::ExternalRam.contains(addr) => self.external_ram.data[(addr - 0xA000) as usize] = value,
            _ if MemorySection::InternalRam.contains(addr) => self.internal_ram.data[(addr - 0xC000) as usize] = value,
            _ if MemorySection::Oam.contains(addr) => self.oam.data[(addr - 0xFE00) as usize] = value,
            _ if MemorySection::IoPorts.contains(addr) => self.io_ports.data[(addr - 0xFF00) as usize] = value,
            _ if MemorySection::HRam.contains(addr) => self.hram.data[(addr - 0xFF80) as usize] = value,
            _ => (),
        }
    }
}