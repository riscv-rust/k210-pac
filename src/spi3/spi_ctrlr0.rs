#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SPI_CTRLR0 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `aitm`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AITMR {
    #[doc = "STANDARD"]
    STANDARD,
    #[doc = "ADDR_STANDARD"]
    ADDR_STANDARD,
    #[doc = "AS_FRAME_FORMAT"]
    AS_FRAME_FORMAT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AITMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AITMR::STANDARD => 0,
            AITMR::ADDR_STANDARD => 1,
            AITMR::AS_FRAME_FORMAT => 2,
            AITMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AITMR {
        match value {
            0 => AITMR::STANDARD,
            1 => AITMR::ADDR_STANDARD,
            2 => AITMR::AS_FRAME_FORMAT,
            i => AITMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline]
    pub fn is_standard(&self) -> bool {
        *self == AITMR::STANDARD
    }
    #[doc = "Checks if the value of the field is `ADDR_STANDARD`"]
    #[inline]
    pub fn is_addr_standard(&self) -> bool {
        *self == AITMR::ADDR_STANDARD
    }
    #[doc = "Checks if the value of the field is `AS_FRAME_FORMAT`"]
    #[inline]
    pub fn is_as_frame_format(&self) -> bool {
        *self == AITMR::AS_FRAME_FORMAT
    }
}
#[doc = r" Value of the field"]
pub struct ADDR_LENGTHR {
    bits: u8,
}
impl ADDR_LENGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INST_LENGTHR {
    bits: u8,
}
impl INST_LENGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WAIT_CYCLESR {
    bits: u8,
}
impl WAIT_CYCLESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `aitm`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AITMW {
    #[doc = "STANDARD"]
    STANDARD,
    #[doc = "ADDR_STANDARD"]
    ADDR_STANDARD,
    #[doc = "AS_FRAME_FORMAT"]
    AS_FRAME_FORMAT,
}
impl AITMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AITMW::STANDARD => 0,
            AITMW::ADDR_STANDARD => 1,
            AITMW::AS_FRAME_FORMAT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AITMW<'a> {
    w: &'a mut W,
}
impl<'a> _AITMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AITMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "STANDARD"]
    #[inline]
    pub fn standard(self) -> &'a mut W {
        self.variant(AITMW::STANDARD)
    }
    #[doc = "ADDR_STANDARD"]
    #[inline]
    pub fn addr_standard(self) -> &'a mut W {
        self.variant(AITMW::ADDR_STANDARD)
    }
    #[doc = "AS_FRAME_FORMAT"]
    #[inline]
    pub fn as_frame_format(self) -> &'a mut W {
        self.variant(AITMW::AS_FRAME_FORMAT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADDR_LENGTHW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDR_LENGTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INST_LENGTHW<'a> {
    w: &'a mut W,
}
impl<'a> _INST_LENGTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WAIT_CYCLESW<'a> {
    w: &'a mut W,
}
impl<'a> _WAIT_CYCLESW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - instruction_address_trans_mode"]
    #[inline]
    pub fn aitm(&self) -> AITMR {
        AITMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:5 - ADDR_LENGTH"]
    #[inline]
    pub fn addr_length(&self) -> ADDR_LENGTHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADDR_LENGTHR { bits }
    }
    #[doc = "Bits 8:9 - INSTRUCTION_LENGTH"]
    #[inline]
    pub fn inst_length(&self) -> INST_LENGTHR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INST_LENGTHR { bits }
    }
    #[doc = "Bits 11:15 - WAIT_CYCLES"]
    #[inline]
    pub fn wait_cycles(&self) -> WAIT_CYCLESR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WAIT_CYCLESR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - instruction_address_trans_mode"]
    #[inline]
    pub fn aitm(&mut self) -> _AITMW {
        _AITMW { w: self }
    }
    #[doc = "Bits 2:5 - ADDR_LENGTH"]
    #[inline]
    pub fn addr_length(&mut self) -> _ADDR_LENGTHW {
        _ADDR_LENGTHW { w: self }
    }
    #[doc = "Bits 8:9 - INSTRUCTION_LENGTH"]
    #[inline]
    pub fn inst_length(&mut self) -> _INST_LENGTHW {
        _INST_LENGTHW { w: self }
    }
    #[doc = "Bits 11:15 - WAIT_CYCLES"]
    #[inline]
    pub fn wait_cycles(&mut self) -> _WAIT_CYCLESW {
        _WAIT_CYCLESW { w: self }
    }
}
