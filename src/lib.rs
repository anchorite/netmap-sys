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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
