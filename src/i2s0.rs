#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Enable Register"]
    pub ier: IER,
    #[doc = "0x04 - Receiver Block Enable Register"]
    pub irer: IRER,
    #[doc = "0x08 - Transmitter Block Enable Register"]
    pub iter: ITER,
    #[doc = "0x0c - Clock Generation enable"]
    pub cer: CER,
    #[doc = "0x10 - Clock Configuration Register"]
    pub ccr: CCR,
    #[doc = "0x14 - Receiver Block FIFO Reset Register"]
    pub rxffr: RXFFR,
    #[doc = "0x18 - Transmitter Block FIFO Reset Register"]
    pub txffr: TXFFR,
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - Channel cluster"]
    pub channel: [CHANNEL; 4],
    _reserved1: [u8; 160usize],
    #[doc = "0x1c0 - Receiver Block DMA Register"]
    pub rxdma: RXDMA,
    #[doc = "0x1c4 - Reset Receiver Block DMA Register"]
    pub rrxdma: RRXDMA,
    #[doc = "0x1c8 - Transmitter Block DMA Register"]
    pub txdma: TXDMA,
    #[doc = "0x1cc - Reset Transmitter Block DMA Register"]
    pub rtxdma: RTXDMA,
    _reserved2: [u8; 32usize],
    #[doc = "0x1f0 - Component Parameter Register 2"]
    pub i2s_comp_param_2: I2S_COMP_PARAM_2,
    #[doc = "0x1f4 - Component Parameter Register 1"]
    pub i2s_comp_param_1: I2S_COMP_PARAM_1,
    #[doc = "0x1f8 - Component Version Register"]
    pub i2s_comp_version_1: I2S_COMP_VERSION_1,
    #[doc = "0x1fc - Component Type Register"]
    pub i2s_comp_type: I2S_COMP_TYPE,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Left Receive or Left Transmit Register"]
    pub left_rxtx: self::channel::LEFT_RXTX,
    #[doc = "0x04 - Right Receive or Right Transmit Register"]
    pub right_rxtx: self::channel::RIGHT_RXTX,
    #[doc = "0x08 - Receive Enable Register"]
    pub rer: self::channel::RER,
    #[doc = "0x0c - Transmit Enable Register"]
    pub ter: self::channel::TER,
    #[doc = "0x10 - Receive Configuration Register"]
    pub rcr: self::channel::RCR,
    #[doc = "0x14 - Transmit Configuration Register"]
    pub tcr: self::channel::TCR,
    #[doc = "0x18 - Interrupt Status Register"]
    pub isr: self::channel::ISR,
    #[doc = "0x1c - Interrupt Mask Register"]
    pub imr: self::channel::IMR,
    #[doc = "0x20 - Receive Overrun Register"]
    pub ror: self::channel::ROR,
    #[doc = "0x24 - Transmit Overrun Register"]
    pub tor: self::channel::TOR,
    #[doc = "0x28 - Receive FIFO Configuration Register"]
    pub rfcr: self::channel::RFCR,
    #[doc = "0x2c - Transmit FIFO Configuration Register"]
    pub tfcr: self::channel::TFCR,
    #[doc = "0x30 - Receive FIFO Flush Register"]
    pub rff: self::channel::RFF,
    #[doc = "0x34 - Transmit FIFO Flush Register"]
    pub tff: self::channel::TFF,
    #[doc = "0x38 - _RESERVED0"]
    pub _reserved: [self::channel::_RESERVED; 2],
}
#[doc = r" Register block"]
#[doc = "Channel cluster"]
pub mod channel;
#[doc = "Enable Register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Register"]
pub mod ier;
#[doc = "Receiver Block Enable Register"]
pub struct IRER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver Block Enable Register"]
pub mod irer;
#[doc = "Transmitter Block Enable Register"]
pub struct ITER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmitter Block Enable Register"]
pub mod iter;
#[doc = "Clock Generation enable"]
pub struct CER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Generation enable"]
pub mod cer;
#[doc = "Clock Configuration Register"]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Configuration Register"]
pub mod ccr;
#[doc = "Receiver Block FIFO Reset Register"]
pub struct RXFFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver Block FIFO Reset Register"]
pub mod rxffr;
#[doc = "Transmitter Block FIFO Reset Register"]
pub struct TXFFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmitter Block FIFO Reset Register"]
pub mod txffr;
#[doc = "Receiver Block DMA Register"]
pub struct RXDMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver Block DMA Register"]
pub mod rxdma;
#[doc = "Reset Receiver Block DMA Register"]
pub struct RRXDMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Receiver Block DMA Register"]
pub mod rrxdma;
#[doc = "Transmitter Block DMA Register"]
pub struct TXDMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmitter Block DMA Register"]
pub mod txdma;
#[doc = "Reset Transmitter Block DMA Register"]
pub struct RTXDMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Transmitter Block DMA Register"]
pub mod rtxdma;
#[doc = "Component Parameter Register 2"]
pub struct I2S_COMP_PARAM_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Component Parameter Register 2"]
pub mod i2s_comp_param_2;
#[doc = "Component Parameter Register 1"]
pub struct I2S_COMP_PARAM_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Component Parameter Register 1"]
pub mod i2s_comp_param_1;
#[doc = "Component Version Register"]
pub struct I2S_COMP_VERSION_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Component Version Register"]
pub mod i2s_comp_version_1;
#[doc = "Component Type Register"]
pub struct I2S_COMP_TYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Component Type Register"]
pub mod i2s_comp_type;
