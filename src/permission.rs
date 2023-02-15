mod rwx;

use self::rwx::Rwx;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Permission(u32);

impl Permission {
    pub fn new(owner: Rwx, group: Rwx, others: Rwx) -> Self {
        Self(owner.into_inner() << 6 | group.into_inner() << 3 | others.into_inner())
    }

    pub fn others(&self) -> Rwx {
        self.0.into()
    }

    pub fn set_others(&mut self, others: Rwx) {
        self.0 = self.0 & 0b111_111_000 | others.into_inner();
    }

    pub fn group(&self) -> Rwx {
        (self.0 >> 3).into()
    }

    pub fn set_group(&mut self, group: Rwx) {
        self.0 = self.0 & 0b111_000_111 | (group.into_inner() << 3);
    }

    pub fn owner(&self) -> Rwx {
        (self.0 >> 6).into()
    }

    pub fn set_owner(&mut self, owner: Rwx) {
        self.0 = self.0 & 0b000_111_111 | (owner.into_inner() << 6);
    }

    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl From<u32> for Permission {
    fn from(value: u32) -> Self {
        Self(value & 0b111_111_111)
    }
}

impl From<Permission> for u32 {
    fn from(value: Permission) -> Self {
        value.0
    }
}
