#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Transmit Data Register"]
    pub txdata: TXDATA,
    #[doc = "0x04 - Receive Data Register"]
    pub rxdata: RXDATA,
    #[doc = "0x08 - Transmit Control Register"]
    pub txctrl: TXCTRL,
    #[doc = "0x0c - Receive Control Register"]
    pub rxctrl: RXCTRL,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ie: IE,
    #[doc = "0x14 - Interrupt Pending Register"]
    pub ip: IP,
    #[doc = "0x18 - Baud Rate Divisor Register"]
    pub div: DIV,
}
#[doc = "Transmit Data Register"]
pub struct TXDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Data Register"]
pub mod txdata;
#[doc = "Receive Data Register"]
pub struct RXDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Data Register"]
pub mod rxdata;
#[doc = "Transmit Control Register"]
pub struct TXCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Control Register"]
pub mod txctrl;
#[doc = "Receive Control Register"]
pub struct RXCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Control Register"]
pub mod rxctrl;
#[doc = "Interrupt Enable Register"]
pub struct IE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ie;
#[doc = "Interrupt Pending Register"]
pub struct IP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Pending Register"]
pub mod ip;
#[doc = "Baud Rate Divisor Register"]
pub struct DIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Baud Rate Divisor Register"]
pub mod div;
