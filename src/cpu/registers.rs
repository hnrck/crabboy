use std::fmt;

use crate::cpu::registers::CpuState::Running;

pub struct Flags {
    pub(crate) z: bool,
    pub(crate) n: bool,
    pub(crate) h: bool,
    pub(crate) c: bool,
}

impl fmt::Debug for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Flags")
            .field("z", &format_args!("{}", self.z as u8))
            .field("n", &format_args!("{}", self.n as u8))
            .field("h", &format_args!("{}", self.h as u8))
            .field("c", &format_args!("{}", self.c as u8))
            .finish()
    }
}

impl Flags {
    fn as_u8(&self) -> u8 {
        let mut value: u8 = 0;
        if self.z { value |= 0b1000_0000; }
        if self.n { value |= 0b0100_0000; }
        if self.h { value |= 0b0010_0000; }
        if self.c { value |= 0b0001_0000; }
        value
    }

    fn from_u8(value: u8) -> Self {
        Flags {
            z: (value & 0b1000_0000) != 0,
            n: (value & 0b0100_0000) != 0,
            h: (value & 0b0010_0000) != 0,
            c: (value & 0b0001_0000) != 0,
        }
    }
}

#[derive(PartialEq)]
pub(crate) enum CpuState {
    Running,
    Halted,
    Stopped,
}

pub struct Registers {
    pub(crate) a: u8,
    pub(crate) f: Flags,
    pub(crate) b: u8,
    pub(crate) c: u8,
    pub(crate) d: u8,
    pub(crate) e: u8,
    pub(crate) h: u8,
    pub(crate) l: u8,
    pub(crate) pc: u16,
    pub(crate) sp: u16,
    pub(crate) interrupts_enabled: bool,
    pub(crate) cpu_state: CpuState,
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Registers")
            .field("a", &format_args!("0x{:02X}", self.a))
            .field("f", &self.f)
            .field("b", &format_args!("0x{:02X}", self.b))
            .field("c", &format_args!("0x{:02X}", self.c))
            .field("d", &format_args!("0x{:02X}", self.d))
            .field("e", &format_args!("0x{:02X}", self.e))
            .field("h", &format_args!("0x{:02X}", self.h))
            .field("l", &format_args!("0x{:02X}", self.l))
            .field("pc", &format_args!("0x{:04X}", self.pc))
            .field("sp", &format_args!("0x{:04X}", self.sp))
            .finish()
    }
}

impl Registers {
    pub(crate) fn new() -> Self {
        Registers {
            a: 0,
            f: Flags { z: false, n: false, h: false, c: false },
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            pc: 0x0100,
            sp: 0,
            interrupts_enabled: false,
            cpu_state: Running,
        }
    }

    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f.as_u8() as u16)
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value >> 8) & 0xFF) as u8;
        self.f = Flags::from_u8((value & 0xFF) as u8);
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value >> 8) & 0xFF) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value >> 8) & 0xFF) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value >> 8) & 0xFF) as u8;
        self.l = (value & 0xFF) as u8;
    }

    pub fn enable_interrupts(&mut self) {
        self.interrupts_enabled = true
    }

    pub fn disable_interrupts(&mut self) {
        self.interrupts_enabled = false
    }
}