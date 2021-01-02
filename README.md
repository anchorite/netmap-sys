# netmap-sys

## Overview

**Warning**: This is experimental library with which I learn bindgen in the context of netmap's
libnetmap library.

This library provides low level bindings to netmap - the fast I/O packet framework

It also exports and environment variable `DEP_NETMAP_INCLUDE` with the include path for the
netmap framework. It can be used by any dependent crate.

## Notes

Netmap won't be build, but may be it should in the future.
