#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EXTENDED {
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
#[doc = r" Value of the field"]
pub struct CENTURYR {
    bits: u8,
}
impl CENTURYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `leap_year`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEAP_YEARR {
    #[doc = "0 is not leap year"]
    NOT_LEAP,
    #[doc = "1 is leap year"]
    LEAP,
}
impl LEAP_YEARR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LEAP_YEARR::NOT_LEAP => false,
            LEAP_YEARR::LEAP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LEAP_YEARR {
        match value {
            false => LEAP_YEARR::NOT_LEAP,
            true => LEAP_YEARR::LEAP,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_LEAP`"]
    #[inline]
    pub fn is_not_leap(&self) -> bool {
        *self == LEAP_YEARR::NOT_LEAP
    }
    #[doc = "Checks if the value of the field is `LEAP`"]
    #[inline]
    pub fn is_leap(&self) -> bool {
        *self == LEAP_YEARR::LEAP
    }
}
#[doc = r" Proxy"]
pub struct _CENTURYW<'a> {
    w: &'a mut W,
}
impl<'a> _CENTURYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `leap_year`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEAP_YEARW {
    #[doc = "0 is not leap year"]
    NOT_LEAP,
    #[doc = "1 is leap year"]
    LEAP,
}
impl LEAP_YEARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LEAP_YEARW::NOT_LEAP => false,
            LEAP_YEARW::LEAP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LEAP_YEARW<'a> {
    w: &'a mut W,
}
impl<'a> _LEAP_YEARW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LEAP_YEARW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "0 is not leap year"]
    #[inline]
    pub fn not_leap(self) -> &'a mut W {
        self.variant(LEAP_YEARW::NOT_LEAP)
    }
    #[doc = "1 is leap year"]
    #[inline]
    pub fn leap(self) -> &'a mut W {
        self.variant(LEAP_YEARW::LEAP)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
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
    #[doc = "Bits 0:4 - Century. Range \\[0,31\\]"]
    #[inline]
    pub fn century(&self) -> CENTURYR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CENTURYR { bits }
    }
    #[doc = "Bit 5 - Is leap year. 1 is leap year, 0 is not leap year"]
    #[inline]
    pub fn leap_year(&self) -> LEAP_YEARR {
        LEAP_YEARR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:4 - Century. Range \\[0,31\\]"]
    #[inline]
    pub fn century(&mut self) -> _CENTURYW {
        _CENTURYW { w: self }
    }
    #[doc = "Bit 5 - Is leap year. 1 is leap year, 0 is not leap year"]
    #[inline]
    pub fn leap_year(&mut self) -> _LEAP_YEARW {
        _LEAP_YEARW { w: self }
    }
}
