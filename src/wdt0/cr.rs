#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
pub struct ENABLER {
    bits: bool,
}
impl ENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `rmod`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMODR {
    #[doc = "RESET"]
    RESET,
    #[doc = "INTERRUPT"]
    INTERRUPT,
}
impl RMODR {
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
            RMODR::RESET => false,
            RMODR::INTERRUPT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RMODR {
        match value {
            false => RMODR::RESET,
            true => RMODR::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == RMODR::RESET
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        *self == RMODR::INTERRUPT
    }
}
#[doc = r" Value of the field"]
pub struct RPLR {
    bits: u8,
}
impl RPLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `rmod`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMODW {
    #[doc = "RESET"]
    RESET,
    #[doc = "INTERRUPT"]
    INTERRUPT,
}
impl RMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RMODW::RESET => false,
            RMODW::INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RMODW<'a> {
    w: &'a mut W,
}
impl<'a> _RMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RMODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RESET"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(RMODW::RESET)
    }
    #[doc = "INTERRUPT"]
    #[inline]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(RMODW::INTERRUPT)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RPLW<'a> {
    w: &'a mut W,
}
impl<'a> _RPLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - enable"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLER { bits }
    }
    #[doc = "Bit 1 - rmod"]
    #[inline]
    pub fn rmod(&self) -> RMODR {
        RMODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:4 - rpl"]
    #[inline]
    pub fn rpl(&self) -> RPLR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RPLR { bits }
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
    #[doc = "Bit 0 - enable"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 1 - rmod"]
    #[inline]
    pub fn rmod(&mut self) -> _RMODW {
        _RMODW { w: self }
    }
    #[doc = "Bits 2:4 - rpl"]
    #[inline]
    pub fn rpl(&mut self) -> _RPLW {
        _RPLW { w: self }
    }
}
