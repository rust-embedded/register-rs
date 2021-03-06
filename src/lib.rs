// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2021 by the author(s)
//
// Author(s):
//   - Tock Project Developers <tock-dev@googlegroups.com>
//   - Andre Richter <andre.o.richter@gmail.com>

//! Unified interface for type-safe MMIO and CPU register access.
//!
//! Based on [tock-register-interface].
//!
//! [tock-register-interface]: https://github.com/tock/tock/tree/master/libraries/tock-register-interface

#![no_std]

pub use tock_registers::{
    register_bitfields, register_bitmasks, register_fields, register_structs,
    registers::{
        Field, FieldValue, InMemoryRegister, IntLike, LocalRegisterCopy, RegisterLongName,
        TryFromValue,
    },
    test_fields,
};

pub mod cpu;

/// MMIO traits.
pub mod mmio {
    pub use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
}
