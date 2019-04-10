#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Git short commit id"]
    pub git_id: GIT_ID,
    #[doc = "0x04 - System clock base frequency"]
    pub clk_freq: CLK_FREQ,
    #[doc = "0x08 - PLL0 controller"]
    pub pll0: PLL0,
    #[doc = "0x0c - PLL1 controller"]
    pub pll1: PLL1,
    #[doc = "0x10 - PLL2 controller"]
    pub pll2: PLL2,
    _reserved0: [u8; 4usize],
    #[doc = "0x18 - PLL lock tester"]
    pub pll_lock: PLL_LOCK,
    #[doc = "0x1c - AXI ROM detector"]
    pub rom_error: ROM_ERROR,
    #[doc = "0x20 - Clock select controller 0"]
    pub clk_sel0: CLK_SEL0,
    #[doc = "0x24 - Clock select controller 1"]
    pub clk_sel1: CLK_SEL1,
    #[doc = "0x28 - Central clock enable"]
    pub clk_en_cent: CLK_EN_CENT,
    #[doc = "0x2c - Peripheral clock enable"]
    pub clk_en_peri: CLK_EN_PERI,
    #[doc = "0x30 - Soft reset ctrl"]
    pub soft_reset: SOFT_RESET,
    #[doc = "0x34 - Peripheral reset controller"]
    pub peri_reset: PERI_RESET,
    #[doc = "0x38 - Clock threshold controller 0"]
    pub clk_th0: CLK_TH0,
    #[doc = "0x3c - Clock threshold controller 1"]
    pub clk_th1: CLK_TH1,
    #[doc = "0x40 - Clock threshold controller 2"]
    pub clk_th2: CLK_TH2,
    #[doc = "0x44 - Clock threshold controller 3"]
    pub clk_th3: CLK_TH3,
    #[doc = "0x48 - Clock threshold controller 4"]
    pub clk_th4: CLK_TH4,
    #[doc = "0x4c - Clock threshold controller 5"]
    pub clk_th5: CLK_TH5,
    #[doc = "0x50 - Clock threshold controller 6"]
    pub clk_th6: CLK_TH6,
    #[doc = "0x54 - Miscellaneous controller"]
    pub misc: MISC,
    #[doc = "0x58 - Peripheral controller"]
    pub peri: PERI,
    #[doc = "0x5c - SPI sleep controller"]
    pub spi_sleep: SPI_SLEEP,
    #[doc = "0x60 - Reset source status"]
    pub reset_status: RESET_STATUS,
    #[doc = "0x64 - DMA handshake selector"]
    pub dma_sel0: DMA_SEL0,
    #[doc = "0x68 - DMA handshake selector"]
    pub dma_sel1: DMA_SEL1,
    #[doc = "0x6c - IO Power Mode Select controller"]
    pub power_sel: POWER_SEL,
}
#[doc = "Git short commit id"]
pub struct GIT_ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Git short commit id"]
pub mod git_id;
#[doc = "System clock base frequency"]
pub struct CLK_FREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System clock base frequency"]
pub mod clk_freq;
#[doc = "PLL0 controller"]
pub struct PLL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0 controller"]
pub mod pll0;
#[doc = "PLL1 controller"]
pub struct PLL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL1 controller"]
pub mod pll1;
#[doc = "PLL2 controller"]
pub struct PLL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL2 controller"]
pub mod pll2;
#[doc = "PLL lock tester"]
pub struct PLL_LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL lock tester"]
pub mod pll_lock;
#[doc = "AXI ROM detector"]
pub struct ROM_ERROR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AXI ROM detector"]
pub mod rom_error;
#[doc = "Clock select controller 0"]
pub struct CLK_SEL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock select controller 0"]
pub mod clk_sel0;
#[doc = "Clock select controller 1"]
pub struct CLK_SEL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock select controller 1"]
pub mod clk_sel1;
#[doc = "Central clock enable"]
pub struct CLK_EN_CENT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Central clock enable"]
pub mod clk_en_cent;
#[doc = "Peripheral clock enable"]
pub struct CLK_EN_PERI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral clock enable"]
pub mod clk_en_peri;
#[doc = "Soft reset ctrl"]
pub struct SOFT_RESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Soft reset ctrl"]
pub mod soft_reset;
#[doc = "Peripheral reset controller"]
pub struct PERI_RESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset controller"]
pub mod peri_reset;
#[doc = "Clock threshold controller 0"]
pub struct CLK_TH0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock threshold controller 0"]
pub mod clk_th0;
#[doc = "Clock threshold controller 1"]
pub struct CLK_TH1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock threshold controller 1"]
pub mod clk_th1;
#[doc = "Clock threshold controller 2"]
pub struct CLK_TH2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock threshold controller 2"]
pub mod clk_th2;
#[doc = "Clock threshold controller 3"]
pub struct CLK_TH3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock threshold controller 3"]
pub mod clk_th3;
#[doc = "Clock threshold controller 4"]
pub struct CLK_TH4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock threshold controller 4"]
pub mod clk_th4;
#[doc = "Clock threshold controller 5"]
pub struct CLK_TH5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock threshold controller 5"]
pub mod clk_th5;
#[doc = "Clock threshold controller 6"]
pub struct CLK_TH6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock threshold controller 6"]
pub mod clk_th6;
#[doc = "Miscellaneous controller"]
pub struct MISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous controller"]
pub mod misc;
#[doc = "Peripheral controller"]
pub struct PERI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral controller"]
pub mod peri;
#[doc = "SPI sleep controller"]
pub struct SPI_SLEEP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI sleep controller"]
pub mod spi_sleep;
#[doc = "Reset source status"]
pub struct RESET_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset source status"]
pub mod reset_status;
#[doc = "DMA handshake selector"]
pub struct DMA_SEL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA handshake selector"]
pub mod dma_sel0;
#[doc = "DMA handshake selector"]
pub struct DMA_SEL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA handshake selector"]
pub mod dma_sel1;
#[doc = "IO Power Mode Select controller"]
pub struct POWER_SEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IO Power Mode Select controller"]
pub mod power_sel;
