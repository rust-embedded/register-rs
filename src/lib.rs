#![feature(use_extern_macros)]
#![feature(const_fn)]
#![no_std]

extern crate tock_registers;

pub use tock_registers::{register_bitfields, register_bitmasks};

pub mod cpu;

pub mod mmio {
    pub use tock_registers::registers::*;
}
