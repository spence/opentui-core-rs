//! Raw FFI bindings to the sst/opentui Zig core library.
//!
//! This crate is not intended for direct use. Use `opentui-core` instead.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::all)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
