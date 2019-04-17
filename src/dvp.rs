#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Config Register"]
    pub dvp_cfg: DVP_CFG,
    #[doc = "0x04 - R_ADDR"]
    pub r_addr: R_ADDR,
    #[doc = "0x08 - G_ADDR"]
    pub g_addr: G_ADDR,
    #[doc = "0x0c - B_ADDR"]
    pub b_addr: B_ADDR,
    #[doc = "0x10 - CMOS Config Register"]
    pub cmos_cfg: CMOS_CFG,
    #[doc = "0x14 - SCCB Config Register"]
    pub sccb_cfg: SCCB_CFG,
    #[doc = "0x18 - SCCB Control Register"]
    pub sccb_ctl: SCCB_CTL,
    #[doc = "0x1c - AXI Register"]
    pub axi: AXI,
    #[doc = "0x20 - STS Register"]
    pub sts: STS,
    #[doc = "0x24 - REVERSE"]
    pub reverse: REVERSE,
    #[doc = "0x28 - RGB_ADDR"]
    pub rgb_addr: RGB_ADDR,
}
#[doc = "Config Register"]
pub struct DVP_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Config Register"]
pub mod dvp_cfg;
#[doc = "R_ADDR"]
pub struct R_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "R_ADDR"]
pub mod r_addr;
#[doc = "G_ADDR"]
pub struct G_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "G_ADDR"]
pub mod g_addr;
#[doc = "B_ADDR"]
pub struct B_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "B_ADDR"]
pub mod b_addr;
#[doc = "CMOS Config Register"]
pub struct CMOS_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CMOS Config Register"]
pub mod cmos_cfg;
#[doc = "SCCB Config Register"]
pub struct SCCB_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCCB Config Register"]
pub mod sccb_cfg;
#[doc = "SCCB Control Register"]
pub struct SCCB_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCCB Control Register"]
pub mod sccb_ctl;
#[doc = "AXI Register"]
pub struct AXI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AXI Register"]
pub mod axi;
#[doc = "STS Register"]
pub struct STS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "STS Register"]
pub mod sts;
#[doc = "REVERSE"]
pub struct REVERSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "REVERSE"]
pub mod reverse;
#[doc = "RGB_ADDR"]
pub struct RGB_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RGB_ADDR"]
pub mod rgb_addr;
