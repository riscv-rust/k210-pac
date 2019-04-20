#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RCR {
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
#[doc = "Possible values of the field `wlen`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WLENR {
    #[doc = "Ignore the word length"]
    IGNORE,
    #[doc = "12-bit data resolution of the receiver"]
    RESOLUTION12,
    #[doc = "16-bit data resolution of the receiver"]
    RESOLUTION16,
    #[doc = "20-bit data resolution of the receiver"]
    RESOLUTION20,
    #[doc = "24-bit data resolution of the receiver"]
    RESOLUTION24,
    #[doc = "32-bit data resolution of the receiver"]
    RESOLUTION32,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WLENR::IGNORE => 0,
            WLENR::RESOLUTION12 => 1,
            WLENR::RESOLUTION16 => 2,
            WLENR::RESOLUTION20 => 3,
            WLENR::RESOLUTION24 => 4,
            WLENR::RESOLUTION32 => 5,
            WLENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WLENR {
        match value {
            0 => WLENR::IGNORE,
            1 => WLENR::RESOLUTION12,
            2 => WLENR::RESOLUTION16,
            3 => WLENR::RESOLUTION20,
            4 => WLENR::RESOLUTION24,
            5 => WLENR::RESOLUTION32,
            i => WLENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IGNORE`"]
    #[inline]
    pub fn is_ignore(&self) -> bool {
        *self == WLENR::IGNORE
    }
    #[doc = "Checks if the value of the field is `RESOLUTION12`"]
    #[inline]
    pub fn is_resolution12(&self) -> bool {
        *self == WLENR::RESOLUTION12
    }
    #[doc = "Checks if the value of the field is `RESOLUTION16`"]
    #[inline]
    pub fn is_resolution16(&self) -> bool {
        *self == WLENR::RESOLUTION16
    }
    #[doc = "Checks if the value of the field is `RESOLUTION20`"]
    #[inline]
    pub fn is_resolution20(&self) -> bool {
        *self == WLENR::RESOLUTION20
    }
    #[doc = "Checks if the value of the field is `RESOLUTION24`"]
    #[inline]
    pub fn is_resolution24(&self) -> bool {
        *self == WLENR::RESOLUTION24
    }
    #[doc = "Checks if the value of the field is `RESOLUTION32`"]
    #[inline]
    pub fn is_resolution32(&self) -> bool {
        *self == WLENR::RESOLUTION32
    }
}
#[doc = "Values that can be written to the field `wlen`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WLENW {
    #[doc = "Ignore the word length"]
    IGNORE,
    #[doc = "12-bit data resolution of the receiver"]
    RESOLUTION12,
    #[doc = "16-bit data resolution of the receiver"]
    RESOLUTION16,
    #[doc = "20-bit data resolution of the receiver"]
    RESOLUTION20,
    #[doc = "24-bit data resolution of the receiver"]
    RESOLUTION24,
    #[doc = "32-bit data resolution of the receiver"]
    RESOLUTION32,
}
impl WLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WLENW::IGNORE => 0,
            WLENW::RESOLUTION12 => 1,
            WLENW::RESOLUTION16 => 2,
            WLENW::RESOLUTION20 => 3,
            WLENW::RESOLUTION24 => 4,
            WLENW::RESOLUTION32 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WLENW<'a> {
    w: &'a mut W,
}
impl<'a> _WLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WLENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Ignore the word length"]
    #[inline]
    pub fn ignore(self) -> &'a mut W {
        self.variant(WLENW::IGNORE)
    }
    #[doc = "12-bit data resolution of the receiver"]
    #[inline]
    pub fn resolution12(self) -> &'a mut W {
        self.variant(WLENW::RESOLUTION12)
    }
    #[doc = "16-bit data resolution of the receiver"]
    #[inline]
    pub fn resolution16(self) -> &'a mut W {
        self.variant(WLENW::RESOLUTION16)
    }
    #[doc = "20-bit data resolution of the receiver"]
    #[inline]
    pub fn resolution20(self) -> &'a mut W {
        self.variant(WLENW::RESOLUTION20)
    }
    #[doc = "24-bit data resolution of the receiver"]
    #[inline]
    pub fn resolution24(self) -> &'a mut W {
        self.variant(WLENW::RESOLUTION24)
    }
    #[doc = "32-bit data resolution of the receiver"]
    #[inline]
    pub fn resolution32(self) -> &'a mut W {
        self.variant(WLENW::RESOLUTION32)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:2 - Desired data resolution of receiver"]
    #[inline]
    pub fn wlen(&self) -> WLENR {
        WLENR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:2 - Desired data resolution of receiver"]
    #[inline]
    pub fn wlen(&mut self) -> _WLENW {
        _WLENW { w: self }
    }
}
