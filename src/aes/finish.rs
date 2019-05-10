#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FINISH {
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
#[doc = "Possible values of the field `finish`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FINISHR {
    #[doc = "Operation not finished"]
    NOT_FINISHED,
    #[doc = "Operation finished"]
    FINISHED,
}
impl FINISHR {
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
            FINISHR::NOT_FINISHED => false,
            FINISHR::FINISHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FINISHR {
        match value {
            false => FINISHR::NOT_FINISHED,
            true => FINISHR::FINISHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FINISHED`"]
    #[inline]
    pub fn is_not_finished(&self) -> bool {
        *self == FINISHR::NOT_FINISHED
    }
    #[doc = "Checks if the value of the field is `FINISHED`"]
    #[inline]
    pub fn is_finished(&self) -> bool {
        *self == FINISHR::FINISHED
    }
}
#[doc = "Values that can be written to the field `finish`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FINISHW {
    #[doc = "Operation not finished"]
    NOT_FINISHED,
    #[doc = "Operation finished"]
    FINISHED,
}
impl FINISHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FINISHW::NOT_FINISHED => false,
            FINISHW::FINISHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FINISHW<'a> {
    w: &'a mut W,
}
impl<'a> _FINISHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FINISHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Operation not finished"]
    #[inline]
    pub fn not_finished(self) -> &'a mut W {
        self.variant(FINISHW::NOT_FINISHED)
    }
    #[doc = "Operation finished"]
    #[inline]
    pub fn finished(self) -> &'a mut W {
        self.variant(FINISHW::FINISHED)
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
    #[doc = "Bit 0 - AES operation finished status"]
    #[inline]
    pub fn finish(&self) -> FINISHR {
        FINISHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 0 - AES operation finished status"]
    #[inline]
    pub fn finish(&mut self) -> _FINISHW {
        _FINISHW { w: self }
    }
}
