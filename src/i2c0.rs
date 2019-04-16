#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub con: CON,
    #[doc = "0x04 - Target Address Register"]
    pub tar: TAR,
    #[doc = "0x08 - Slave Address Register"]
    pub sar: SAR,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Data Buffer and Command Register"]
    pub data_cmd: DATA_CMD,
    #[doc = "0x14 - Standard Speed Clock SCL High Count Register"]
    pub ss_scl_hcnt: SS_SCL_HCNT,
    #[doc = "0x18 - Standard Speed Clock SCL Low Count Register"]
    pub ss_scl_lcnt: SS_SCL_LCNT,
    _reserved1: [u8; 16usize],
    #[doc = "0x2c - Interrupt Status Register"]
    pub intr_stat: INTR_STAT,
    #[doc = "0x30 - Interrupt Mask Register"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x34 - Raw Interrupt Status Register"]
    pub raw_intr_stat: RAW_INTR_STAT,
    #[doc = "0x38 - Receive FIFO Threshold Register"]
    pub rx_tl: RX_TL,
    #[doc = "0x3c - Transmit FIFO Threshold Register"]
    pub tx_tl: TX_TL,
    #[doc = "0x40 - Clear Combined and Individual Interrupt Register"]
    pub clr_intr: CLR_INTR,
    #[doc = "0x44 - Clear RX_UNDER Interrupt Register"]
    pub clr_rx_under: CLR_RX_UNDER,
    #[doc = "0x48 - Clear RX_OVER Interrupt Register"]
    pub clr_rx_over: CLR_RX_OVER,
    #[doc = "0x4c - Clear TX_OVER Interrupt Register"]
    pub clr_tx_over: CLR_TX_OVER,
    #[doc = "0x50 - Clear RD_REQ Interrupt Register"]
    pub clr_rd_req: CLR_RD_REQ,
    #[doc = "0x54 - Clear TX_ABRT Interrupt Register"]
    pub clr_tx_abrt: CLR_TX_ABRT,
    #[doc = "0x58 - Clear RX_DONE Interrupt Register"]
    pub clr_rx_done: CLR_RX_DONE,
    #[doc = "0x5c - Clear ACTIVITY Interrupt Register"]
    pub clr_activity: CLR_ACTIVITY,
    #[doc = "0x60 - Clear STOP_DET Interrupt Register"]
    pub clr_stop_det: CLR_STOP_DET,
    #[doc = "0x64 - Clear START_DET Interrupt Register"]
    pub clr_start_det: CLR_START_DET,
    #[doc = "0x68 - I2C Clear GEN_CALL Interrupt Register"]
    pub clr_gen_call: CLR_GEN_CALL,
    #[doc = "0x6c - Enable Register"]
    pub enable: ENABLE,
    #[doc = "0x70 - Status Register"]
    pub status: STATUS,
    #[doc = "0x74 - Transmit FIFO Level Register"]
    pub txflr: TXFLR,
    #[doc = "0x78 - Receive FIFO Level Register"]
    pub rxflr: RXFLR,
    #[doc = "0x7c - SDA Hold Time Length Register"]
    pub sda_hold: SDA_HOLD,
    #[doc = "0x80 - Transmit Abort Source Register"]
    pub tx_abrt_source: TX_ABRT_SOURCE,
    _reserved2: [u8; 4usize],
    #[doc = "0x88 - I2C DMA Control Register"]
    pub dma_cr: DMA_CR,
    #[doc = "0x8c - DMA Transmit Data Level Register"]
    pub dma_tdlr: DMA_TDLR,
    #[doc = "0x90 - DMA Receive Data Level Register"]
    pub dma_rdlr: DMA_RDLR,
    #[doc = "0x94 - SDA Setup Register"]
    pub sda_setup: SDA_SETUP,
    #[doc = "0x98 - ACK General Call Register"]
    pub general_call: GENERAL_CALL,
    #[doc = "0x9c - Enable Status Register"]
    pub enable_status: ENABLE_STATUS,
    #[doc = "0xa0 - SS, FS or FM+ spike suppression limit"]
    pub fs_spklen: FS_SPKLEN,
    _reserved3: [u8; 80usize],
    #[doc = "0xf4 - Component Parameter Register 1"]
    pub comp_param_1: COMP_PARAM_1,
    #[doc = "0xf8 - Component Version Register"]
    pub comp_version: COMP_VERSION,
    #[doc = "0xfc - Component Type Register"]
    pub comp_type: COMP_TYPE,
}
#[doc = "Control Register"]
pub struct CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod con;
#[doc = "Target Address Register"]
pub struct TAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Address Register"]
pub mod tar;
#[doc = "Slave Address Register"]
pub struct SAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave Address Register"]
pub mod sar;
#[doc = "Data Buffer and Command Register"]
pub struct DATA_CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Buffer and Command Register"]
pub mod data_cmd;
#[doc = "Standard Speed Clock SCL High Count Register"]
pub struct SS_SCL_HCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Standard Speed Clock SCL High Count Register"]
pub mod ss_scl_hcnt;
#[doc = "Standard Speed Clock SCL Low Count Register"]
pub struct SS_SCL_LCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Standard Speed Clock SCL Low Count Register"]
pub mod ss_scl_lcnt;
#[doc = "Interrupt Status Register"]
pub struct INTR_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod intr_stat;
#[doc = "Interrupt Mask Register"]
pub struct INTR_MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod intr_mask;
#[doc = "Raw Interrupt Status Register"]
pub struct RAW_INTR_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw Interrupt Status Register"]
pub mod raw_intr_stat;
#[doc = "Receive FIFO Threshold Register"]
pub struct RX_TL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO Threshold Register"]
pub mod rx_tl;
#[doc = "Transmit FIFO Threshold Register"]
pub struct TX_TL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit FIFO Threshold Register"]
pub mod tx_tl;
#[doc = "Clear Combined and Individual Interrupt Register"]
pub struct CLR_INTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear Combined and Individual Interrupt Register"]
pub mod clr_intr;
#[doc = "Clear RX_UNDER Interrupt Register"]
pub struct CLR_RX_UNDER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear RX_UNDER Interrupt Register"]
pub mod clr_rx_under;
#[doc = "Clear RX_OVER Interrupt Register"]
pub struct CLR_RX_OVER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear RX_OVER Interrupt Register"]
pub mod clr_rx_over;
#[doc = "Clear TX_OVER Interrupt Register"]
pub struct CLR_TX_OVER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear TX_OVER Interrupt Register"]
pub mod clr_tx_over;
#[doc = "Clear RD_REQ Interrupt Register"]
pub struct CLR_RD_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear RD_REQ Interrupt Register"]
pub mod clr_rd_req;
#[doc = "Clear TX_ABRT Interrupt Register"]
pub struct CLR_TX_ABRT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear TX_ABRT Interrupt Register"]
pub mod clr_tx_abrt;
#[doc = "Clear RX_DONE Interrupt Register"]
pub struct CLR_RX_DONE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear RX_DONE Interrupt Register"]
pub mod clr_rx_done;
#[doc = "Clear ACTIVITY Interrupt Register"]
pub struct CLR_ACTIVITY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear ACTIVITY Interrupt Register"]
pub mod clr_activity;
#[doc = "Clear STOP_DET Interrupt Register"]
pub struct CLR_STOP_DET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear STOP_DET Interrupt Register"]
pub mod clr_stop_det;
#[doc = "Clear START_DET Interrupt Register"]
pub struct CLR_START_DET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear START_DET Interrupt Register"]
pub mod clr_start_det;
#[doc = "I2C Clear GEN_CALL Interrupt Register"]
pub struct CLR_GEN_CALL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Clear GEN_CALL Interrupt Register"]
pub mod clr_gen_call;
#[doc = "Enable Register"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Register"]
pub mod enable;
#[doc = "Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
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
#[doc = "SDA Hold Time Length Register"]
pub struct SDA_HOLD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDA Hold Time Length Register"]
pub mod sda_hold;
#[doc = "Transmit Abort Source Register"]
pub struct TX_ABRT_SOURCE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Abort Source Register"]
pub mod tx_abrt_source;
#[doc = "I2C DMA Control Register"]
pub struct DMA_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C DMA Control Register"]
pub mod dma_cr;
#[doc = "DMA Transmit Data Level Register"]
pub struct DMA_TDLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Transmit Data Level Register"]
pub mod dma_tdlr;
#[doc = "DMA Receive Data Level Register"]
pub struct DMA_RDLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Receive Data Level Register"]
pub mod dma_rdlr;
#[doc = "SDA Setup Register"]
pub struct SDA_SETUP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDA Setup Register"]
pub mod sda_setup;
#[doc = "ACK General Call Register"]
pub struct GENERAL_CALL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ACK General Call Register"]
pub mod general_call;
#[doc = "Enable Status Register"]
pub struct ENABLE_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Status Register"]
pub mod enable_status;
#[doc = "SS, FS or FM+ spike suppression limit"]
pub struct FS_SPKLEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SS, FS or FM+ spike suppression limit"]
pub mod fs_spklen;
#[doc = "Component Parameter Register 1"]
pub struct COMP_PARAM_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Component Parameter Register 1"]
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
