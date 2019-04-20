#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TCR {
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
pub type WLENR = super::rcr::WLENR;
#[doc = "Values that can be written to the field `wlen`"]
pub type WLENW = super::rcr::WLENW;
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
        self.variant(super::rcr::WLENW::IGNORE)
    }
    #[doc = "12-bit data resolution of the receiver"]
    #[inline]
    pub fn resolution12(self) -> &'a mut W {
        self.variant(super::rcr::WLENW::RESOLUTION12)
    }
    #[doc = "16-bit data resolution of the receiver"]
    #[inline]
    pub fn resolution16(self) -> &'a mut W {
        self.variant(super::rcr::WLENW::RESOLUTION16)
    }
    #[doc = "20-bit data resolution of the receiver"]
    #[inline]
    pub fn resolution20(self) -> &'a mut W {
        self.variant(super::rcr::WLENW::RESOLUTION20)
    }
    #[doc = "24-bit data resolution of the receiver"]
    #[inline]
    pub fn resolution24(self) -> &'a mut W {
        self.variant(super::rcr::WLENW::RESOLUTION24)
    }
    #[doc = "32-bit data resolution of the receiver"]
    #[inline]
    pub fn resolution32(self) -> &'a mut W {
        self.variant(super::rcr::WLENW::RESOLUTION32)
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
    #[doc = "Bits 0:2 - Desired data resolution of transmitter"]
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
    #[doc = "Bits 0:2 - Desired data resolution of transmitter"]
    #[inline]
    pub fn wlen(&mut self) -> _WLENW {
        _WLENW { w: self }
    }
}
