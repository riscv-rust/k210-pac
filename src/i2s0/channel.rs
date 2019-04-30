#[doc = "Left Receive or Left Transmit Register"]
pub struct LEFT_RXTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Left Receive or Left Transmit Register"]
pub mod left_rxtx;
#[doc = "Right Receive or Right Transmit Register"]
pub struct RIGHT_RXTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Right Receive or Right Transmit Register"]
pub mod right_rxtx;
#[doc = "Receive Enable Register"]
pub struct RER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Enable Register"]
pub mod rer;
#[doc = "Transmit Enable Register"]
pub struct TER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Enable Register"]
pub mod ter;
#[doc = "Receive Configuration Register"]
pub struct RCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Configuration Register"]
pub mod rcr;
#[doc = "Transmit Configuration Register"]
pub struct TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Configuration Register"]
pub mod tcr;
#[doc = "Interrupt Status Register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Interrupt Mask Register"]
pub struct IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "Receive Overrun Register"]
pub struct ROR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Overrun Register"]
pub mod ror;
#[doc = "Transmit Overrun Register"]
pub struct TOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Overrun Register"]
pub mod tor;
#[doc = "Receive FIFO Configuration Register"]
pub struct RFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO Configuration Register"]
pub mod rfcr;
#[doc = "Transmit FIFO Configuration Register"]
pub struct TFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit FIFO Configuration Register"]
pub mod tfcr;
#[doc = "Receive FIFO Flush Register"]
pub struct RFF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO Flush Register"]
pub mod rff;
#[doc = "Transmit FIFO Flush Register"]
pub struct TFF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit FIFO Flush Register"]
pub mod tff;
#[doc = "_RESERVED0"]
pub struct _RESERVED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "_RESERVED0"]
pub mod _reserved;
