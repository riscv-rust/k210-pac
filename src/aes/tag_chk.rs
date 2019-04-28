#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TAG_CHK {
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
#[doc = "Possible values of the field `tag_chk`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAG_CHKR {
    #[doc = "Check not finished"]
    BUSY,
    #[doc = "Check failed"]
    FAIL,
    #[doc = "Check success"]
    SUCCESS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TAG_CHKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TAG_CHKR::BUSY => 0,
            TAG_CHKR::FAIL => 1,
            TAG_CHKR::SUCCESS => 2,
            TAG_CHKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TAG_CHKR {
        match value {
            0 => TAG_CHKR::BUSY,
            1 => TAG_CHKR::FAIL,
            2 => TAG_CHKR::SUCCESS,
            i => TAG_CHKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline]
    pub fn is_busy(&self) -> bool {
        *self == TAG_CHKR::BUSY
    }
    #[doc = "Checks if the value of the field is `FAIL`"]
    #[inline]
    pub fn is_fail(&self) -> bool {
        *self == TAG_CHKR::FAIL
    }
    #[doc = "Checks if the value of the field is `SUCCESS`"]
    #[inline]
    pub fn is_success(&self) -> bool {
        *self == TAG_CHKR::SUCCESS
    }
}
#[doc = "Values that can be written to the field `tag_chk`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAG_CHKW {
    #[doc = "Check not finished"]
    BUSY,
    #[doc = "Check failed"]
    FAIL,
    #[doc = "Check success"]
    SUCCESS,
}
impl TAG_CHKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TAG_CHKW::BUSY => 0,
            TAG_CHKW::FAIL => 1,
            TAG_CHKW::SUCCESS => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAG_CHKW<'a> {
    w: &'a mut W,
}
impl<'a> _TAG_CHKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAG_CHKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Check not finished"]
    #[inline]
    pub fn busy(self) -> &'a mut W {
        self.variant(TAG_CHKW::BUSY)
    }
    #[doc = "Check failed"]
    #[inline]
    pub fn fail(self) -> &'a mut W {
        self.variant(TAG_CHKW::FAIL)
    }
    #[doc = "Check success"]
    #[inline]
    pub fn success(self) -> &'a mut W {
        self.variant(TAG_CHKW::SUCCESS)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Tag check status"]
    #[inline]
    pub fn tag_chk(&self) -> TAG_CHKR {
        TAG_CHKR::_from({
            const MASK: u8 = 3;
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
    #[doc = "Bits 0:1 - Tag check status"]
    #[inline]
    pub fn tag_chk(&mut self) -> _TAG_CHKW {
        _TAG_CHKW { w: self }
    }
}
