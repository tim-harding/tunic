mod if_tun;
mod ifreq;
mod permission;

use if_tun::{tun_set_group, tun_set_iff, tun_set_owner, tun_set_persist};
use ifreq::Ifreq;
use nix::{
    errno::Errno,
    fcntl::{open, OFlag},
    libc::{IFF_NO_PI, IFF_TAP, IFF_TUN, IFNAMSIZ},
    sys::stat::Mode,
};
use permission::Permission;
use std::{
    error::Error,
    ffi::{CStr, CString, IntoStringError},
    fmt::Display,
    mem::transmute,
    os::fd::RawFd,
};

// TODO: Support multiqueue TUN/TAP

#[derive(Debug, Clone)]
pub struct Tunic {
    fd: RawFd,
}

impl Tunic {
    pub fn unset_persist(&self) -> Result<(), TunicError> {
        unsafe { tun_set_persist(self.fd, false as u64) }?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct TunicBuilder {
    ifr: Ifreq,
    persist: bool,
    owner: Option<Permission>,
    group: Option<Permission>,
}

impl TunicBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn name(mut self, name: &CStr) -> Result<Self, TunicError> {
        let name = name.to_bytes();
        let name = unsafe { transmute::<_, &[i8]>(name) };
        if name.len() > IFNAMSIZ {
            Err(TunicError)
        } else {
            self.ifr.name[..name.len()].clone_from_slice(name);
            Ok(self)
        }
    }

    pub fn tap(mut self) -> Self {
        self.ifr.flags &= !IFF_TUN as i16;
        self.ifr.flags |= IFF_TAP as i16;
        self
    }

    pub fn no_packet_info(mut self) -> Self {
        self.ifr.flags |= IFF_NO_PI as i16;
        self
    }

    pub fn persist(mut self) -> Self {
        self.persist = true;
        self
    }

    pub fn owner(mut self, permission: Permission) -> Self {
        self.owner = Some(permission);
        self
    }

    pub fn group(mut self, permission: Permission) -> Self {
        self.group = Some(permission);
        self
    }

    pub fn build(mut self) -> Result<(Tunic, String), TunicError> {
        let fd = open("/dev/net/tun", OFlag::O_RDWR, Mode::empty())?;
        unsafe { tun_set_iff(fd, &self.ifr) }?;

        if self.persist {
            unsafe { tun_set_persist(fd, true as u64) }?;
        }

        if let Some(owner) = self.owner {
            unsafe { tun_set_owner(fd, owner.into_inner() as u64) }?;
        }

        if let Some(group) = self.group {
            unsafe { tun_set_group(fd, group.into_inner() as u64) }?;
        }

        let name = self.ifr.name.as_mut_ptr();
        let name = unsafe { CString::from_raw(name) }.into_string()?;
        Ok((Tunic { fd }, name))
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TunicError;

impl Error for TunicError {}

impl Display for TunicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to open TUN interface")
    }
}

impl From<Errno> for TunicError {
    fn from(_: Errno) -> Self {
        Self
    }
}

impl From<IntoStringError> for TunicError {
    fn from(_: IntoStringError) -> Self {
        Self
    }
}
