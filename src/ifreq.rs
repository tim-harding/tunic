use nix::libc::{self, IFF_TUN};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ifreq {
    pub name: [libc::c_char; libc::IFNAMSIZ],
    pub flags: libc::c_short,
}

impl Default for Ifreq {
    fn default() -> Self {
        Self {
            name: Default::default(),
            flags: IFF_TUN as libc::c_short,
        }
    }
}
