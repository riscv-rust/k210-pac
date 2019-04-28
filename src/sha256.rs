#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Calculated SHA256 return value"]
    pub result: [RESULT; 8],
    #[doc = "0x20 - SHA256 input data is written to this register"]
    pub data_in: DATA_IN,
    _reserved0: [u8; 4usize],
    #[doc = "0x28 - Counters register"]
    pub num_reg: NUM_REG,
    #[doc = "0x2c - Function configuration register 0"]
    pub function_reg_0: FUNCTION_REG_0,
    _reserved1: [u8; 4usize],
    #[doc = "0x34 - Function configuration register 1"]
    pub function_reg_1: FUNCTION_REG_1,
}
#[doc = "Calculated SHA256 return value"]
pub struct RESULT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calculated SHA256 return value"]
pub mod result;
#[doc = "SHA256 input data is written to this register"]
pub struct DATA_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SHA256 input data is written to this register"]
pub mod data_in;
#[doc = "Counters register"]
pub struct NUM_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counters register"]
pub mod num_reg;
#[doc = "Function configuration register 0"]
pub struct FUNCTION_REG_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Function configuration register 0"]
pub mod function_reg_0;
#[doc = "Function configuration register 1"]
pub struct FUNCTION_REG_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Function configuration register 1"]
pub mod function_reg_1;
