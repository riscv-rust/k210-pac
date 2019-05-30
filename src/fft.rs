#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FFT input data fifo"]
    pub input_fifo: INPUT_FIFO,
    #[doc = "0x08 - FFT control register"]
    pub ctrl: CTRL,
    #[doc = "0x10 - FIFO control"]
    pub fifo_ctrl: FIFO_CTRL,
    #[doc = "0x18 - intr_mask"]
    pub interruptmask: INTERRUPTMASK,
    #[doc = "0x20 - Interrupt clear"]
    pub intr_clear: INTR_CLEAR,
    #[doc = "0x28 - FFT status register"]
    pub status: STATUS,
    #[doc = "0x30 - FFT status raw"]
    pub status_raw: STATUS_RAW,
    #[doc = "0x38 - FFT output FIFO"]
    pub output_fifo: OUTPUT_FIFO,
}
#[doc = "FFT input data fifo"]
pub struct INPUT_FIFO {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "FFT input data fifo"]
pub mod input_fifo;
#[doc = "FFT control register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "FFT control register"]
pub mod ctrl;
#[doc = "FIFO control"]
pub struct FIFO_CTRL {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "FIFO control"]
pub mod fifo_ctrl;
#[doc = "intr_mask"]
pub struct INTERRUPTMASK {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "intr_mask"]
pub mod interruptmask;
#[doc = "Interrupt clear"]
pub struct INTR_CLEAR {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "Interrupt clear"]
pub mod intr_clear;
#[doc = "FFT status register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "FFT status register"]
pub mod status;
#[doc = "FFT status raw"]
pub struct STATUS_RAW {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "FFT status raw"]
pub mod status_raw;
#[doc = "FFT output FIFO"]
pub struct OUTPUT_FIFO {
    register: ::vcell::VolatileCell<u64>,
}
#[doc = "FFT output FIFO"]
pub mod output_fifo;
