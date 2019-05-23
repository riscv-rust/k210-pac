#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DWSZ_CFG {
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
pub struct DIR_DWN_SIZ_RATER {
    bits: u8,
}
impl DIR_DWN_SIZ_RATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VOC_DWN_SIZ_RATER {
    bits: u8,
}
impl VOC_DWN_SIZ_RATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SMPL_SHIFT_BITSR {
    bits: u8,
}
impl SMPL_SHIFT_BITSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DIR_DWN_SIZ_RATEW<'a> {
    w: &'a mut W,
}
impl<'a> _DIR_DWN_SIZ_RATEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VOC_DWN_SIZ_RATEW<'a> {
    w: &'a mut W,
}
impl<'a> _VOC_DWN_SIZ_RATEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SMPL_SHIFT_BITSW<'a> {
    w: &'a mut W,
}
impl<'a> _SMPL_SHIFT_BITSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:3 - Down-sizing ratio used for direction searching"]
    #[inline]
    pub fn dir_dwn_siz_rate(&self) -> DIR_DWN_SIZ_RATER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIR_DWN_SIZ_RATER { bits }
    }
    #[doc = "Bits 4:7 - Down-sizing ratio used for voice stream generation"]
    #[inline]
    pub fn voc_dwn_siz_rate(&self) -> VOC_DWN_SIZ_RATER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VOC_DWN_SIZ_RATER { bits }
    }
    #[doc = "Bits 8:12 - Sample precision reduction when the source sound sample precision is 20/24/32 bits"]
    #[inline]
    pub fn smpl_shift_bits(&self) -> SMPL_SHIFT_BITSR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SMPL_SHIFT_BITSR { bits }
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
    #[doc = "Bits 0:3 - Down-sizing ratio used for direction searching"]
    #[inline]
    pub fn dir_dwn_siz_rate(&mut self) -> _DIR_DWN_SIZ_RATEW {
        _DIR_DWN_SIZ_RATEW { w: self }
    }
    #[doc = "Bits 4:7 - Down-sizing ratio used for voice stream generation"]
    #[inline]
    pub fn voc_dwn_siz_rate(&mut self) -> _VOC_DWN_SIZ_RATEW {
        _VOC_DWN_SIZ_RATEW { w: self }
    }
    #[doc = "Bits 8:12 - Sample precision reduction when the source sound sample precision is 20/24/32 bits"]
    #[inline]
    pub fn smpl_shift_bits(&mut self) -> _SMPL_SHIFT_BITSW {
        _SMPL_SHIFT_BITSW { w: self }
    }
}
