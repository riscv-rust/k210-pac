#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Source Priority Register"]
    pub priority: [PRIORITY; 1024],
    #[doc = "0x1000 - Interrupt Pending Register"]
    pub pending: [PENDING; 32],
    _reserved0: [u8; 3968usize],
    #[doc = "0x2000 - Target Interrupt Enables"]
    pub target_enables: [TARGET_ENABLES; 4],
    _reserved1: [u8; 203415040usize],
    #[doc = "0xc200000 - Target Configuration"]
    pub targets0: TARGETS,
    _reserved2: [u8; 4088usize],
    #[doc = "0xc201000 - Target Configuration"]
    pub targets1: TARGETS,
    _reserved3: [u8; 4088usize],
    #[doc = "0xc202000 - Target Configuration"]
    pub targets2: TARGETS,
    _reserved4: [u8; 4088usize],
    #[doc = "0xc203000 - Target Configuration"]
    pub targets3: TARGETS,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct TARGET_ENABLES {
    #[doc = "0x00 - Interrupt Enable Register"]
    pub enable: [self::target_enables::ENABLE; 32],
}
#[doc = r" Register block"]
#[doc = "Target Interrupt Enables"]
pub mod target_enables;
#[doc = r" Register block"]
#[repr(C)]
pub struct TARGETS {
    #[doc = "0x00 - Priority Threshold Register"]
    pub threshold: self::targets::THRESHOLD,
    #[doc = "0x04 - Claim/Complete Register"]
    pub claim: self::targets::CLAIM,
}
#[doc = r" Register block"]
#[doc = "Target Configuration"]
pub mod targets;
#[doc = "Interrupt Source Priority Register"]
pub struct PRIORITY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Source Priority Register"]
pub mod priority;
#[doc = "Interrupt Pending Register"]
pub struct PENDING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Pending Register"]
pub mod pending;
