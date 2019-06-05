#[doc = r" Value read from the register"]
pub struct R {
    bits: u64,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u64,
}
impl super::CHEN {
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
pub struct CH1_ENR {
    bits: bool,
}
impl CH1_ENR {
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
pub struct CH2_ENR {
    bits: bool,
}
impl CH2_ENR {
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
pub struct CH3_ENR {
    bits: bool,
}
impl CH3_ENR {
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
pub struct CH4_ENR {
    bits: bool,
}
impl CH4_ENR {
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
pub struct CH5_ENR {
    bits: bool,
}
impl CH5_ENR {
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
pub struct CH6_ENR {
    bits: bool,
}
impl CH6_ENR {
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
pub struct CH1_EN_WER {
    bits: bool,
}
impl CH1_EN_WER {
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
pub struct CH2_EN_WER {
    bits: bool,
}
impl CH2_EN_WER {
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
pub struct CH3_EN_WER {
    bits: bool,
}
impl CH3_EN_WER {
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
pub struct CH4_EN_WER {
    bits: bool,
}
impl CH4_EN_WER {
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
pub struct CH5_EN_WER {
    bits: bool,
}
impl CH5_EN_WER {
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
pub struct CH6_EN_WER {
    bits: bool,
}
impl CH6_EN_WER {
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
pub struct CH1_SUSPR {
    bits: bool,
}
impl CH1_SUSPR {
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
pub struct CH2_SUSPR {
    bits: bool,
}
impl CH2_SUSPR {
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
pub struct CH3_SUSPR {
    bits: bool,
}
impl CH3_SUSPR {
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
pub struct CH4_SUSPR {
    bits: bool,
}
impl CH4_SUSPR {
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
pub struct CH5_SUSPR {
    bits: bool,
}
impl CH5_SUSPR {
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
pub struct CH6_SUSPR {
    bits: bool,
}
impl CH6_SUSPR {
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
pub struct CH1_SUSP_WER {
    bits: bool,
}
impl CH1_SUSP_WER {
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
pub struct CH2_SUSP_WER {
    bits: bool,
}
impl CH2_SUSP_WER {
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
pub struct CH3_SUSP_WER {
    bits: bool,
}
impl CH3_SUSP_WER {
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
pub struct CH4_SUSP_WER {
    bits: bool,
}
impl CH4_SUSP_WER {
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
pub struct CH5_SUSP_WER {
    bits: bool,
}
impl CH5_SUSP_WER {
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
pub struct CH6_SUSP_WER {
    bits: bool,
}
impl CH6_SUSP_WER {
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
pub struct CH1_ABORTR {
    bits: bool,
}
impl CH1_ABORTR {
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
pub struct CH2_ABORTR {
    bits: bool,
}
impl CH2_ABORTR {
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
pub struct CH3_ABORTR {
    bits: bool,
}
impl CH3_ABORTR {
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
pub struct CH4_ABORTR {
    bits: bool,
}
impl CH4_ABORTR {
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
pub struct CH5_ABORTR {
    bits: bool,
}
impl CH5_ABORTR {
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
pub struct CH6_ABORTR {
    bits: bool,
}
impl CH6_ABORTR {
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
pub struct CH1_ABORT_WER {
    bits: bool,
}
impl CH1_ABORT_WER {
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
pub struct CH2_ABORT_WER {
    bits: bool,
}
impl CH2_ABORT_WER {
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
pub struct CH3_ABORT_WER {
    bits: bool,
}
impl CH3_ABORT_WER {
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
pub struct CH4_ABORT_WER {
    bits: bool,
}
impl CH4_ABORT_WER {
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
pub struct CH5_ABORT_WER {
    bits: bool,
}
impl CH5_ABORT_WER {
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
pub struct CH6_ABORT_WER {
    bits: bool,
}
impl CH6_ABORT_WER {
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
pub struct _CH1_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1_ENW<'a> {
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
pub struct _CH2_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2_ENW<'a> {
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
pub struct _CH3_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3_ENW<'a> {
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
pub struct _CH4_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4_ENW<'a> {
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
pub struct _CH5_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5_ENW<'a> {
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
pub struct _CH6_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6_ENW<'a> {
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
pub struct _CH1_EN_WEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1_EN_WEW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH2_EN_WEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2_EN_WEW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH3_EN_WEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3_EN_WEW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH4_EN_WEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4_EN_WEW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH5_EN_WEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5_EN_WEW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH6_EN_WEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6_EN_WEW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH1_SUSPW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1_SUSPW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH2_SUSPW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2_SUSPW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH3_SUSPW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3_SUSPW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH4_SUSPW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4_SUSPW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH5_SUSPW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5_SUSPW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH6_SUSPW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6_SUSPW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH1_SUSP_WEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1_SUSP_WEW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH2_SUSP_WEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2_SUSP_WEW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH3_SUSP_WEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3_SUSP_WEW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH4_SUSP_WEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4_SUSP_WEW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH5_SUSP_WEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5_SUSP_WEW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH6_SUSP_WEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6_SUSP_WEW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH1_ABORTW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1_ABORTW<'a> {
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
        const OFFSET: u8 = 32;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH2_ABORTW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2_ABORTW<'a> {
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
        const OFFSET: u8 = 33;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH3_ABORTW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3_ABORTW<'a> {
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
        const OFFSET: u8 = 34;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH4_ABORTW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4_ABORTW<'a> {
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
        const OFFSET: u8 = 35;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH5_ABORTW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5_ABORTW<'a> {
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
        const OFFSET: u8 = 36;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH6_ABORTW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6_ABORTW<'a> {
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
        const OFFSET: u8 = 37;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH1_ABORT_WEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1_ABORT_WEW<'a> {
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
        const OFFSET: u8 = 40;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH2_ABORT_WEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2_ABORT_WEW<'a> {
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
        const OFFSET: u8 = 41;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH3_ABORT_WEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3_ABORT_WEW<'a> {
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
        const OFFSET: u8 = 42;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH4_ABORT_WEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4_ABORT_WEW<'a> {
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
        const OFFSET: u8 = 43;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH5_ABORT_WEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5_ABORT_WEW<'a> {
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
        const OFFSET: u8 = 44;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH6_ABORT_WEW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6_ABORT_WEW<'a> {
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
        const OFFSET: u8 = 45;
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
    #[doc = "Bit 0 - Enable channel 1"]
    #[inline]
    pub fn ch1_en(&self) -> CH1_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH1_ENR { bits }
    }
    #[doc = "Bit 1 - Enable channel 2"]
    #[inline]
    pub fn ch2_en(&self) -> CH2_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH2_ENR { bits }
    }
    #[doc = "Bit 2 - Enable channel 3"]
    #[inline]
    pub fn ch3_en(&self) -> CH3_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH3_ENR { bits }
    }
    #[doc = "Bit 3 - Enable channel 4"]
    #[inline]
    pub fn ch4_en(&self) -> CH4_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH4_ENR { bits }
    }
    #[doc = "Bit 4 - Enable channel 5"]
    #[inline]
    pub fn ch5_en(&self) -> CH5_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH5_ENR { bits }
    }
    #[doc = "Bit 5 - Enable channel 6"]
    #[inline]
    pub fn ch6_en(&self) -> CH6_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH6_ENR { bits }
    }
    #[doc = "Bit 8 - Write enable channel 1"]
    #[inline]
    pub fn ch1_en_we(&self) -> CH1_EN_WER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH1_EN_WER { bits }
    }
    #[doc = "Bit 9 - Write enable channel 2"]
    #[inline]
    pub fn ch2_en_we(&self) -> CH2_EN_WER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH2_EN_WER { bits }
    }
    #[doc = "Bit 10 - Write enable channel 3"]
    #[inline]
    pub fn ch3_en_we(&self) -> CH3_EN_WER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH3_EN_WER { bits }
    }
    #[doc = "Bit 11 - Write enable channel 4"]
    #[inline]
    pub fn ch4_en_we(&self) -> CH4_EN_WER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH4_EN_WER { bits }
    }
    #[doc = "Bit 12 - Write enable channel 5"]
    #[inline]
    pub fn ch5_en_we(&self) -> CH5_EN_WER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH5_EN_WER { bits }
    }
    #[doc = "Bit 13 - Write enable channel 6"]
    #[inline]
    pub fn ch6_en_we(&self) -> CH6_EN_WER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH6_EN_WER { bits }
    }
    #[doc = "Bit 16 - Suspend request channel 1"]
    #[inline]
    pub fn ch1_susp(&self) -> CH1_SUSPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH1_SUSPR { bits }
    }
    #[doc = "Bit 17 - Suspend request channel 2"]
    #[inline]
    pub fn ch2_susp(&self) -> CH2_SUSPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH2_SUSPR { bits }
    }
    #[doc = "Bit 18 - Suspend request channel 3"]
    #[inline]
    pub fn ch3_susp(&self) -> CH3_SUSPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH3_SUSPR { bits }
    }
    #[doc = "Bit 19 - Suspend request channel 4"]
    #[inline]
    pub fn ch4_susp(&self) -> CH4_SUSPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH4_SUSPR { bits }
    }
    #[doc = "Bit 20 - Suspend request channel 5"]
    #[inline]
    pub fn ch5_susp(&self) -> CH5_SUSPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH5_SUSPR { bits }
    }
    #[doc = "Bit 21 - Suspend request channel 6"]
    #[inline]
    pub fn ch6_susp(&self) -> CH6_SUSPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH6_SUSPR { bits }
    }
    #[doc = "Bit 24 - Enable write to ch1_susp bit"]
    #[inline]
    pub fn ch1_susp_we(&self) -> CH1_SUSP_WER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH1_SUSP_WER { bits }
    }
    #[doc = "Bit 25 - Enable write to ch2_susp bit"]
    #[inline]
    pub fn ch2_susp_we(&self) -> CH2_SUSP_WER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH2_SUSP_WER { bits }
    }
    #[doc = "Bit 26 - Enable write to ch3_susp bit"]
    #[inline]
    pub fn ch3_susp_we(&self) -> CH3_SUSP_WER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH3_SUSP_WER { bits }
    }
    #[doc = "Bit 27 - Enable write to ch4_susp bit"]
    #[inline]
    pub fn ch4_susp_we(&self) -> CH4_SUSP_WER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH4_SUSP_WER { bits }
    }
    #[doc = "Bit 28 - Enable write to ch5_susp bit"]
    #[inline]
    pub fn ch5_susp_we(&self) -> CH5_SUSP_WER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH5_SUSP_WER { bits }
    }
    #[doc = "Bit 29 - Enable write to ch6_susp bit"]
    #[inline]
    pub fn ch6_susp_we(&self) -> CH6_SUSP_WER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH6_SUSP_WER { bits }
    }
    #[doc = "Bit 32 - Abort request channel 1"]
    #[inline]
    pub fn ch1_abort(&self) -> CH1_ABORTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 32;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH1_ABORTR { bits }
    }
    #[doc = "Bit 33 - Abort request channel 2"]
    #[inline]
    pub fn ch2_abort(&self) -> CH2_ABORTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 33;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH2_ABORTR { bits }
    }
    #[doc = "Bit 34 - Abort request channel 3"]
    #[inline]
    pub fn ch3_abort(&self) -> CH3_ABORTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 34;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH3_ABORTR { bits }
    }
    #[doc = "Bit 35 - Abort request channel 4"]
    #[inline]
    pub fn ch4_abort(&self) -> CH4_ABORTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 35;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH4_ABORTR { bits }
    }
    #[doc = "Bit 36 - Abort request channel 5"]
    #[inline]
    pub fn ch5_abort(&self) -> CH5_ABORTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 36;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH5_ABORTR { bits }
    }
    #[doc = "Bit 37 - Abort request channel 6"]
    #[inline]
    pub fn ch6_abort(&self) -> CH6_ABORTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 37;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH6_ABORTR { bits }
    }
    #[doc = "Bit 40 - Enable write to ch1_abort bit"]
    #[inline]
    pub fn ch1_abort_we(&self) -> CH1_ABORT_WER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 40;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH1_ABORT_WER { bits }
    }
    #[doc = "Bit 41 - Enable write to ch2_abort bit"]
    #[inline]
    pub fn ch2_abort_we(&self) -> CH2_ABORT_WER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 41;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH2_ABORT_WER { bits }
    }
    #[doc = "Bit 42 - Enable write to ch3_abort bit"]
    #[inline]
    pub fn ch3_abort_we(&self) -> CH3_ABORT_WER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 42;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH3_ABORT_WER { bits }
    }
    #[doc = "Bit 43 - Enable write to ch4_abort bit"]
    #[inline]
    pub fn ch4_abort_we(&self) -> CH4_ABORT_WER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 43;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH4_ABORT_WER { bits }
    }
    #[doc = "Bit 44 - Enable write to ch5_abort bit"]
    #[inline]
    pub fn ch5_abort_we(&self) -> CH5_ABORT_WER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 44;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH5_ABORT_WER { bits }
    }
    #[doc = "Bit 45 - Enable write to ch6_abort bit"]
    #[inline]
    pub fn ch6_abort_we(&self) -> CH6_ABORT_WER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 45;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CH6_ABORT_WER { bits }
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
    #[doc = "Bit 0 - Enable channel 1"]
    #[inline]
    pub fn ch1_en(&mut self) -> _CH1_ENW {
        _CH1_ENW { w: self }
    }
    #[doc = "Bit 1 - Enable channel 2"]
    #[inline]
    pub fn ch2_en(&mut self) -> _CH2_ENW {
        _CH2_ENW { w: self }
    }
    #[doc = "Bit 2 - Enable channel 3"]
    #[inline]
    pub fn ch3_en(&mut self) -> _CH3_ENW {
        _CH3_ENW { w: self }
    }
    #[doc = "Bit 3 - Enable channel 4"]
    #[inline]
    pub fn ch4_en(&mut self) -> _CH4_ENW {
        _CH4_ENW { w: self }
    }
    #[doc = "Bit 4 - Enable channel 5"]
    #[inline]
    pub fn ch5_en(&mut self) -> _CH5_ENW {
        _CH5_ENW { w: self }
    }
    #[doc = "Bit 5 - Enable channel 6"]
    #[inline]
    pub fn ch6_en(&mut self) -> _CH6_ENW {
        _CH6_ENW { w: self }
    }
    #[doc = "Bit 8 - Write enable channel 1"]
    #[inline]
    pub fn ch1_en_we(&mut self) -> _CH1_EN_WEW {
        _CH1_EN_WEW { w: self }
    }
    #[doc = "Bit 9 - Write enable channel 2"]
    #[inline]
    pub fn ch2_en_we(&mut self) -> _CH2_EN_WEW {
        _CH2_EN_WEW { w: self }
    }
    #[doc = "Bit 10 - Write enable channel 3"]
    #[inline]
    pub fn ch3_en_we(&mut self) -> _CH3_EN_WEW {
        _CH3_EN_WEW { w: self }
    }
    #[doc = "Bit 11 - Write enable channel 4"]
    #[inline]
    pub fn ch4_en_we(&mut self) -> _CH4_EN_WEW {
        _CH4_EN_WEW { w: self }
    }
    #[doc = "Bit 12 - Write enable channel 5"]
    #[inline]
    pub fn ch5_en_we(&mut self) -> _CH5_EN_WEW {
        _CH5_EN_WEW { w: self }
    }
    #[doc = "Bit 13 - Write enable channel 6"]
    #[inline]
    pub fn ch6_en_we(&mut self) -> _CH6_EN_WEW {
        _CH6_EN_WEW { w: self }
    }
    #[doc = "Bit 16 - Suspend request channel 1"]
    #[inline]
    pub fn ch1_susp(&mut self) -> _CH1_SUSPW {
        _CH1_SUSPW { w: self }
    }
    #[doc = "Bit 17 - Suspend request channel 2"]
    #[inline]
    pub fn ch2_susp(&mut self) -> _CH2_SUSPW {
        _CH2_SUSPW { w: self }
    }
    #[doc = "Bit 18 - Suspend request channel 3"]
    #[inline]
    pub fn ch3_susp(&mut self) -> _CH3_SUSPW {
        _CH3_SUSPW { w: self }
    }
    #[doc = "Bit 19 - Suspend request channel 4"]
    #[inline]
    pub fn ch4_susp(&mut self) -> _CH4_SUSPW {
        _CH4_SUSPW { w: self }
    }
    #[doc = "Bit 20 - Suspend request channel 5"]
    #[inline]
    pub fn ch5_susp(&mut self) -> _CH5_SUSPW {
        _CH5_SUSPW { w: self }
    }
    #[doc = "Bit 21 - Suspend request channel 6"]
    #[inline]
    pub fn ch6_susp(&mut self) -> _CH6_SUSPW {
        _CH6_SUSPW { w: self }
    }
    #[doc = "Bit 24 - Enable write to ch1_susp bit"]
    #[inline]
    pub fn ch1_susp_we(&mut self) -> _CH1_SUSP_WEW {
        _CH1_SUSP_WEW { w: self }
    }
    #[doc = "Bit 25 - Enable write to ch2_susp bit"]
    #[inline]
    pub fn ch2_susp_we(&mut self) -> _CH2_SUSP_WEW {
        _CH2_SUSP_WEW { w: self }
    }
    #[doc = "Bit 26 - Enable write to ch3_susp bit"]
    #[inline]
    pub fn ch3_susp_we(&mut self) -> _CH3_SUSP_WEW {
        _CH3_SUSP_WEW { w: self }
    }
    #[doc = "Bit 27 - Enable write to ch4_susp bit"]
    #[inline]
    pub fn ch4_susp_we(&mut self) -> _CH4_SUSP_WEW {
        _CH4_SUSP_WEW { w: self }
    }
    #[doc = "Bit 28 - Enable write to ch5_susp bit"]
    #[inline]
    pub fn ch5_susp_we(&mut self) -> _CH5_SUSP_WEW {
        _CH5_SUSP_WEW { w: self }
    }
    #[doc = "Bit 29 - Enable write to ch6_susp bit"]
    #[inline]
    pub fn ch6_susp_we(&mut self) -> _CH6_SUSP_WEW {
        _CH6_SUSP_WEW { w: self }
    }
    #[doc = "Bit 32 - Abort request channel 1"]
    #[inline]
    pub fn ch1_abort(&mut self) -> _CH1_ABORTW {
        _CH1_ABORTW { w: self }
    }
    #[doc = "Bit 33 - Abort request channel 2"]
    #[inline]
    pub fn ch2_abort(&mut self) -> _CH2_ABORTW {
        _CH2_ABORTW { w: self }
    }
    #[doc = "Bit 34 - Abort request channel 3"]
    #[inline]
    pub fn ch3_abort(&mut self) -> _CH3_ABORTW {
        _CH3_ABORTW { w: self }
    }
    #[doc = "Bit 35 - Abort request channel 4"]
    #[inline]
    pub fn ch4_abort(&mut self) -> _CH4_ABORTW {
        _CH4_ABORTW { w: self }
    }
    #[doc = "Bit 36 - Abort request channel 5"]
    #[inline]
    pub fn ch5_abort(&mut self) -> _CH5_ABORTW {
        _CH5_ABORTW { w: self }
    }
    #[doc = "Bit 37 - Abort request channel 6"]
    #[inline]
    pub fn ch6_abort(&mut self) -> _CH6_ABORTW {
        _CH6_ABORTW { w: self }
    }
    #[doc = "Bit 40 - Enable write to ch1_abort bit"]
    #[inline]
    pub fn ch1_abort_we(&mut self) -> _CH1_ABORT_WEW {
        _CH1_ABORT_WEW { w: self }
    }
    #[doc = "Bit 41 - Enable write to ch2_abort bit"]
    #[inline]
    pub fn ch2_abort_we(&mut self) -> _CH2_ABORT_WEW {
        _CH2_ABORT_WEW { w: self }
    }
    #[doc = "Bit 42 - Enable write to ch3_abort bit"]
    #[inline]
    pub fn ch3_abort_we(&mut self) -> _CH3_ABORT_WEW {
        _CH3_ABORT_WEW { w: self }
    }
    #[doc = "Bit 43 - Enable write to ch4_abort bit"]
    #[inline]
    pub fn ch4_abort_we(&mut self) -> _CH4_ABORT_WEW {
        _CH4_ABORT_WEW { w: self }
    }
    #[doc = "Bit 44 - Enable write to ch5_abort bit"]
    #[inline]
    pub fn ch5_abort_we(&mut self) -> _CH5_ABORT_WEW {
        _CH5_ABORT_WEW { w: self }
    }
    #[doc = "Bit 45 - Enable write to ch6_abort bit"]
    #[inline]
    pub fn ch6_abort_we(&mut self) -> _CH6_ABORT_WEW {
        _CH6_ABORT_WEW { w: self }
    }
}
