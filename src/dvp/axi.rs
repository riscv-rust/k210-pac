#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AXI {
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
#[doc = "Possible values of the field `gm_mlen`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GM_MLENR {
    #[doc = "GM_MLEN_1BYTE"]
    BYTE1,
    #[doc = "GM_MLEN_4BYTE"]
    BYTE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GM_MLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GM_MLENR::BYTE1 => 0,
            GM_MLENR::BYTE4 => 3,
            GM_MLENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GM_MLENR {
        match value {
            0 => GM_MLENR::BYTE1,
            3 => GM_MLENR::BYTE4,
            i => GM_MLENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE1`"]
    #[inline]
    pub fn is_byte1(&self) -> bool {
        *self == GM_MLENR::BYTE1
    }
    #[doc = "Checks if the value of the field is `BYTE4`"]
    #[inline]
    pub fn is_byte4(&self) -> bool {
        *self == GM_MLENR::BYTE4
    }
}
#[doc = "Values that can be written to the field `gm_mlen`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GM_MLENW {
    #[doc = "GM_MLEN_1BYTE"]
    BYTE1,
    #[doc = "GM_MLEN_4BYTE"]
    BYTE4,
}
impl GM_MLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GM_MLENW::BYTE1 => 0,
            GM_MLENW::BYTE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GM_MLENW<'a> {
    w: &'a mut W,
}
impl<'a> _GM_MLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GM_MLENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GM_MLEN_1BYTE"]
    #[inline]
    pub fn byte1(self) -> &'a mut W {
        self.variant(GM_MLENW::BYTE1)
    }
    #[doc = "GM_MLEN_4BYTE"]
    #[inline]
    pub fn byte4(self) -> &'a mut W {
        self.variant(GM_MLENW::BYTE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - GM_MLEN"]
    #[inline]
    pub fn gm_mlen(&self) -> GM_MLENR {
        GM_MLENR::_from({
            const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - GM_MLEN"]
    #[inline]
    pub fn gm_mlen(&mut self) -> _GM_MLENW {
        _GM_MLENW { w: self }
    }
}
