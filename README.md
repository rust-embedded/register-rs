[![crates.io](https://img.shields.io/crates/d/register.svg)](https://crates.io/crates/register)
[![crates.io](https://img.shields.io/crates/v/register.svg)](https://crates.io/crates/register)

# register-rs

Unified interface for type-safe MMIO and CPU register access in Rust.

## Outline

- [Usage](#usage)
  - [Defining MMIO registers](#defining-mmio-registers)
    - [The Deref pattern for drivers](#the-deref-pattern-for-drivers)
  - [Defining CPU registers](#defining-cpu-registers)
- [License](#license)
- [Contribution](#contribution)

## Usage

This crate uses the tock-register-interface, please refer to their
[Readme](https://github.com/tock/tock/tree/master/libraries/tock-register-interface) for the whole
API. Make sure to check the `register_structs!` explanation, especially the fact that you must
[explicitly annotate gaps].

[explicitly annotate gaps]: https://github.com/tock/tock/tree/master/libraries/tock-register-interface#defining-registers

### Defining MMIO registers

```rust
use register::{mmio::*, register_bitfields, register_structs};

register_bitfields! {
    u32,

    GPFSEL1 [
        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            TXD0 = 0b100
        ],

        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            RXD0 = 0b100
        ]
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x000 => GPFSEL1: ReadWrite<u32, GPFSEL1::Register>),
        (0x004 => SYSTMR_HI: ReadOnly<u32>),
        (0x008 => @END),
    }
}

fn main() {
    let regs = 0x1337_0000 as *const RegisterBlock;

    unsafe { (*regs).SYSTMR_HI.get() };
}
```

#### The Deref pattern for drivers

The `MMIO` part of this crate can and will often be used for implementing device drivers. In this
case, you might find the `Deref pattern` useful for referencing your registers. It alleviates you
from manually dereferencing each time a register access is due, and also encapsulates the `unsafe`
keyword.

Here is an example (extending the code snippet from above):

```rust
register_bitfields! {
    u32,

    // omitted
}

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        // omitted
    }
}

pub struct DeviceDriver {
    base_addr: usize,
}

impl ops::Deref for DeviceDriver {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl DeviceDriver {
    pub fn new(base_addr: usize) -> Self {
        DeviceDriver { base_addr }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base_addr as *const _
    }

    fn do_something(&self) -> u32 {
        self.GPFSEL1.set(0x1337);
        self.SYSTMR_HI.get()
    }
}
```

### Defining CPU registers

For CPU registers, you only need to implement the respective read/write trait. All other methods are
provided by default.

```rust
#![feature(llvm_asm)]

use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u32,
    pub CNTP_CTL_EL0 [
        /// Enables the timer.
        ENABLE        OFFSET(0)  NUMBITS(1) [],

        /// Timer interrupt mask bit.
        IMASK         OFFSET(1)  NUMBITS(1) [],

        /// The status of the timer.
        ISTATUS       OFFSET(2)  NUMBITS(1) []
    ]
}

struct Reg;

impl RegisterReadWrite<u32, CNTP_CTL_EL0::Register> for Reg {
    /// Reads the raw bits of the CPU register.
    #[inline(always)]
    fn get(&self) -> u32 {
        let reg;
        unsafe {
            llvm_asm!("mrs $0, CNTP_CTL_EL0" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    /// Writes raw bits to the CPU register.
    #[inline(always)]
    fn set(&self, value: u32) {
        unsafe {
            llvm_asm!("msr CNTP_CTL_EL0, $0" :: "r"(value) :: "volatile");
        }
    }
}

static CNTP_CTL_EL0: Reg = Reg {};

fn main() {
    CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::SET + CNTP_CTL_EL0::IMASK::SET);
}

```

## Testing

In order to use this crate in `custom_test_frameworks` environments, please set the
`no_std_unit_tests` feature flag in your dependency section.

Otherwise, you might encounter errors like the following:

```console
error[E0463]: can't find crate for `test`
  --> src/bsp/driver/bcm/bcm2xxx_gpio.rs:52:1
   |
52 | / register_structs! {
53 | |     #[allow(non_snake_case)]
54 | |     RegisterBlock {
55 | |         (0x00 => GPFSEL0: ReadWrite<u32>),
...  |
66 | |     }
67 | | }
   | |_^ can't find crate
   |
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error: aborting due to previous error
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
