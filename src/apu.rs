#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel Config Register"]
    pub ch_cfg: CH_CFG,
    #[doc = "0x04 - Control Register"]
    pub ctl: CTL,
    #[doc = "0x08 - Direction Sample Buffer Read Index Configure Register (16 directions * 2 values * 4 indices)"]
    pub dir_bidx: [DIR_BIDX; 32],
    #[doc = "0x88 - FIR0 pre-filter coefficients"]
    pub pre_fir0_coef: [PRE_FIR0_COEF; 9],
    #[doc = "0xac - FIR0 post-filter coefficients"]
    pub post_fir0_coef: [POST_FIR0_COEF; 9],
    #[doc = "0xd0 - FIR1 pre-filter coeffecients"]
    pub pre_fir1_coef: [PRE_FIR1_COEF; 9],
    #[doc = "0xf4 - FIR1 post-filter coefficients"]
    pub post_fir1_coef: [POST_FIR1_COEF; 9],
    #[doc = "0x118 - Downsize Config Register"]
    pub dwsz_cfg: DWSZ_CFG,
    #[doc = "0x11c - FFT Config Register"]
    pub fft_cfg: FFT_CFG,
    #[doc = "0x120 - Read register for DMA to sample-out buffers"]
    pub sobuf_dma_rdata: SOBUF_DMA_RDATA,
    #[doc = "0x124 - Read register for DMA to voice-out buffers"]
    pub vobuf_dma_rdata: VOBUF_DMA_RDATA,
    #[doc = "0x128 - Interrupt Status Register"]
    pub int_stat: INT_STAT,
    #[doc = "0x12c - Interrupt Mask Register"]
    pub int_mask: INT_MASK,
    #[doc = "0x130 - Saturation Counter"]
    pub sat_counter: SAT_COUNTER,
    #[doc = "0x134 - Saturation Limits"]
    pub sat_limits: SAT_LIMITS,
}
#[doc = "Channel Config Register"]
pub struct CH_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Config Register"]
pub mod ch_cfg;
#[doc = "Control Register"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctl;
#[doc = "Direction Sample Buffer Read Index Configure Register (16 directions * 2 values * 4 indices)"]
pub struct DIR_BIDX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direction Sample Buffer Read Index Configure Register (16 directions * 2 values * 4 indices)"]
pub mod dir_bidx;
#[doc = "FIR0 pre-filter coefficients"]
pub struct PRE_FIR0_COEF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIR0 pre-filter coefficients"]
pub mod pre_fir0_coef;
#[doc = "FIR0 post-filter coefficients"]
pub struct POST_FIR0_COEF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIR0 post-filter coefficients"]
pub mod post_fir0_coef;
#[doc = "FIR1 pre-filter coeffecients"]
pub struct PRE_FIR1_COEF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIR1 pre-filter coeffecients"]
pub mod pre_fir1_coef;
#[doc = "FIR1 post-filter coefficients"]
pub struct POST_FIR1_COEF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIR1 post-filter coefficients"]
pub mod post_fir1_coef;
#[doc = "Downsize Config Register"]
pub struct DWSZ_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Downsize Config Register"]
pub mod dwsz_cfg;
#[doc = "FFT Config Register"]
pub struct FFT_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FFT Config Register"]
pub mod fft_cfg;
#[doc = "Read register for DMA to sample-out buffers"]
pub struct SOBUF_DMA_RDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read register for DMA to sample-out buffers"]
pub mod sobuf_dma_rdata;
#[doc = "Read register for DMA to voice-out buffers"]
pub struct VOBUF_DMA_RDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read register for DMA to voice-out buffers"]
pub mod vobuf_dma_rdata;
#[doc = "Interrupt Status Register"]
pub struct INT_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod int_stat;
#[doc = "Interrupt Mask Register"]
pub struct INT_MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod int_mask;
#[doc = "Saturation Counter"]
pub struct SAT_COUNTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Saturation Counter"]
pub mod sat_counter;
#[doc = "Saturation Limits"]
pub struct SAT_LIMITS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Saturation Limits"]
pub mod sat_limits;
