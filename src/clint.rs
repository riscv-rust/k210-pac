#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Hart software interrupt register"]
    pub msip: [MSIP; 2],
    _reserved0: [u8; 16376usize],
    #[doc = "0x4000 - Hart time comparator register"]
    pub mtimecmp: [MTIMECMP; 2],
    _reserved1: [u8; 32744usize],
    #[doc = "0xbff8 - Timer register"]
    pub mtime: MTIME,
}
#[doc = "Hart software interrupt register"]
pub struct MSIP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hart software interrupt register"]
pub mod msip;
#[doc = "Hart time comparator register"]
pub struct MTIMECMP {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Hart time comparator register"]
pub mod mtimecmp;
#[doc = "Timer register"]
pub struct MTIME {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Timer register"]
pub mod mtime;
