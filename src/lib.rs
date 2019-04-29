#![doc = "Peripheral access API for K210 microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate riscv;
#[cfg(feature = "rt")]
extern crate riscv_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = "Core Local Interruptor"]
pub struct CLINT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLINT {}
impl CLINT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const clint::RegisterBlock {
        33554432 as *const _
    }
}
impl Deref for CLINT {
    type Target = clint::RegisterBlock;
    fn deref(&self) -> &clint::RegisterBlock {
        unsafe { &*CLINT::ptr() }
    }
}
#[doc = "Core Local Interruptor"]
pub mod clint;
#[doc = "Platform-Level Interrupt Controller"]
pub struct PLIC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PLIC {}
impl PLIC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const plic::RegisterBlock {
        201326592 as *const _
    }
}
impl Deref for PLIC {
    type Target = plic::RegisterBlock;
    fn deref(&self) -> &plic::RegisterBlock {
        unsafe { &*PLIC::ptr() }
    }
}
#[doc = "Platform-Level Interrupt Controller"]
pub mod plic;
#[doc = "High-speed UART"]
pub struct UARTHS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UARTHS {}
impl UARTHS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uarths::RegisterBlock {
        939524096 as *const _
    }
}
impl Deref for UARTHS {
    type Target = uarths::RegisterBlock;
    fn deref(&self) -> &uarths::RegisterBlock {
        unsafe { &*UARTHS::ptr() }
    }
}
#[doc = "High-speed UART"]
pub mod uarths;
#[doc = "High-speed GPIO"]
pub struct GPIOHS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOHS {}
impl GPIOHS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiohs::RegisterBlock {
        939528192 as *const _
    }
}
impl Deref for GPIOHS {
    type Target = gpiohs::RegisterBlock;
    fn deref(&self) -> &gpiohs::RegisterBlock {
        unsafe { &*GPIOHS::ptr() }
    }
}
#[doc = "High-speed GPIO"]
pub mod gpiohs;
#[doc = "Neural Network Accelerator"]
pub struct KPU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for KPU {}
impl KPU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const kpu::RegisterBlock {
        1082130432 as *const _
    }
}
impl Deref for KPU {
    type Target = kpu::RegisterBlock;
    fn deref(&self) -> &kpu::RegisterBlock {
        unsafe { &*KPU::ptr() }
    }
}
#[doc = "Neural Network Accelerator"]
pub mod kpu;
#[doc = "Direct Memory Access Controller"]
pub struct DMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAC {}
impl DMAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dmac::RegisterBlock {
        1342177280 as *const _
    }
}
impl Deref for DMAC {
    type Target = dmac::RegisterBlock;
    fn deref(&self) -> &dmac::RegisterBlock {
        unsafe { &*DMAC::ptr() }
    }
}
#[doc = "Direct Memory Access Controller"]
pub mod dmac;
#[doc = "General Purpose Input/Output Interface"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio::RegisterBlock {
        1344274432 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    fn deref(&self) -> &gpio::RegisterBlock {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "General Purpose Input/Output Interface"]
