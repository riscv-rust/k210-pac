#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FPIOA GPIO multiplexer io array"]
    pub io: [IO; 48],
    #[doc = "0xc0 - FPIOA GPIO multiplexer tie enable array"]
    pub tie_en: [TIE_EN; 8],
    #[doc = "0xe0 - FPIOA GPIO multiplexer tie value array"]
    pub tie_val: [TIE_VAL; 8],
}
#[doc = "FPIOA GPIO multiplexer io array"]
pub struct IO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FPIOA GPIO multiplexer io array"]
pub mod io;
#[doc = "FPIOA GPIO multiplexer tie enable array"]
pub struct TIE_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FPIOA GPIO multiplexer tie enable array"]
pub mod tie_en;
#[doc = "FPIOA GPIO multiplexer tie value array"]
pub struct TIE_VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FPIOA GPIO multiplexer tie value array"]
pub mod tie_val;
