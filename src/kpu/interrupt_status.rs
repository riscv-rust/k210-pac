#[doc = r" Value read from the register"]
pub struct R {
    bits: u64,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u64,
}
impl super::INTERRUPT_STATUS {
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
pub struct CALC_DONER {
    bits: bool,
}
impl CALC_DONER {
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
#[doc = r" Value of the field"]
pub struct LAYER_CFG_ALMOST_EMPTYR {
    bits: bool,
}
impl LAYER_CFG_ALMOST_EMPTYR {
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
#[doc = r" Value of the field"]
pub struct LAYER_CFG_ALMOST_FULLR {
    bits: bool,
}
impl LAYER_CFG_ALMOST_FULLR {
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
#[doc = r" Proxy"]
pub struct _CALC_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _CALC_DONEW<'a> {
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
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LAYER_CFG_ALMOST_EMPTYW<'a> {
    w: &'a mut W,
}
impl<'a> _LAYER_CFG_ALMOST_EMPTYW<'a> {
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
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LAYER_CFG_ALMOST_FULLW<'a> {
    w: &'a mut W,
}
impl<'a> _LAYER_CFG_ALMOST_FULLW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u64 {
        self.bits
    }
    #[doc = "Bit 0 - Interrupt raised when calculation is done"]
    #[inline]
    pub fn calc_done(&self) -> CALC_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CALC_DONER { bits }
    }
    #[doc = "Bit 1 - Interrupt raised when layer arguments FIFO almost empty"]
    #[inline]
    pub fn layer_cfg_almost_empty(&self) -> LAYER_CFG_ALMOST_EMPTYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        LAYER_CFG_ALMOST_EMPTYR { bits }
    }
    #[doc = "Bit 2 - Interrupt raised when layer arguments FIFO almost full"]
    #[inline]
    pub fn layer_cfg_almost_full(&self) -> LAYER_CFG_ALMOST_FULLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        LAYER_CFG_ALMOST_FULLR { bits }
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
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Interrupt raised when calculation is done"]
    #[inline]
    pub fn calc_done(&mut self) -> _CALC_DONEW {
        _CALC_DONEW { w: self }
    }
    #[doc = "Bit 1 - Interrupt raised when layer arguments FIFO almost empty"]
    #[inline]
    pub fn layer_cfg_almost_empty(&mut self) -> _LAYER_CFG_ALMOST_EMPTYW {
        _LAYER_CFG_ALMOST_EMPTYW { w: self }
    }
    #[doc = "Bit 2 - Interrupt raised when layer arguments FIFO almost full"]
    #[inline]
    pub fn layer_cfg_almost_full(&mut self) -> _LAYER_CFG_ALMOST_FULLW {
        _LAYER_CFG_ALMOST_FULLW { w: self }
    }
}
