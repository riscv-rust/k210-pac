#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Input Value Register"]
    pub input_val: INPUT_VAL,
    #[doc = "0x04 - Pin Input Enable Register"]
    pub input_en: INPUT_EN,
    #[doc = "0x08 - Pin Output Enable Register"]
    pub output_en: OUTPUT_EN,
    #[doc = "0x0c - Output Value Register"]
    pub output_val: OUTPUT_VAL,
    #[doc = "0x10 - Internal Pull-Up Enable Register"]
    pub pullup_en: PULLUP_EN,
    #[doc = "0x14 - Drive Strength Register"]
    pub drive: DRIVE,
    #[doc = "0x18 - Rise Interrupt Enable Register"]
    pub rise_ie: RISE_IE,
    #[doc = "0x1c - Rise Interrupt Pending Register"]
    pub rise_ip: RISE_IP,
    #[doc = "0x20 - Fall Interrupt Enable Register"]
    pub fall_ie: FALL_IE,
    #[doc = "0x24 - Fall Interrupt Pending Register"]
    pub fall_ip: FALL_IP,
    #[doc = "0x28 - High Interrupt Enable Register"]
    pub high_ie: HIGH_IE,
    #[doc = "0x2c - High Interrupt Pending Register"]
    pub high_ip: HIGH_IP,
    #[doc = "0x30 - Low Interrupt Enable Register"]
    pub low_ie: LOW_IE,
    #[doc = "0x34 - Low Interrupt Pending Register"]
    pub low_ip: LOW_IP,
    #[doc = "0x38 - HW I/O Function Enable Register"]
    pub iof_en: IOF_EN,
    #[doc = "0x3c - HW I/O Function Select Register"]
    pub iof_sel: IOF_SEL,
    #[doc = "0x40 - Output XOR (invert) Register"]
    pub output_xor: OUTPUT_XOR,
}
#[doc = "Input Value Register"]
pub struct INPUT_VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Value Register"]
pub mod input_val;
#[doc = "Pin Input Enable Register"]
pub struct INPUT_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Input Enable Register"]
pub mod input_en;
#[doc = "Pin Output Enable Register"]
pub struct OUTPUT_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Output Enable Register"]
pub mod output_en;
#[doc = "Output Value Register"]
pub struct OUTPUT_VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Value Register"]
pub mod output_val;
#[doc = "Internal Pull-Up Enable Register"]
pub struct PULLUP_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal Pull-Up Enable Register"]
pub mod pullup_en;
#[doc = "Drive Strength Register"]
pub struct DRIVE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Drive Strength Register"]
pub mod drive;
#[doc = "Rise Interrupt Enable Register"]
pub struct RISE_IE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rise Interrupt Enable Register"]
pub mod rise_ie;
#[doc = "Rise Interrupt Pending Register"]
pub struct RISE_IP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rise Interrupt Pending Register"]
pub mod rise_ip;
#[doc = "Fall Interrupt Enable Register"]
pub struct FALL_IE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fall Interrupt Enable Register"]
pub mod fall_ie;
#[doc = "Fall Interrupt Pending Register"]
pub struct FALL_IP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fall Interrupt Pending Register"]
pub mod fall_ip;
#[doc = "High Interrupt Enable Register"]
pub struct HIGH_IE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "High Interrupt Enable Register"]
pub mod high_ie;
#[doc = "High Interrupt Pending Register"]
pub struct HIGH_IP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "High Interrupt Pending Register"]
pub mod high_ip;
#[doc = "Low Interrupt Enable Register"]
pub struct LOW_IE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Interrupt Enable Register"]
pub mod low_ie;
#[doc = "Low Interrupt Pending Register"]
pub struct LOW_IP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Interrupt Pending Register"]
pub mod low_ip;
#[doc = "HW I/O Function Enable Register"]
pub struct IOF_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HW I/O Function Enable Register"]
pub mod iof_en;
#[doc = "HW I/O Function Select Register"]
pub struct IOF_SEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HW I/O Function Select Register"]
pub mod iof_sel;
#[doc = "Output XOR (invert) Register"]
pub struct OUTPUT_XOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output XOR (invert) Register"]
pub mod output_xor;
