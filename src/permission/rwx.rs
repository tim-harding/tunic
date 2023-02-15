#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Rwx(u32);

impl Rwx {
    pub const NONE: Self = Self(0);
    pub const X: Self = Self(0o1);
    pub const W: Self = Self(0o2);
    pub const WX: Self = Self(0o3);
    pub const R: Self = Self(0o4);
    pub const RX: Self = Self(0o5);
    pub const RW: Self = Self(0o6);
    pub const RWX: Self = Self(0o7);

    pub fn execute(&self) -> bool {
        self.0 & 0b1 == 0b1
    }

    pub fn set_execute(&mut self, execute: bool) {
        self.0 |= execute as u32;
    }

    pub fn write(&self) -> bool {
        self.0 & 0b10 == 0b10
    }

    pub fn set_write(&mut self, write: bool) {
        self.0 |= (write as u32) << 1;
    }

    pub fn read(&self) -> bool {
        self.0 & 0b100 == 0b100
    }

    pub fn set_read(&mut self, read: bool) {
        self.0 |= (read as u32) << 2;
    }

    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl From<u32> for Rwx {
    fn from(value: u32) -> Self {
        Self(value & 0b111)
    }
}

impl From<Rwx> for u32 {
    fn from(value: Rwx) -> Self {
        value.0
    }
}
