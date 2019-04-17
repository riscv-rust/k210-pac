#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register 0"]
    pub ctrlr0: CTRLR0,
    #[doc = "0x04 - Control Register 1"]
    pub ctrlr1: CTRLR1,
    #[doc = "0x08 - Enable Register"]
    pub ssienr: SSIENR,
    #[doc = "0x0c - Microwire Control Register"]
    pub mwcr: MWCR,
    #[doc = "0x10 - Slave Enable Register"]
    pub ser: SER,
    #[doc = "0x14 - Baud Rate Select"]
    pub baudr: BAUDR,
    #[doc = "0x18 - Transmit FIFO Threshold Level"]
    pub txftlr: TXFTLR,
    #[doc = "0x1c - Receive FIFO Threshold Level"]
    pub rxftlr: RXFTLR,
    #[doc = "0x20 - Transmit FIFO Level Register"]
    pub txflr: TXFLR,
    #[doc = "0x24 - Receive FIFO Level Register"]
    pub rxflr: RXFLR,
    #[doc = "0x28 - Status Register"]
    pub sr: SR,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x30 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x34 - Raw Interrupt Status Register"]
    pub risr: RISR,
    #[doc = "0x38 - Transmit FIFO Overflow Interrupt Clear Register"]
    pub txoicr: TXOICR,
    #[doc = "0x3c - Receive FIFO Overflow Interrupt Clear Register"]
    pub rxoicr: RXOICR,
    #[doc = "0x40 - Receive FIFO Underflow Interrupt Clear Register"]
    pub rxuicr: RXUICR,
    #[doc = "0x44 - Multi-Master Interrupt Clear Register"]
    pub msticr: MSTICR,
    #[doc = "0x48 - Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x4c - DMA Control Register"]
    pub dmacr: DMACR,
    #[doc = "0x50 - DMA Transmit Data Level"]
    pub dmatdlr: DMATDLR,
    #[doc = "0x54 - DMA Receive Data Level"]
    pub dmardlr: DMARDLR,
    #[doc = "0x58 - Identification Register"]
    pub idr: IDR,
    #[doc = "0x5c - DWC_ssi component version"]
    pub ssic_version_id: SSIC_VERSION_ID,
    #[doc = "0x60 - Data Register"]
    pub dr: [DR; 36],
    #[doc = "0xf0 - RX Sample Delay Register"]
    pub rx_sample_delay: RX_SAMPLE_DELAY,
    #[doc = "0xf4 - SPI Control Register"]
    pub spi_ctrlr0: SPI_CTRLR0,
    _reserved0: [u8; 4usize],
    #[doc = "0xfc - XIP Mode bits"]
    pub xip_mode_bits: XIP_MODE_BITS,
    #[doc = "0x100 - XIP INCR transfer opcode"]
    pub xip_incr_inst: XIP_INCR_INST,
    #[doc = "0x104 - XIP WRAP transfer opcode"]
    pub xip_wrap_inst: XIP_WRAP_INST,
    #[doc = "0x108 - XIP Control Register"]
    pub xip_ctrl: XIP_CTRL,
    #[doc = "0x10c - XIP Slave Enable Register"]
    pub xip_ser: XIP_SER,
    #[doc = "0x110 - XIP Receive FIFO Overflow Interrupt Clear Register"]
    pub xrxoicr: XRXOICR,
    #[doc = "0x114 - XIP time out register for continuous transfers"]
    pub xip_cnt_time_out: XIP_CNT_TIME_OUT,
    #[doc = "0x118 - ENDIAN"]
    pub endian: ENDIAN,
}
#[doc = "Control Register 0"]
pub struct CTRLR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register 0"]
pub mod ctrlr0;
#[doc = "Control Register 1"]
pub struct CTRLR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register 1"]
pub mod ctrlr1;
#[doc = "Enable Register"]
pub struct SSIENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Register"]
pub mod ssienr;
#[doc = "Microwire Control Register"]
pub struct MWCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Microwire Control Register"]
pub mod mwcr;
#[doc = "Slave Enable Register"]
pub struct SER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave Enable Register"]
pub mod ser;
#[doc = "Baud Rate Select"]
pub struct BAUDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Baud Rate Select"]
pub mod baudr;
#[doc = "Transmit FIFO Threshold Level"]
pub struct TXFTLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit FIFO Threshold Level"]
pub mod txftlr;
#[doc = "Receive FIFO Threshold Level"]
pub struct RXFTLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO Threshold Level"]
pub mod rxftlr;
#[doc = "Transmit FIFO Level Register"]
pub struct TXFLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit FIFO Level Register"]
pub mod txflr;
#[doc = "Receive FIFO Level Register"]
pub struct RXFLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO Level Register"]
pub mod rxflr;
#[doc = "Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Interrupt Mask Register"]
pub struct IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "Interrupt Status Register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Raw Interrupt Status Register"]
pub struct RISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw Interrupt Status Register"]
pub mod risr;
#[doc = "Transmit FIFO Overflow Interrupt Clear Register"]
pub struct TXOICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit FIFO Overflow Interrupt Clear Register"]
pub mod txoicr;
#[doc = "Receive FIFO Overflow Interrupt Clear Register"]
pub struct RXOICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO Overflow Interrupt Clear Register"]
pub mod rxoicr;
#[doc = "Receive FIFO Underflow Interrupt Clear Register"]
pub struct RXUICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO Underflow Interrupt Clear Register"]
pub mod rxuicr;
#[doc = "Multi-Master Interrupt Clear Register"]
pub struct MSTICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Multi-Master Interrupt Clear Register"]
pub mod msticr;
#[doc = "Interrupt Clear Register"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "DMA Control Register"]
pub struct DMACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Control Register"]
pub mod dmacr;
#[doc = "DMA Transmit Data Level"]
pub struct DMATDLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Transmit Data Level"]
pub mod dmatdlr;
#[doc = "DMA Receive Data Level"]
pub struct DMARDLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Receive Data Level"]
pub mod dmardlr;
#[doc = "Identification Register"]
pub struct IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Identification Register"]
pub mod idr;
#[doc = "DWC_ssi component version"]
pub struct SSIC_VERSION_ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DWC_ssi component version"]
pub mod ssic_version_id;
#[doc = "Data Register"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Register"]
pub mod dr;
#[doc = "RX Sample Delay Register"]
pub struct RX_SAMPLE_DELAY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX Sample Delay Register"]
pub mod rx_sample_delay;
#[doc = "SPI Control Register"]
pub struct SPI_CTRLR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Control Register"]
pub mod spi_ctrlr0;
#[doc = "XIP Mode bits"]
pub struct XIP_MODE_BITS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XIP Mode bits"]
pub mod xip_mode_bits;
#[doc = "XIP INCR transfer opcode"]
pub struct XIP_INCR_INST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XIP INCR transfer opcode"]
pub mod xip_incr_inst;
#[doc = "XIP WRAP transfer opcode"]
pub struct XIP_WRAP_INST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XIP WRAP transfer opcode"]
pub mod xip_wrap_inst;
#[doc = "XIP Control Register"]
pub struct XIP_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XIP Control Register"]
pub mod xip_ctrl;
#[doc = "XIP Slave Enable Register"]
pub struct XIP_SER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XIP Slave Enable Register"]
pub mod xip_ser;
#[doc = "XIP Receive FIFO Overflow Interrupt Clear Register"]
pub struct XRXOICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XIP Receive FIFO Overflow Interrupt Clear Register"]
pub mod xrxoicr;
#[doc = "XIP time out register for continuous transfers"]
pub struct XIP_CNT_TIME_OUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XIP time out register for continuous transfers"]
pub mod xip_cnt_time_out;
#[doc = "ENDIAN"]
pub struct ENDIAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ENDIAN"]
pub mod endian;
