#[doc = r" Value read from the register"]
pub struct R {
    bits: u64,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u64,
}
impl super::FIFO_CTRL {
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
pub struct DMA_FIFO_FLUSH_NR {
    bits: bool,
}
impl DMA_FIFO_FLUSH_NR {
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
pub struct GS_FIFO_FLUSH_NR {
    bits: bool,
}
impl GS_FIFO_FLUSH_NR {
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
pub struct CFG_FIFO_FLUSH_NR {
    bits: bool,
}
impl CFG_FIFO_FLUSH_NR {
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
pub struct CMD_FIFO_FLUSH_NR {
    bits: bool,
}
impl CMD_FIFO_FLUSH_NR {
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
pub struct RESP_FIFO_FLUSH_NR {
    bits: bool,
}
impl RESP_FIFO_FLUSH_NR {
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
pub struct _DMA_FIFO_FLUSH_NW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_FIFO_FLUSH_NW<'a> {
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
pub struct _GS_FIFO_FLUSH_NW<'a> {
    w: &'a mut W,
}
impl<'a> _GS_FIFO_FLUSH_NW<'a> {
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
pub struct _CFG_FIFO_FLUSH_NW<'a> {
    w: &'a mut W,
}
impl<'a> _CFG_FIFO_FLUSH_NW<'a> {
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
pub struct _CMD_FIFO_FLUSH_NW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_FIFO_FLUSH_NW<'a> {
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
pub struct _RESP_FIFO_FLUSH_NW<'a> {
    w: &'a mut W,
}
impl<'a> _RESP_FIFO_FLUSH_NW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u64 {
        self.bits
    }
    #[doc = "Bit 0 - Flush DMA FIFO"]
    #[inline]
    pub fn dma_fifo_flush_n(&self) -> DMA_FIFO_FLUSH_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        DMA_FIFO_FLUSH_NR { bits }
    }
    #[doc = "Bit 1 - Flush GS FIFO"]
    #[inline]
    pub fn gs_fifo_flush_n(&self) -> GS_FIFO_FLUSH_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        GS_FIFO_FLUSH_NR { bits }
    }
    #[doc = "Bit 2 - Flush configuration FIFO"]
    #[inline]
    pub fn cfg_fifo_flush_n(&self) -> CFG_FIFO_FLUSH_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CFG_FIFO_FLUSH_NR { bits }
    }
    #[doc = "Bit 3 - Flush command FIFO"]
    #[inline]
    pub fn cmd_fifo_flush_n(&self) -> CMD_FIFO_FLUSH_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        CMD_FIFO_FLUSH_NR { bits }
    }
    #[doc = "Bit 4 - Flush response FIFO"]
    #[inline]
    pub fn resp_fifo_flush_n(&self) -> RESP_FIFO_FLUSH_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        RESP_FIFO_FLUSH_NR { bits }
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
    #[doc = "Bit 0 - Flush DMA FIFO"]
    #[inline]
    pub fn dma_fifo_flush_n(&mut self) -> _DMA_FIFO_FLUSH_NW {
        _DMA_FIFO_FLUSH_NW { w: self }
    }
    #[doc = "Bit 1 - Flush GS FIFO"]
    #[inline]
    pub fn gs_fifo_flush_n(&mut self) -> _GS_FIFO_FLUSH_NW {
        _GS_FIFO_FLUSH_NW { w: self }
    }
    #[doc = "Bit 2 - Flush configuration FIFO"]
    #[inline]
    pub fn cfg_fifo_flush_n(&mut self) -> _CFG_FIFO_FLUSH_NW {
        _CFG_FIFO_FLUSH_NW { w: self }
    }
    #[doc = "Bit 3 - Flush command FIFO"]
    #[inline]
    pub fn cmd_fifo_flush_n(&mut self) -> _CMD_FIFO_FLUSH_NW {
        _CMD_FIFO_FLUSH_NW { w: self }
    }
    #[doc = "Bit 4 - Flush response FIFO"]
    #[inline]
    pub fn resp_fifo_flush_n(&mut self) -> _RESP_FIFO_FLUSH_NW {
        _RESP_FIFO_FLUSH_NW { w: self }
    }
}
