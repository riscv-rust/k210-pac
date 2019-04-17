#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel cluster: load_count, current_value, control, eoi and intr_stat registers"]
    pub channel: [CHANNEL; 4],
    _reserved0: [u8; 80usize],
    #[doc = "0xa0 - Interrupt Status Register"]
    pub intr_stat: INTR_STAT,
    #[doc = "0xa4 - Interrupt Clear Register"]
    pub eoi: EOI,
    #[doc = "0xa8 - Raw Interrupt Status Register"]
    pub raw_intr_stat: RAW_INTR_STAT,
    #[doc = "0xac - Component Version Register"]
    pub comp_version: COMP_VERSION,
    #[doc = "0xb0 - Load Count2 Register"]
    pub load_count2: [LOAD_COUNT2; 4],
}
#[doc = r" Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Load Count Register"]
    pub load_count: self::channel::LOAD_COUNT,
    #[doc = "0x04 - Current Value Register"]
    pub current_value: self::channel::CURRENT_VALUE,
    #[doc = "0x08 - Control Register"]
    pub control: self::channel::CONTROL,
    #[doc = "0x0c - Interrupt Clear Register"]
    pub eoi: self::channel::EOI,
    #[doc = "0x10 - Interrupt Status Register"]
    pub intr_stat: self::channel::INTR_STAT,
}
#[doc = r" Register block"]
#[doc = "Channel cluster: load_count, current_value, control, eoi and intr_stat registers"]
pub mod channel;
#[doc = "Interrupt Status Register"]
pub struct INTR_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod intr_stat;
#[doc = "Interrupt Clear Register"]
pub struct EOI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear Register"]
pub mod eoi;
#[doc = "Raw Interrupt Status Register"]
pub struct RAW_INTR_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw Interrupt Status Register"]
pub mod raw_intr_stat;
#[doc = "Component Version Register"]
pub struct COMP_VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Component Version Register"]
pub mod comp_version;
#[doc = "Load Count2 Register"]
pub struct LOAD_COUNT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Load Count2 Register"]
pub mod load_count2;
