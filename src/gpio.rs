#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data (output) registers"]
    pub data_output: DATA_OUTPUT,
    #[doc = "0x04 - Data direction registers"]
    pub direction: DIRECTION,
    #[doc = "0x08 - Data source registers"]
    pub source: SOURCE,
    _reserved0: [u8; 36usize],
    #[doc = "0x30 - Interrupt enable/disable registers"]
    pub interrupt_enable: INTERRUPT_ENABLE,
    #[doc = "0x34 - Interrupt mask registers"]
    pub interrupt_mask: INTERRUPT_MASK,
    #[doc = "0x38 - Interrupt level registers"]
    pub interrupt_level: INTERRUPT_LEVEL,
    #[doc = "0x3c - Interrupt polarity registers"]
    pub interrupt_polarity: INTERRUPT_POLARITY,
    #[doc = "0x40 - Interrupt status registers"]
    pub interrupt_status: INTERRUPT_STATUS,
    #[doc = "0x44 - Raw interrupt status registers"]
    pub interrupt_status_raw: INTERRUPT_STATUS_RAW,
    #[doc = "0x48 - Interrupt debounce registers"]
    pub interrupt_debounce: INTERRUPT_DEBOUNCE,
    #[doc = "0x4c - Registers for clearing interrupts"]
    pub interrupt_clear: INTERRUPT_CLEAR,
    #[doc = "0x50 - External port (data input) registers"]
    pub data_input: DATA_INPUT,
    _reserved1: [u8; 12usize],
    #[doc = "0x60 - Sync level registers"]
    pub sync_level: SYNC_LEVEL,
    #[doc = "0x64 - ID code"]
    pub id_code: ID_CODE,
    #[doc = "0x68 - Interrupt both edge type"]
    pub interrupt_bothedge: INTERRUPT_BOTHEDGE,
}
#[doc = "Data (output) registers"]
pub struct DATA_OUTPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data (output) registers"]
pub mod data_output;
#[doc = "Data direction registers"]
pub struct DIRECTION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data direction registers"]
pub mod direction;
#[doc = "Data source registers"]
pub struct SOURCE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data source registers"]
pub mod source;
#[doc = "Interrupt enable/disable registers"]
pub struct INTERRUPT_ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable/disable registers"]
pub mod interrupt_enable;
#[doc = "Interrupt mask registers"]
pub struct INTERRUPT_MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt mask registers"]
pub mod interrupt_mask;
#[doc = "Interrupt level registers"]
pub struct INTERRUPT_LEVEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt level registers"]
pub mod interrupt_level;
#[doc = "Interrupt polarity registers"]
pub struct INTERRUPT_POLARITY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt polarity registers"]
pub mod interrupt_polarity;
#[doc = "Interrupt status registers"]
pub struct INTERRUPT_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt status registers"]
pub mod interrupt_status;
#[doc = "Raw interrupt status registers"]
pub struct INTERRUPT_STATUS_RAW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw interrupt status registers"]
pub mod interrupt_status_raw;
#[doc = "Interrupt debounce registers"]
pub struct INTERRUPT_DEBOUNCE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt debounce registers"]
pub mod interrupt_debounce;
#[doc = "Registers for clearing interrupts"]
pub struct INTERRUPT_CLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Registers for clearing interrupts"]
pub mod interrupt_clear;
#[doc = "External port (data input) registers"]
pub struct DATA_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External port (data input) registers"]
pub mod data_input;
#[doc = "Sync level registers"]
pub struct SYNC_LEVEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sync level registers"]
pub mod sync_level;
#[doc = "ID code"]
pub struct ID_CODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ID code"]
pub mod id_code;
#[doc = "Interrupt both edge type"]
pub struct INTERRUPT_BOTHEDGE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt both edge type"]
pub mod interrupt_bothedge;
