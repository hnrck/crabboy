pub struct Flags {
    pub(crate) z: bool,
    pub(crate) n: bool,
    pub(crate) h: bool,
    pub(crate) c: bool,
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
            pc: 0,
            sp: 0,
        }
    }
}