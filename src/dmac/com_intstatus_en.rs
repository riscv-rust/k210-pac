#[doc = r" Value read from the register"]
pub struct R {
    bits: u64,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u64,
}
impl super::COM_INTSTATUS_EN {
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
pub struct SLVIF_DEC_ERRR {
    bits: bool,
}
impl SLVIF_DEC_ERRR {
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
pub struct SLVIF_WR2RO_ERRR {
    bits: bool,
}
impl SLVIF_WR2RO_ERRR {
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
pub struct SLVIF_RD2WO_ERRR {
    bits: bool,
}
impl SLVIF_RD2WO_ERRR {
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
pub struct SLVIF_WRONHOLD_ERRR {
    bits: bool,
}
impl SLVIF_WRONHOLD_ERRR {
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
pub struct SLVIF_UNDEFINEDREG_DEC_ERRR {
    bits: bool,
}
impl SLVIF_UNDEFINEDREG_DEC_ERRR {
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
pub struct _SLVIF_DEC_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVIF_DEC_ERRW<'a> {
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
pub struct _SLVIF_WR2RO_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVIF_WR2RO_ERRW<'a> {
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
pub struct _SLVIF_RD2WO_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVIF_RD2WO_ERRW<'a> {
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
pub struct _SLVIF_WRONHOLD_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVIF_WRONHOLD_ERRW<'a> {
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
pub struct _SLVIF_UNDEFINEDREG_DEC_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVIF_UNDEFINEDREG_DEC_ERRW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u64 {
        self.bits
    }
    #[doc = "Bit 0 - Slave Interface Common Register Decode Error"]
    #[inline]
    pub fn slvif_dec_err(&self) -> SLVIF_DEC_ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        SLVIF_DEC_ERRR { bits }
    }
    #[doc = "Bit 1 - Slave Interface Common Register Write to Read only Error"]
    #[inline]
    pub fn slvif_wr2ro_err(&self) -> SLVIF_WR2RO_ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        SLVIF_WR2RO_ERRR { bits }
    }
    #[doc = "Bit 2 - Slave Interface Common Register Read to Write-only Error"]
    #[inline]
    pub fn slvif_rd2wo_err(&self) -> SLVIF_RD2WO_ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        SLVIF_RD2WO_ERRR { bits }
    }
    #[doc = "Bit 3 - Slave Interface Common Register Write On Hold Error"]
    #[inline]
    pub fn slvif_wronhold_err(&self) -> SLVIF_WRONHOLD_ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        SLVIF_WRONHOLD_ERRR { bits }
    }
    #[doc = "Bit 8 - Slave Interface Undefined Register Decode Error"]
    #[inline]
    pub fn slvif_undefinedreg_dec_err(&self) -> SLVIF_UNDEFINEDREG_DEC_ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        SLVIF_UNDEFINEDREG_DEC_ERRR { bits }
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
    #[doc = "Bit 0 - Slave Interface Common Register Decode Error"]
    #[inline]
    pub fn slvif_dec_err(&mut self) -> _SLVIF_DEC_ERRW {
        _SLVIF_DEC_ERRW { w: self }
    }
    #[doc = "Bit 1 - Slave Interface Common Register Write to Read only Error"]
    #[inline]
    pub fn slvif_wr2ro_err(&mut self) -> _SLVIF_WR2RO_ERRW {
        _SLVIF_WR2RO_ERRW { w: self }
    }
    #[doc = "Bit 2 - Slave Interface Common Register Read to Write-only Error"]
    #[inline]
    pub fn slvif_rd2wo_err(&mut self) -> _SLVIF_RD2WO_ERRW {
        _SLVIF_RD2WO_ERRW { w: self }
    }
    #[doc = "Bit 3 - Slave Interface Common Register Write On Hold Error"]
    #[inline]
    pub fn slvif_wronhold_err(&mut self) -> _SLVIF_WRONHOLD_ERRW {
        _SLVIF_WRONHOLD_ERRW { w: self }
    }
    #[doc = "Bit 8 - Slave Interface Undefined Register Decode Error"]
    #[inline]
    pub fn slvif_undefinedreg_dec_err(&mut self) -> _SLVIF_UNDEFINEDREG_DEC_ERRW {
        _SLVIF_UNDEFINEDREG_DEC_ERRW { w: self }
    }
}
