#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CH_CFG {
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
pub struct SOUND_CH_ENR {
    bits: u8,
}
impl SOUND_CH_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TARGET_DIRR {
    bits: u8,
}
impl TARGET_DIRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AUDIO_GAINR {
    bits: u16,
}
impl AUDIO_GAINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATA_SRC_MODER {
    bits: bool,
}
impl DATA_SRC_MODER {
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
pub struct _SOUND_CH_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOUND_CH_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _TARGET_DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _TARGET_DIRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AUDIO_GAINW<'a> {
    w: &'a mut W,
}
impl<'a> _AUDIO_GAINW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 2047;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATA_SRC_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_SRC_MODEW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WE_SOUND_CH_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _WE_SOUND_CH_ENW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WE_TARGET_DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _WE_TARGET_DIRW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WE_AUDIO_GAINW<'a> {
    w: &'a mut W,
}
impl<'a> _WE_AUDIO_GAINW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WE_DATA_SRC_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _WE_DATA_SRC_MODEW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:7 - BF unit sound channel enable control bits"]
    #[inline]
    pub fn sound_ch_en(&self) -> SOUND_CH_ENR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SOUND_CH_ENR { bits }
    }
    #[doc = "Bits 8:11 - Target direction select for valid voice output"]
    #[inline]
    pub fn target_dir(&self) -> TARGET_DIRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TARGET_DIRR { bits }
    }
    #[doc = "Bits 12:22 - Audio sample gain factor"]
    #[inline]
    pub fn audio_gain(&self) -> AUDIO_GAINR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        AUDIO_GAINR { bits }
    }
    #[doc = "Bit 24 - Audio data source configure parameter"]
    #[inline]
    pub fn data_src_mode(&self) -> DATA_SRC_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATA_SRC_MODER { bits }
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
    #[doc = "Bits 0:7 - BF unit sound channel enable control bits"]
    #[inline]
    pub fn sound_ch_en(&mut self) -> _SOUND_CH_ENW {
        _SOUND_CH_ENW { w: self }
    }
    #[doc = "Bits 8:11 - Target direction select for valid voice output"]
    #[inline]
    pub fn target_dir(&mut self) -> _TARGET_DIRW {
        _TARGET_DIRW { w: self }
    }
    #[doc = "Bits 12:22 - Audio sample gain factor"]
    #[inline]
    pub fn audio_gain(&mut self) -> _AUDIO_GAINW {
        _AUDIO_GAINW { w: self }
    }
    #[doc = "Bit 24 - Audio data source configure parameter"]
    #[inline]
    pub fn data_src_mode(&mut self) -> _DATA_SRC_MODEW {
        _DATA_SRC_MODEW { w: self }
    }
    #[doc = "Bit 28 - Write enable for sound_ch_en parameter"]
    #[inline]
    pub fn we_sound_ch_en(&mut self) -> _WE_SOUND_CH_ENW {
        _WE_SOUND_CH_ENW { w: self }
    }
    #[doc = "Bit 29 - Write enable for target_dir parameter"]
    #[inline]
    pub fn we_target_dir(&mut self) -> _WE_TARGET_DIRW {
        _WE_TARGET_DIRW { w: self }
    }
    #[doc = "Bit 30 - Write enable for audio_gain parameter"]
    #[inline]
    pub fn we_audio_gain(&mut self) -> _WE_AUDIO_GAINW {
        _WE_AUDIO_GAINW { w: self }
    }
    #[doc = "Bit 31 - Write enable for data_out_mode parameter"]
    #[inline]
    pub fn we_data_src_mode(&mut self) -> _WE_DATA_SRC_MODEW {
        _WE_DATA_SRC_MODEW { w: self }
    }
}
