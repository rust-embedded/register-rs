extern crate register;

mod deref;

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
    RegisterBlock {
        (0x000 => GPFSEL1: ReadWrite<u32, GPFSEL1::Register>),
        (0x004 => SYSTMR_HI: ReadOnly<u32>),
        (0x008 => @END),
    }
}

fn main() {
    let regs = 0x1337_0000 as *const RegisterBlock;

    unsafe { (*regs).SYSTMR_HI.get() };

    deref::main2();
}
