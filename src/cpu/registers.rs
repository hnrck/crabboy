use std::fmt;

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

    fn set_flags(&mut self, value: u8) {
        self.z = (value & 0b1000_0000) != 0;
        self.n = (value & 0b0100_0000) != 0;
        self.h = (value & 0b0010_0000) != 0;
        self.c = (value & 0b0001_0000) != 0;
    }
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
        }
    }
}