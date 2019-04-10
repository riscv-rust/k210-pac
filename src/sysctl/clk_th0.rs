#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLK_TH0 {
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
pub struct SRAM0_GCLKR {
    bits: u8,
}
impl SRAM0_GCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SRAM1_GCLKR {
    bits: u8,
}
impl SRAM1_GCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AI_GCLKR {
    bits: u8,
}
impl AI_GCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DVP_GCLKR {
    bits: u8,
}
impl DVP_GCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ROM_GCLKR {
    bits: u8,
}
impl ROM_GCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SRAM0_GCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAM0_GCLKW<'a> {
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
pub struct _SRAM1_GCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAM1_GCLKW<'a> {
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
pub struct _AI_GCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _AI_GCLKW<'a> {
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
pub struct _DVP_GCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _DVP_GCLKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ROM_GCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _ROM_GCLKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:3"]
    #[inline]
    pub fn sram0_gclk(&self) -> SRAM0_GCLKR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SRAM0_GCLKR { bits }
    }
    #[doc = "Bits 4:7"]
    #[inline]
    pub fn sram1_gclk(&self) -> SRAM1_GCLKR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SRAM1_GCLKR { bits }
    }
    #[doc = "Bits 8:11"]
    #[inline]
    pub fn ai_gclk(&self) -> AI_GCLKR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AI_GCLKR { bits }
    }
    #[doc = "Bits 12:15"]
    #[inline]
    pub fn dvp_gclk(&self) -> DVP_GCLKR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DVP_GCLKR { bits }
    }
    #[doc = "Bits 16:19"]
    #[inline]
    pub fn rom_gclk(&self) -> ROM_GCLKR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ROM_GCLKR { bits }
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
    #[doc = "Bits 0:3"]
    #[inline]
    pub fn sram0_gclk(&mut self) -> _SRAM0_GCLKW {
        _SRAM0_GCLKW { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline]
    pub fn sram1_gclk(&mut self) -> _SRAM1_GCLKW {
        _SRAM1_GCLKW { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline]
    pub fn ai_gclk(&mut self) -> _AI_GCLKW {
        _AI_GCLKW { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline]
    pub fn dvp_gclk(&mut self) -> _DVP_GCLKW {
        _DVP_GCLKW { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline]
    pub fn rom_gclk(&mut self) -> _ROM_GCLKW {
        _ROM_GCLKW { w: self }
    }
}
