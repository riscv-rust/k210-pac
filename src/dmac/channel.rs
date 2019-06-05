#[doc = "SAR Address Register"]
pub struct SAR {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "SAR Address Register"]
pub mod sar;
#[doc = "DAR Address Register"]
pub struct DAR {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "DAR Address Register"]
pub mod dar;
#[doc = "Block Transfer Size Register"]
pub struct BLOCK_TS {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Block Transfer Size Register"]
pub mod block_ts;
#[doc = "Control Register"]
pub struct CTL {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Control Register"]
pub mod ctl;
#[doc = "Configure Register"]
pub struct CFG {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Configure Register"]
pub mod cfg;
#[doc = "Linked List Pointer register"]
pub struct LLP {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Linked List Pointer register"]
pub mod llp;
#[doc = "Channel Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Channel Status Register"]
pub mod status;
#[doc = "Channel Software handshake Source Register"]
pub struct SWHSSRC {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Channel Software handshake Source Register"]
pub mod swhssrc;
#[doc = "Channel Software handshake Destination Register"]
pub struct SWHSDST {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Channel Software handshake Destination Register"]
pub mod swhsdst;
#[doc = "Channel Block Transfer Resume Request Register"]
pub struct BLK_TFR {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Channel Block Transfer Resume Request Register"]
pub mod blk_tfr;
#[doc = "Channel AXI ID Register"]
pub struct AXI_ID {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Channel AXI ID Register"]
pub mod axi_id;
#[doc = "AXI QOS Register"]
pub struct AXI_QOS {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "AXI QOS Register"]
pub mod axi_qos;
#[doc = "Interrupt Status Enable Register"]
pub struct INTSTATUS_EN {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Interrupt Status Enable Register"]
pub mod intstatus_en;
#[doc = "Channel Interrupt Status Register"]
pub struct INTSTATUS {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Channel Interrupt Status Register"]
pub mod intstatus;
#[doc = "Interrupt Signal Enable Register"]
pub struct INTSIGNAL_EN {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Interrupt Signal Enable Register"]
pub mod intsignal_en;
#[doc = "Interrupt Clear Register"]
pub struct INTCLEAR {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Interrupt Clear Register"]
pub mod intclear;
#[doc = "Padding to make structure size 256 bytes so that channels\\[\\] is an array"]
pub struct _RESERVED {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Padding to make structure size 256 bytes so that channels\\[\\] is an array"]
pub mod _reserved;
