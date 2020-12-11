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

unsafe fn netmap_ring_by_index(interface: *mut netmap_if, index: usize) -> *mut netmap_ring {
    let netmap_ring_offset = (*interface).ring_ofs.as_slice(index + 1)[index];
    let netmap_ring_base = interface as *mut u8;
    netmap_ring_base.offset(netmap_ring_offset as isize) as *mut netmap_ring
}

unsafe fn netmap_rx_ring_index(interface: *mut netmap_if, index: usize) -> usize {
    index + (*interface).ni_tx_rings as usize + (*interface).ni_host_tx_rings as usize
}

/// # Safety
///
/// [`index`] *must* be in [0, ni_tx_rings)
pub unsafe fn netmap_txring(interface: *mut netmap_if, index: u16) -> *mut netmap_ring {
    netmap_ring_by_index(interface, index as usize)
}

/// # Safety
///
/// [`index`] *must* be in [0, ni_rx_rings)
pub unsafe fn netmap_rxring(interface: *mut netmap_if, index: u16) -> *mut netmap_ring {
    let index = netmap_rx_ring_index(interface, index as usize);
    netmap_ring_by_index(interface, index)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
