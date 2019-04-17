#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Dummy register: this peripheral is not implemented yet"]
    pub dummy: DUMMY,
}
#[doc = "Dummy register: this peripheral is not implemented yet"]
pub struct DUMMY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Dummy register: this peripheral is not implemented yet"]
pub mod dummy;
