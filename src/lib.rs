//! # Overview
//!
//! This library provides low level bindings to netmap - the fast I/O packet framework
//!
//! It also exports and environment variable `DEP_NETMAP_INCLUDE` with the include path for the
//! netmap framework. It can be used by any dependent crate.
//!
//! # Notes
//!
//! Netmap won't be build, but may be it should in the future.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

unsafe fn netmap_ring_offset(interface: &netmap_if, index: usize) -> ssize_t {
    let offset_in_ring_ofs = std::mem::size_of_val(&*interface.ring_ofs.as_ptr()) * index;
    let ring_offset_position = (*interface).ring_ofs.as_ptr().add(offset_in_ring_ofs);
    *ring_offset_position
}

unsafe fn netmap_ring_by_offset(interface: &netmap_if, offset: ssize_t) -> *mut netmap_ring {
    let base_addr = interface as *const _ as *const u8;
    let netmap_ring_ptr = base_addr.offset(offset as isize);
    netmap_ring_ptr as *mut netmap_ring
}

/// # Safety
///
/// [`index`] *must* be in [0, ni_tx_rings]
pub unsafe fn netmap_txring(interface: *mut netmap_if, index: u16) -> *mut netmap_ring {
    let interface = &*interface;
    netmap_ring_by_offset(interface, netmap_ring_offset(interface, index as usize))
}

/// # Safety
///
/// [`index`] *must* be in [0, ni_rx_rings]
pub unsafe fn netmap_rxring(interface: *mut netmap_if, index: u16) -> *mut netmap_ring {
    let interface = &*interface;
    let index = index as u32 + interface.ni_tx_rings + interface.ni_host_tx_rings;
    netmap_ring_by_offset(interface, netmap_ring_offset(interface, index as usize))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
