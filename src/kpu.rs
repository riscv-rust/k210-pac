#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Layer arguments FIFO: each layer is defined by writing 12 successive argument values to this register"]
    pub layer_argument_fifo: LAYER_ARGUMENT_FIFO,
    #[doc = "0x08 - Interrupt status"]
    pub interrupt_status: INTERRUPT_STATUS,
    #[doc = "0x10 - Interrupt raw"]
    pub interrupt_raw: INTERRUPT_RAW,
    #[doc = "0x18 - Interrupt mask: 0 enables the interrupt, 1 masks the interrupt"]
    pub interrupt_mask: INTERRUPT_MASK,
    #[doc = "0x20 - Interrupt clear: write 1 to a bit to clear interrupt"]
    pub interrupt_clear: INTERRUPT_CLEAR,
    #[doc = "0x28 - FIFO threshold"]
    pub fifo_threshold: FIFO_THRESHOLD,
    #[doc = "0x30 - FIFO data output"]
    pub fifo_data_out: FIFO_DATA_OUT,
    #[doc = "0x38 - FIFO control"]
    pub fifo_ctrl: FIFO_CTRL,
    #[doc = "0x40 - Eight bit mode"]
    pub eight_bit_mode: EIGHT_BIT_MODE,
}
#[doc = "Layer arguments FIFO: each layer is defined by writing 12 successive argument values to this register"]
pub struct LAYER_ARGUMENT_FIFO {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Layer arguments FIFO: each layer is defined by writing 12 successive argument values to this register"]
pub mod layer_argument_fifo;
#[doc = "Interrupt status"]
pub struct INTERRUPT_STATUS {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Interrupt status"]
pub mod interrupt_status;
#[doc = "Interrupt raw"]
pub struct INTERRUPT_RAW {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Interrupt raw"]
pub mod interrupt_raw;
#[doc = "Interrupt mask: 0 enables the interrupt, 1 masks the interrupt"]
pub struct INTERRUPT_MASK {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Interrupt mask: 0 enables the interrupt, 1 masks the interrupt"]
pub mod interrupt_mask;
#[doc = "Interrupt clear: write 1 to a bit to clear interrupt"]
pub struct INTERRUPT_CLEAR {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Interrupt clear: write 1 to a bit to clear interrupt"]
pub mod interrupt_clear;
#[doc = "FIFO threshold"]
pub struct FIFO_THRESHOLD {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "FIFO threshold"]
pub mod fifo_threshold;
#[doc = "FIFO data output"]
pub struct FIFO_DATA_OUT {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "FIFO data output"]
pub mod fifo_data_out;
#[doc = "FIFO control"]
pub struct FIFO_CTRL {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "FIFO control"]
pub mod fifo_ctrl;
#[doc = "Eight bit mode"]
pub struct EIGHT_BIT_MODE {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Eight bit mode"]
pub mod eight_bit_mode;
