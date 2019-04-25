#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCCB_CFG {
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
#[doc = "Possible values of the field `byte_num`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYTE_NUMR {
    #[doc = "BYTE_NUM_2"]
    NUM2,
    #[doc = "BYTE_NUM_3"]
    NUM3,
    #[doc = "BYTE_NUM_4"]
    NUM4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BYTE_NUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BYTE_NUMR::NUM2 => 1,
            BYTE_NUMR::NUM3 => 2,
            BYTE_NUMR::NUM4 => 3,
            BYTE_NUMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BYTE_NUMR {
        match value {
            1 => BYTE_NUMR::NUM2,
            2 => BYTE_NUMR::NUM3,
            3 => BYTE_NUMR::NUM4,
            i => BYTE_NUMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NUM2`"]
    #[inline]
    pub fn is_num2(&self) -> bool {
        *self == BYTE_NUMR::NUM2
    }
    #[doc = "Checks if the value of the field is `NUM3`"]
    #[inline]
    pub fn is_num3(&self) -> bool {
        *self == BYTE_NUMR::NUM3
    }
    #[doc = "Checks if the value of the field is `NUM4`"]
    #[inline]
    pub fn is_num4(&self) -> bool {
        *self == BYTE_NUMR::NUM4
    }
}
#[doc = r" Value of the field"]
pub struct SCL_LCNTR {
    bits: u8,
}
impl SCL_LCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SCL_HCNTR {
    bits: u8,
}
impl SCL_HCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RDATAR {
    bits: u8,
}
impl RDATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `byte_num`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYTE_NUMW {
    #[doc = "BYTE_NUM_2"]
    NUM2,
    #[doc = "BYTE_NUM_3"]
    NUM3,
    #[doc = "BYTE_NUM_4"]
    NUM4,
}
impl BYTE_NUMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BYTE_NUMW::NUM2 => 1,
            BYTE_NUMW::NUM3 => 2,
            BYTE_NUMW::NUM4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BYTE_NUMW<'a> {
    w: &'a mut W,
}
impl<'a> _BYTE_NUMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BYTE_NUMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "BYTE_NUM_2"]
    #[inline]
    pub fn num2(self) -> &'a mut W {
        self.variant(BYTE_NUMW::NUM2)
    }
    #[doc = "BYTE_NUM_3"]
    #[inline]
    pub fn num3(self) -> &'a mut W {
        self.variant(BYTE_NUMW::NUM3)
    }
    #[doc = "BYTE_NUM_4"]
    #[inline]
    pub fn num4(self) -> &'a mut W {
        self.variant(BYTE_NUMW::NUM4)
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
pub struct _SCL_LCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _SCL_LCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SCL_HCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _SCL_HCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:1 - BYTE_NUM"]
    #[inline]
    pub fn byte_num(&self) -> BYTE_NUMR {
        BYTE_NUMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - SCL_LCNT"]
    #[inline]
    pub fn scl_lcnt(&self) -> SCL_LCNTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCL_LCNTR { bits }
    }
    #[doc = "Bits 16:23 - SCL_HCNT"]
    #[inline]
    pub fn scl_hcnt(&self) -> SCL_HCNTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCL_HCNTR { bits }
    }
    #[doc = "Bits 24:31 - RDATA"]
    #[inline]
    pub fn rdata(&self) -> RDATAR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RDATAR { bits }
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
    #[doc = "Bits 0:1 - BYTE_NUM"]
    #[inline]
    pub fn byte_num(&mut self) -> _BYTE_NUMW {
        _BYTE_NUMW { w: self }
    }
    #[doc = "Bits 8:15 - SCL_LCNT"]
    #[inline]
    pub fn scl_lcnt(&mut self) -> _SCL_LCNTW {
        _SCL_LCNTW { w: self }
    }
    #[doc = "Bits 16:23 - SCL_HCNT"]
    #[inline]
    pub fn scl_hcnt(&mut self) -> _SCL_HCNTW {
        _SCL_HCNTW { w: self }
    }
}
