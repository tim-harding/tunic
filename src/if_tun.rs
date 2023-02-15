//! This file is a port of
//! https://github.com/torvalds/linux/blob/master/include/uapi/linux/if_tun.h

use crate::Ifreq;
use nix::{ioctl_none, ioctl_read, ioctl_write_int, ioctl_write_ptr, libc};

ioctl_write_ptr!(tun_attach_filter, b'T', 213, libc::sock_fprog);
ioctl_write_ptr!(tun_detach_filter, b'T', 214, libc::sock_fprog);

ioctl_write_ptr!(tun_set_if_index, b'T', 218, libc::c_uint);
ioctl_write_ptr!(tun_set_offload, b'T', 208, libc::c_uint);
ioctl_write_ptr!(tun_set_ttx_filter, b'T', 209, libc::c_uint);

ioctl_read!(tun_get_filter, b'T', 219, libc::sock_fprog);

ioctl_read!(tun_get_features, b'T', 207, libc::c_uint);
ioctl_read!(tun_get_iff, b'T', 210, libc::c_uint);

ioctl_none!(tun_get_dev_net_ns, b'T', 227);

ioctl_write_ptr!(tun_set_iff, b'T', 202, Ifreq);

ioctl_write_int!(tun_set_nocsum, b'T', 200);
ioctl_write_int!(tun_set_debug, b'T', 201);
ioctl_write_int!(tun_set_persist, b'T', 203);
ioctl_write_int!(tun_set_owner, b'T', 204);
ioctl_write_int!(tun_set_link, b'T', 205);
ioctl_write_int!(tun_set_group, b'T', 206);
ioctl_write_int!(tun_set_sndbuf, b'T', 212);
ioctl_write_int!(tun_set_vnethdrsz, b'T', 216);
ioctl_write_int!(tun_set_queue, b'T', 217);
ioctl_write_int!(tun_set_vnetle, b'T', 220);
ioctl_write_int!(tun_set_vnetbe, b'T', 222);
ioctl_write_int!(tun_set_carrier, b'T', 226);

ioctl_read!(tun_get_vnethdrsz, b'T', 215, libc::c_int);
ioctl_read!(tun_get_vnetle, b'T', 221, libc::c_int);
ioctl_read!(tun_get_vnetbe, b'T', 223, libc::c_int);
ioctl_read!(tun_set_steeringebpf, b'T', 224, libc::c_int);
ioctl_read!(tun_set_filterebpf, b'T', 225, libc::c_int);
ioctl_read!(tun_get_sndbuf, b'T', 211, libc::c_int);
