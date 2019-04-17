#[doc = "Load Count Register"]
pub struct LOAD_COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Load Count Register"]
pub mod load_count;
#[doc = "Current Value Register"]
pub struct CURRENT_VALUE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Value Register"]
pub mod current_value;
#[doc = "Control Register"]
pub struct CONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod control;
#[doc = "Interrupt Clear Register"]
pub struct EOI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear Register"]
pub mod eoi;
#[doc = "Interrupt Status Register"]
pub struct INTR_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod intr_stat;