pub mod gpio;
#[doc = "Universal Asynchronous Receiver-Transmitter 1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart1::RegisterBlock {
        1344339968 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    fn deref(&self) -> &uart1::RegisterBlock {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver-Transmitter 1"]
pub mod uart1;
#[doc = "Universal Asynchronous Receiver-Transmitter 2"]
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART2 {}
impl UART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart1::RegisterBlock {
        1344405504 as *const _
    }
}
impl Deref for UART2 {
    type Target = uart1::RegisterBlock;
    fn deref(&self) -> &uart1::RegisterBlock {
        unsafe { &*UART2::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver-Transmitter 3"]
pub struct UART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART3 {}
impl UART3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart1::RegisterBlock {
        1344471040 as *const _
    }
}
impl Deref for UART3 {
    type Target = uart1::RegisterBlock;
    fn deref(&self) -> &uart1::RegisterBlock {
        unsafe { &*UART3::ptr() }
    }
}
#[doc = "Serial Peripheral Interface 0 (master)"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi0::RegisterBlock {
        1375731712 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &spi0::RegisterBlock {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "Serial Peripheral Interface 0 (master)"]
pub mod spi0;
#[doc = "Serial Peripheral Interface 1 (master)"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi0::RegisterBlock {
        1392508928 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &spi0::RegisterBlock {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial Peripheral Interface 2 (slave)"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi2::RegisterBlock {
        1344536576 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi2::RegisterBlock;
    fn deref(&self) -> &spi2::RegisterBlock {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "Serial Peripheral Interface 2 (slave)"]
pub mod spi2;
#[doc = "Serial Peripheral Interface 3 (master)"]
pub struct SPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI3 {}
impl SPI3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi3::RegisterBlock {
        1409286144 as *const _
    }
}
impl Deref for SPI3 {
    type Target = spi3::RegisterBlock;
    fn deref(&self) -> &spi3::RegisterBlock {
        unsafe { &*SPI3::ptr() }
    }
}
#[doc = "Serial Peripheral Interface 3 (master)"]
pub mod spi3;
#[doc = "Inter-Integrated Sound Interface 0"]
pub struct I2S0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S0 {}
impl I2S0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s0::RegisterBlock {
        1344602112 as *const _
    }
}
impl Deref for I2S0 {
    type Target = i2s0::RegisterBlock;
    fn deref(&self) -> &i2s0::RegisterBlock {
        unsafe { &*I2S0::ptr() }
    }
}
#[doc = "Inter-Integrated Sound Interface 0"]
pub mod i2s0;
#[doc = "Audio Processor"]
pub struct APU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for APU {}
impl APU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const apu::RegisterBlock {
        1344602624 as *const _
    }
}
impl Deref for APU {
    type Target = apu::RegisterBlock;
    fn deref(&self) -> &apu::RegisterBlock {
        unsafe { &*APU::ptr() }
    }
}
#[doc = "Audio Processor"]
pub mod apu;
#[doc = "Inter-Integrated Sound Interface 1"]
pub struct I2S1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S1 {}
impl I2S1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s0::RegisterBlock {
        1344667648 as *const _
    }
}
impl Deref for I2S1 {
    type Target = i2s0::RegisterBlock;
    fn deref(&self) -> &i2s0::RegisterBlock {
        unsafe { &*I2S1::ptr() }
    }
}
#[doc = "Inter-Integrated Sound Interface 2"]
pub struct I2S2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S2 {}
impl I2S2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s0::RegisterBlock {
        1344733184 as *const _
    }
}
impl Deref for I2S2 {
    type Target = i2s0::RegisterBlock;
    fn deref(&self) -> &i2s0::RegisterBlock {
        unsafe { &*I2S2::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit Bus 0"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1344798720 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit Bus 0"]
pub mod i2c0;
#[doc = "Inter-Integrated Circuit Bus 1"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1344864256 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit Bus 2"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1344929792 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "Field Programmable IO Array"]
pub struct FPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FPIOA {}
impl FPIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fpioa::RegisterBlock {
        1344995328 as *const _
    }
}
impl Deref for FPIOA {
    type Target = fpioa::RegisterBlock;
    fn deref(&self) -> &fpioa::RegisterBlock {
        unsafe { &*FPIOA::ptr() }
    }
}
#[doc = "Field Programmable IO Array"]
pub mod fpioa;
#[doc = "SHA256 Accelerator"]
pub struct SHA256 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SHA256 {}
impl SHA256 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sha256::RegisterBlock {
        1345060864 as *const _
    }
}
impl Deref for SHA256 {
    type Target = sha256::RegisterBlock;
    fn deref(&self) -> &sha256::RegisterBlock {
        unsafe { &*SHA256::ptr() }
    }
}
#[doc = "SHA256 Accelerator"]
pub mod sha256;
#[doc = "Timer 0"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1345126400 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "Timer 0"]
pub mod timer0;
#[doc = "Timer 1"]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1345191936 as *const _
    }
}
impl Deref for TIMER1 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER1::ptr() }
    }
}
#[doc = "Timer 2"]
pub struct TIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2 {}
impl TIMER2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1345257472 as *const _
    }
}
impl Deref for TIMER2 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER2::ptr() }
    }
}
#[doc = "Watchdog Timer 0"]
pub struct WDT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT0 {}
impl WDT0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdt0::RegisterBlock {
        1346371584 as *const _
    }
}
impl Deref for WDT0 {
    type Target = wdt0::RegisterBlock;
    fn deref(&self) -> &wdt0::RegisterBlock {
        unsafe { &*WDT0::ptr() }
    }
}
#[doc = "Watchdog Timer 0"]
pub mod wdt0;
#[doc = "Watchdog Timer 1"]
pub struct WDT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT1 {}
impl WDT1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdt0::RegisterBlock {
        1346437120 as *const _
    }
}
impl Deref for WDT1 {
    type Target = wdt0::RegisterBlock;
    fn deref(&self) -> &wdt0::RegisterBlock {
        unsafe { &*WDT1::ptr() }
    }
}
#[doc = "One-Time Programmable Memory Controller"]
pub struct OTP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTP {}
impl OTP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otp::RegisterBlock {
        1346502656 as *const _
    }
}
impl Deref for OTP {
    type Target = otp::RegisterBlock;
    fn deref(&self) -> &otp::RegisterBlock {
        unsafe { &*OTP::ptr() }
    }
}
#[doc = "One-Time Programmable Memory Controller"]
pub mod otp;
#[doc = "Digital Video Port"]
pub struct DVP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DVP {}
impl DVP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dvp::RegisterBlock {
        1346568192 as *const _
    }
}
impl Deref for DVP {
    type Target = dvp::RegisterBlock;
    fn deref(&self) -> &dvp::RegisterBlock {
        unsafe { &*DVP::ptr() }
    }
}
#[doc = "Digital Video Port"]
pub mod dvp;
#[doc = "System Controller"]
pub struct SYSCTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCTL {}
impl SYSCTL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sysctl::RegisterBlock {
        1346633728 as *const _
    }
}
impl Deref for SYSCTL {
    type Target = sysctl::RegisterBlock;
    fn deref(&self) -> &sysctl::RegisterBlock {
        unsafe { &*SYSCTL::ptr() }
    }
}
#[doc = "System Controller"]
pub mod sysctl;
#[doc = "AES Accelerator"]
pub struct AES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES {}
impl AES {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aes::RegisterBlock {
        1346699264 as *const _
    }
}
impl Deref for AES {
    type Target = aes::RegisterBlock;
    fn deref(&self) -> &aes::RegisterBlock {
        unsafe { &*AES::ptr() }
    }
}
#[doc = "AES Accelerator"]
pub mod aes;
#[doc = "Real Time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1346764800 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real Time Clock"]
pub mod rtc;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "CLINT"]
    pub CLINT: CLINT,
    #[doc = "PLIC"]
    pub PLIC: PLIC,
    #[doc = "UARTHS"]
    pub UARTHS: UARTHS,
    #[doc = "GPIOHS"]
    pub GPIOHS: GPIOHS,
    #[doc = "KPU"]
    pub KPU: KPU,
    #[doc = "DMAC"]
    pub DMAC: DMAC,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "UART2"]
    pub UART2: UART2,
    #[doc = "UART3"]
    pub UART3: UART3,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "SPI3"]
    pub SPI3: SPI3,
    #[doc = "I2S0"]
    pub I2S0: I2S0,
    #[doc = "APU"]
    pub APU: APU,
    #[doc = "I2S1"]
    pub I2S1: I2S1,
    #[doc = "I2S2"]
    pub I2S2: I2S2,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "FPIOA"]
    pub FPIOA: FPIOA,
    #[doc = "SHA256"]
    pub SHA256: SHA256,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "TIMER2"]
    pub TIMER2: TIMER2,
    #[doc = "WDT0"]
    pub WDT0: WDT0,
    #[doc = "WDT1"]
    pub WDT1: WDT1,
    #[doc = "OTP"]
    pub OTP: OTP,
    #[doc = "DVP"]
    pub DVP: DVP,
    #[doc = "SYSCTL"]
    pub SYSCTL: SYSCTL,
    #[doc = "AES"]
    pub AES: AES,
    #[doc = "RTC"]
    pub RTC: RTC,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        riscv::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            CLINT: CLINT {
                _marker: PhantomData,
            },
            PLIC: PLIC {
                _marker: PhantomData,
            },
            UARTHS: UARTHS {
                _marker: PhantomData,
            },
            GPIOHS: GPIOHS {
                _marker: PhantomData,
            },
            KPU: KPU {
                _marker: PhantomData,
            },
            DMAC: DMAC {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            UART2: UART2 {
                _marker: PhantomData,
            },
            UART3: UART3 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            SPI3: SPI3 {
                _marker: PhantomData,
            },
            I2S0: I2S0 {
                _marker: PhantomData,
            },
            APU: APU {
                _marker: PhantomData,
            },
            I2S1: I2S1 {
                _marker: PhantomData,
            },
            I2S2: I2S2 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            FPIOA: FPIOA {
                _marker: PhantomData,
            },
            SHA256: SHA256 {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER1: TIMER1 {
                _marker: PhantomData,
            },
            TIMER2: TIMER2 {
                _marker: PhantomData,
            },
            WDT0: WDT0 {
                _marker: PhantomData,
            },
            WDT1: WDT1 {
                _marker: PhantomData,
            },
            OTP: OTP {
                _marker: PhantomData,
            },
            DVP: DVP {
                _marker: PhantomData,
            },
            SYSCTL: SYSCTL {
                _marker: PhantomData,
            },
            AES: AES {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
        }
    }
}
