#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Timeout Range Register"]
    pub torr: TORR,
    #[doc = "0x08 - Current Counter Value Register"]
    pub ccvr: CCVR,
    #[doc = "0x0c - Counter Restart Register"]
    pub crr: CRR,
    #[doc = "0x10 - Interrupt Status Register"]
    pub stat: STAT,
    #[doc = "0x14 - Interrupt Clear Register"]
    pub eoi: EOI,
    _reserved0: [u8; 4usize],
    #[doc = "0x1c - Protection level Register"]
    pub prot_level: PROT_LEVEL,
    _reserved1: [u8; 196usize],
    #[doc = "0xe4 - Component Parameters Register 5"]
    pub comp_param_5: COMP_PARAM_5,
    #[doc = "0xe8 - Component Parameters Register 4"]
    pub comp_param_4: COMP_PARAM_4,
    #[doc = "0xec - Component Parameters Register 3"]
    pub comp_param_3: COMP_PARAM_3,
    #[doc = "0xf0 - Component Parameters Register 2"]
    pub comp_param_2: COMP_PARAM_2,
    #[doc = "0xf4 - Component Parameters Register 1"]
    pub comp_param_1: COMP_PARAM_1,
    #[doc = "0xf8 - Component Version Register"]
    pub comp_version: COMP_VERSION,
    #[doc = "0xfc - Component Type Register"]
    pub comp_type: COMP_TYPE,
}
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Timeout Range Register"]
pub struct TORR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timeout Range Register"]
pub mod torr;
#[doc = "Current Counter Value Register"]
pub struct CCVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Counter Value Register"]
pub mod ccvr;
#[doc = "Counter Restart Register"]
pub struct CRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Restart Register"]
pub mod crr;
#[doc = "Interrupt Status Register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod stat;
#[doc = "Interrupt Clear Register"]
pub struct EOI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear Register"]
pub mod eoi;
#[doc = "Protection level Register"]
pub struct PROT_LEVEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Protection level Register"]
pub mod prot_level;
#[doc = "Component Parameters Register 5"]
pub struct COMP_PARAM_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Component Parameters Register 5"]
pub mod comp_param_5;
#[doc = "Component Parameters Register 4"]
pub struct COMP_PARAM_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Component Parameters Register 4"]
pub mod comp_param_4;
#[doc = "Component Parameters Register 3"]
pub struct COMP_PARAM_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Component Parameters Register 3"]
pub mod comp_param_3;
#[doc = "Component Parameters Register 2"]
pub struct COMP_PARAM_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Component Parameters Register 2"]
pub mod comp_param_2;
#[doc = "Component Parameters Register 1"]
pub struct COMP_PARAM_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Component Parameters Register 1"]
pub mod comp_param_1;
#[doc = "Component Version Register"]
pub struct COMP_VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Component Version Register"]
pub mod comp_version;
#[doc = "Component Type Register"]
pub struct COMP_TYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Component Type Register"]
pub mod comp_type;
