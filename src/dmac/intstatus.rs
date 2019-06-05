#[doc = r" Value read from the register"]
pub struct R {
    bits: u64,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u64,
}
impl super::INTSTATUS {
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
pub struct CH1_INTSTATR {
    bits: bool,
}
impl CH1_INTSTATR {
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
pub struct CH2_INTSTATR {
    bits: bool,
}
impl CH2_INTSTATR {
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
pub struct CH3_INTSTATR {
    bits: bool,
}
impl CH3_INTSTATR {
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
pub struct CH4_INTSTATR {
    bits: bool,
}
impl CH4_INTSTATR {
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
pub struct CH5_INTSTATR {
    bits: bool,
}
impl CH5_INTSTATR {
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
pub struct CH6_INTSTATR {
    bits: bool,
}
impl CH6_INTSTATR {
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
pub struct COMMONREG_INTSTATR {
    bits: bool,
}
impl COMMONREG_INTSTATR {
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
pub struct _CH1_INTSTATW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1_INTSTATW<'a> {
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
pub struct _CH2_INTSTATW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2_INTSTATW<'a> {
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
pub struct _CH3_INTSTATW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3_INTSTATW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH4_INTSTATW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4_INTSTATW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH5_INTSTATW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5_INTSTATW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH6_INTSTATW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6_INTSTATW<'a> {
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
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COMMONREG_INTSTATW<'a> {
    w: &'a mut W,
}
impl<'a> _COMMONREG_INTSTATW<'a> {
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
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Channel 1 interrupt bit"]
    #[inline]
    pub fn ch1_intstat(&self) -> CH1_INTSTATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH1_INTSTATR { bits }
    }
    #[doc = "Bit 1 - Channel 2 interrupt bit"]
    #[inline]
    pub fn ch2_intstat(&self) -> CH2_INTSTATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH2_INTSTATR { bits }
    }
    #[doc = "Bit 2 - Channel 3 interrupt bit"]
    #[inline]
    pub fn ch3_intstat(&self) -> CH3_INTSTATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH3_INTSTATR { bits }
    }
    #[doc = "Bit 3 - Channel 4 interrupt bit"]
    #[inline]
    pub fn ch4_intstat(&self) -> CH4_INTSTATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH4_INTSTATR { bits }
    }
    #[doc = "Bit 4 - Channel 5 interrupt bit"]
    #[inline]
    pub fn ch5_intstat(&self) -> CH5_INTSTATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH5_INTSTATR { bits }
    }
    #[doc = "Bit 5 - Channel 6 interrupt bit"]
    #[inline]
    pub fn ch6_intstat(&self) -> CH6_INTSTATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH6_INTSTATR { bits }
    }
    #[doc = "Bit 16 - Common register status bit"]
    #[inline]
    pub fn commonreg_intstat(&self) -> COMMONREG_INTSTATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        COMMONREG_INTSTATR { bits }
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
    #[doc = "Bit 0 - Channel 1 interrupt bit"]
    #[inline]
    pub fn ch1_intstat(&mut self) -> _CH1_INTSTATW {
        _CH1_INTSTATW { w: self }
    }
    #[doc = "Bit 1 - Channel 2 interrupt bit"]
    #[inline]
    pub fn ch2_intstat(&mut self) -> _CH2_INTSTATW {
        _CH2_INTSTATW { w: self }
    }
    #[doc = "Bit 2 - Channel 3 interrupt bit"]
    #[inline]
    pub fn ch3_intstat(&mut self) -> _CH3_INTSTATW {
        _CH3_INTSTATW { w: self }
    }
    #[doc = "Bit 3 - Channel 4 interrupt bit"]
    #[inline]
    pub fn ch4_intstat(&mut self) -> _CH4_INTSTATW {
        _CH4_INTSTATW { w: self }
    }
    #[doc = "Bit 4 - Channel 5 interrupt bit"]
    #[inline]
    pub fn ch5_intstat(&mut self) -> _CH5_INTSTATW {
        _CH5_INTSTATW { w: self }
    }
    #[doc = "Bit 5 - Channel 6 interrupt bit"]
    #[inline]
    pub fn ch6_intstat(&mut self) -> _CH6_INTSTATW {
        _CH6_INTSTATW { w: self }
    }
    #[doc = "Bit 16 - Common register status bit"]
    #[inline]
    pub fn commonreg_intstat(&mut self) -> _COMMONREG_INTSTATW {
        _COMMONREG_INTSTATW { w: self }
    }
}
