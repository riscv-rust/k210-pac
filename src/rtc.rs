#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer date information"]
    pub date: DATE,
    #[doc = "0x04 - Timer time information"]
    pub time: TIME,
    #[doc = "0x08 - Alarm date information"]
    pub alarm_date: ALARM_DATE,
    #[doc = "0x0c - Alarm time information"]
    pub alarm_time: ALARM_TIME,
    #[doc = "0x10 - Timer counter initial value"]
    pub initial_count: INITIAL_COUNT,
    #[doc = "0x14 - Timer counter current value"]
    pub current_count: CURRENT_COUNT,
    #[doc = "0x18 - RTC interrupt settings"]
    pub interrupt_ctrl: INTERRUPT_CTRL,
    #[doc = "0x1c - RTC register settings"]
    pub register_ctrl: REGISTER_CTRL,
    _reserved0: [u8; 8usize],
    #[doc = "0x28 - Timer extended information"]
    pub extended: EXTENDED,
}
#[doc = "Timer date information"]
pub struct DATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer date information"]
pub mod date;
#[doc = "Timer time information"]
pub struct TIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer time information"]
pub mod time;
#[doc = "Alarm date information"]
pub struct ALARM_DATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alarm date information"]
pub mod alarm_date;
#[doc = "Alarm time information"]
pub struct ALARM_TIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alarm time information"]
pub mod alarm_time;
#[doc = "Timer counter initial value"]
pub struct INITIAL_COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer counter initial value"]
pub mod initial_count;
#[doc = "Timer counter current value"]
pub struct CURRENT_COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer counter current value"]
pub mod current_count;
#[doc = "RTC interrupt settings"]
pub struct INTERRUPT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC interrupt settings"]
pub mod interrupt_ctrl;
#[doc = "RTC register settings"]
pub struct REGISTER_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC register settings"]
pub mod register_ctrl;
#[doc = "Timer extended information"]
pub struct EXTENDED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer extended information"]
pub mod extended;
