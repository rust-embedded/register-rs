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

#[repr(C)]
struct Dummy {
    _0: u32,
    _1: u32,
}

fn main() {
    let stack_mem: Dummy = Dummy { _0: 11, _1: 12 };

    let regs = &stack_mem as *const _ as *const RegisterBlock;

    println!("{}", unsafe { (*regs).SYSTMR_HI.get() });
}
