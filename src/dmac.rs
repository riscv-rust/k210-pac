#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ID Register"]
    pub id: ID,
    #[doc = "0x08 - COMPVER Register"]
    pub compver: COMPVER,
    #[doc = "0x10 - Configure Register"]
    pub cfg: CFG,
    #[doc = "0x18 - Channel Enable Register"]
    pub chen: CHEN,
    _reserved0: [u8; 16usize],
    #[doc = "0x30 - Interrupt Status Register"]
    pub intstatus: INTSTATUS,
    #[doc = "0x38 - Common Interrupt Clear Register"]
    pub com_intclear: COM_INTCLEAR,
    #[doc = "0x40 - Common Interrupt Status Enable Register"]
    pub com_intstatus_en: COM_INTSTATUS_EN,
    #[doc = "0x48 - Common Interrupt Signal Enable Register"]
    pub com_intsignal_en: COM_INTSIGNAL_EN,
    #[doc = "0x50 - Common Interrupt Status"]
    pub com_intstatus: COM_INTSTATUS,
    #[doc = "0x58 - Reset register"]
    pub reset: RESET,
    _reserved1: [u8; 160usize],
    #[doc = "0x100 - Channel configuration"]
    pub channel: [CHANNEL; 6],
}
#[doc = r" Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - SAR Address Register"]
    pub sar: self::channel::SAR,
    #[doc = "0x08 - DAR Address Register"]
    pub dar: self::channel::DAR,
    #[doc = "0x10 - Block Transfer Size Register"]
    pub block_ts: self::channel::BLOCK_TS,
    #[doc = "0x18 - Control Register"]
    pub ctl: self::channel::CTL,
    #[doc = "0x20 - Configure Register"]
    pub cfg: self::channel::CFG,
    #[doc = "0x28 - Linked List Pointer register"]
    pub llp: self::channel::LLP,
    #[doc = "0x30 - Channel Status Register"]
    pub status: self::channel::STATUS,
    #[doc = "0x38 - Channel Software handshake Source Register"]
    pub swhssrc: self::channel::SWHSSRC,
    #[doc = "0x40 - Channel Software handshake Destination Register"]
    pub swhsdst: self::channel::SWHSDST,
    #[doc = "0x48 - Channel Block Transfer Resume Request Register"]
    pub blk_tfr: self::channel::BLK_TFR,
    #[doc = "0x50 - Channel AXI ID Register"]
    pub axi_id: self::channel::AXI_ID,
    #[doc = "0x58 - AXI QOS Register"]
    pub axi_qos: self::channel::AXI_QOS,
    _reserved0: [u8; 32usize],
    #[doc = "0x80 - Interrupt Status Enable Register"]
    pub intstatus_en: self::channel::INTSTATUS_EN,
    #[doc = "0x88 - Channel Interrupt Status Register"]
    pub intstatus: self::channel::INTSTATUS,
    #[doc = "0x90 - Interrupt Signal Enable Register"]
    pub intsignal_en: self::channel::INTSIGNAL_EN,
    #[doc = "0x98 - Interrupt Clear Register"]
    pub intclear: self::channel::INTCLEAR,
    _reserved1: [u8; 88usize],
    #[doc = "0xf8 - Padding to make structure size 256 bytes so that channels\\[\\] is an array"]
    pub _reserved: self::channel::_RESERVED,
}
#[doc = r" Register block"]
#[doc = "Channel configuration"]
pub mod channel;
#[doc = "ID Register"]
pub struct ID {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "ID Register"]
pub mod id;
#[doc = "COMPVER Register"]
pub struct COMPVER {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "COMPVER Register"]
pub mod compver;
#[doc = "Configure Register"]
pub struct CFG {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Configure Register"]
pub mod cfg;
#[doc = "Channel Enable Register"]
pub struct CHEN {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Channel Enable Register"]
pub mod chen;
#[doc = "Interrupt Status Register"]
pub struct INTSTATUS {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Interrupt Status Register"]
pub mod intstatus;
#[doc = "Common Interrupt Clear Register"]
pub struct COM_INTCLEAR {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Common Interrupt Clear Register"]
pub mod com_intclear;
#[doc = "Common Interrupt Status Enable Register"]
pub struct COM_INTSTATUS_EN {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Common Interrupt Status Enable Register"]
pub mod com_intstatus_en;
#[doc = "Common Interrupt Signal Enable Register"]
pub struct COM_INTSIGNAL_EN {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Common Interrupt Signal Enable Register"]
pub mod com_intsignal_en;
#[doc = "Common Interrupt Status"]
pub struct COM_INTSTATUS {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Common Interrupt Status"]
pub mod com_intstatus;
#[doc = "Reset register"]
pub struct RESET {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Reset register"]
pub mod reset;
