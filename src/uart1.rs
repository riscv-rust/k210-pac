#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Receive Buffer Register / Divisor Latch (Low) / Transmit Holding Register (depending on context and R/W)"]
    pub rbr_dll_thr: RBR_DLL_THR,
    #[doc = "0x04 - Divisor Latch (High) / Interrupt Enable Register"]
    pub dlh_ier: DLH_IER,
    #[doc = "0x08 - FIFO Control Register / Interrupt Identification Register"]
    pub fcr_iir: FCR_IIR,
    #[doc = "0x0c - Line Control Register"]
    pub lcr: LCR,
    #[doc = "0x10 - Modem Control Register"]
    pub mcr: MCR,
    #[doc = "0x14 - Line Status Register"]
    pub lsr: LSR,
    #[doc = "0x18 - Modem Status Register"]
    pub msr: MSR,
    #[doc = "0x1c - Scratchpad Register"]
    pub scr: SCR,
    #[doc = "0x20 - Low Power Divisor Latch (Low) Register"]
    pub lpdll: LPDLL,
    #[doc = "0x24 - Low Power Divisor Latch (High) Register"]
    pub lpdlh: LPDLH,
    _reserved0: [u8; 8usize],
    #[doc = "0x30 - Shadow Receive Buffer Register / Shadow Transmit Holding Register (depending on R/W)"]
    pub srbr_sthr: [SRBR_STHR; 16],
    #[doc = "0x70 - FIFO Access Register"]
    pub far: FAR,
    #[doc = "0x74 - Transmit FIFO Read Register"]
    pub tfr: TFR,
    #[doc = "0x78 - Receive FIFO Write Register"]
    pub rfw: RFW,
    #[doc = "0x7c - UART Status Register"]
    pub usr: USR,
    #[doc = "0x80 - Transmit FIFO Level"]
    pub tfl: TFL,
    #[doc = "0x84 - Receive FIFO Level"]
    pub rfl: RFL,
    #[doc = "0x88 - Software Reset Register"]
    pub srr: SRR,
    #[doc = "0x8c - Shadow Request to Send Register"]
    pub srts: SRTS,
    #[doc = "0x90 - Shadow Break Control Register"]
    pub sbcr: SBCR,
    #[doc = "0x94 - Shadow DMA Mode"]
    pub sdmam: SDMAM,
    #[doc = "0x98 - Shadow FIFO Enable"]
    pub sfe: SFE,
    #[doc = "0x9c - Shadow RCVR Trigger Register"]
    pub srt: SRT,
    #[doc = "0xa0 - Shadow TX Empty Trigger Register"]
    pub stet: STET,
    #[doc = "0xa4 - Halt TX Regster"]
    pub htx: HTX,
    #[doc = "0xa8 - DMA Software Acknowledge Register"]
    pub dmasa: DMASA,
    #[doc = "0xac - Transfer Control Register"]
    pub tcr: TCR,
    #[doc = "0xb0 - DE Enable Register"]
    pub de_en: DE_EN,
    #[doc = "0xb4 - RE Enable Register"]
    pub re_en: RE_EN,
    #[doc = "0xb8 - DE Assertion Time Register"]
    pub det: DET,
    #[doc = "0xbc - Turn-Around Time Register"]
    pub tat: TAT,
    #[doc = "0xc0 - Divisor Latch (Fractional) Register"]
    pub dlf: DLF,
    #[doc = "0xc4 - Receive-Mode Address Register"]
    pub rar: RAR,
    #[doc = "0xc8 - Transmit-Mode Address Register"]
    pub tar: TAR,
    #[doc = "0xcc - Line Control Register (Extended)"]
    pub lcr_ext: LCR_EXT,
    _reserved1: [u8; 36usize],
    #[doc = "0xf4 - Component Parameter Register"]
    pub cpr: CPR,
    #[doc = "0xf8 - UART Component Version"]
    pub ucv: UCV,
    #[doc = "0xfc - Component Type Register"]
    pub ctr: CTR,
}
#[doc = "Receive Buffer Register / Divisor Latch (Low) / Transmit Holding Register (depending on context and R/W)"]
pub struct RBR_DLL_THR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Buffer Register / Divisor Latch (Low) / Transmit Holding Register (depending on context and R/W)"]
pub mod rbr_dll_thr;
#[doc = "Divisor Latch (High) / Interrupt Enable Register"]
pub struct DLH_IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Divisor Latch (High) / Interrupt Enable Register"]
pub mod dlh_ier;
#[doc = "FIFO Control Register / Interrupt Identification Register"]
pub struct FCR_IIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Control Register / Interrupt Identification Register"]
pub mod fcr_iir;
#[doc = "Line Control Register"]
pub struct LCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Line Control Register"]
pub mod lcr;
#[doc = "Modem Control Register"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Modem Control Register"]
pub mod mcr;
#[doc = "Line Status Register"]
pub struct LSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Line Status Register"]
pub mod lsr;
#[doc = "Modem Status Register"]
pub struct MSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Modem Status Register"]
pub mod msr;
#[doc = "Scratchpad Register"]
pub struct SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scratchpad Register"]
pub mod scr;
#[doc = "Low Power Divisor Latch (Low) Register"]
pub struct LPDLL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Power Divisor Latch (Low) Register"]
pub mod lpdll;
#[doc = "Low Power Divisor Latch (High) Register"]
pub struct LPDLH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Power Divisor Latch (High) Register"]
pub mod lpdlh;
#[doc = "Shadow Receive Buffer Register / Shadow Transmit Holding Register (depending on R/W)"]
pub struct SRBR_STHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Receive Buffer Register / Shadow Transmit Holding Register (depending on R/W)"]
pub mod srbr_sthr;
#[doc = "FIFO Access Register"]
pub struct FAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Access Register"]
pub mod far;
#[doc = "Transmit FIFO Read Register"]
pub struct TFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit FIFO Read Register"]
pub mod tfr;
#[doc = "Receive FIFO Write Register"]
pub struct RFW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO Write Register"]
pub mod rfw;
#[doc = "UART Status Register"]
pub struct USR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Status Register"]
pub mod usr;
#[doc = "Transmit FIFO Level"]
pub struct TFL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit FIFO Level"]
pub mod tfl;
#[doc = "Receive FIFO Level"]
pub struct RFL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO Level"]
pub mod rfl;
#[doc = "Software Reset Register"]
pub struct SRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Reset Register"]
pub mod srr;
#[doc = "Shadow Request to Send Register"]
pub struct SRTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Request to Send Register"]
pub mod srts;
#[doc = "Shadow Break Control Register"]
pub struct SBCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Break Control Register"]
pub mod sbcr;
#[doc = "Shadow DMA Mode"]
pub struct SDMAM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow DMA Mode"]
pub mod sdmam;
#[doc = "Shadow FIFO Enable"]
pub struct SFE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow FIFO Enable"]
pub mod sfe;
#[doc = "Shadow RCVR Trigger Register"]
pub struct SRT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow RCVR Trigger Register"]
pub mod srt;
#[doc = "Shadow TX Empty Trigger Register"]
pub struct STET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow TX Empty Trigger Register"]
pub mod stet;
#[doc = "Halt TX Regster"]
pub struct HTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Halt TX Regster"]
pub mod htx;
#[doc = "DMA Software Acknowledge Register"]
pub struct DMASA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Software Acknowledge Register"]
pub mod dmasa;
#[doc = "Transfer Control Register"]
pub struct TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer Control Register"]
pub mod tcr;
#[doc = "DE Enable Register"]
pub struct DE_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DE Enable Register"]
pub mod de_en;
#[doc = "RE Enable Register"]
pub struct RE_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RE Enable Register"]
pub mod re_en;
#[doc = "DE Assertion Time Register"]
pub struct DET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DE Assertion Time Register"]
pub mod det;
#[doc = "Turn-Around Time Register"]
pub struct TAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Turn-Around Time Register"]
pub mod tat;
#[doc = "Divisor Latch (Fractional) Register"]
pub struct DLF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Divisor Latch (Fractional) Register"]
pub mod dlf;
#[doc = "Receive-Mode Address Register"]
pub struct RAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive-Mode Address Register"]
pub mod rar;
#[doc = "Transmit-Mode Address Register"]
pub struct TAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit-Mode Address Register"]
pub mod tar;
#[doc = "Line Control Register (Extended)"]
pub struct LCR_EXT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Line Control Register (Extended)"]
pub mod lcr_ext;
#[doc = "Component Parameter Register"]
pub struct CPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Component Parameter Register"]
pub mod cpr;
#[doc = "UART Component Version"]
pub struct UCV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Component Version"]
pub mod ucv;
#[doc = "Component Type Register"]
pub struct CTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Component Type Register"]
pub mod ctr;
