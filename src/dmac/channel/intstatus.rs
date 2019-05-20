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
pub struct BLOCK_TFR_DONER {
    bits: bool,
}
impl BLOCK_TFR_DONER {
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
pub struct TFR_DONER {
    bits: bool,
}
impl TFR_DONER {
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
pub struct SRC_TRANSCOMPR {
    bits: bool,
}
impl SRC_TRANSCOMPR {
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
pub struct DST_TRANSCOMPR {
    bits: bool,
}
impl DST_TRANSCOMPR {
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
pub struct SRC_DEC_ERRR {
    bits: bool,
}
impl SRC_DEC_ERRR {
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
pub struct DST_DEC_ERRR {
    bits: bool,
}
impl DST_DEC_ERRR {
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
pub struct SRC_SLV_ERRR {
    bits: bool,
}
impl SRC_SLV_ERRR {
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
pub struct DST_SLV_ERRR {
    bits: bool,
}
impl DST_SLV_ERRR {
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
pub struct LLI_RD_DEC_ERRR {
    bits: bool,
}
impl LLI_RD_DEC_ERRR {
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
pub struct LLI_WR_DEC_ERRR {
    bits: bool,
}
impl LLI_WR_DEC_ERRR {
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
pub struct LLI_RD_SLV_ERRR {
    bits: bool,
}
impl LLI_RD_SLV_ERRR {
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
pub struct LLI_WR_SLV_ERRR {
    bits: bool,
}
impl LLI_WR_SLV_ERRR {
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
pub struct _BLOCK_TFR_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _BLOCK_TFR_DONEW<'a> {
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
pub struct _TFR_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _TFR_DONEW<'a> {
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
pub struct _SRC_TRANSCOMPW<'a> {
    w: &'a mut W,
}
impl<'a> _SRC_TRANSCOMPW<'a> {
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
pub struct _DST_TRANSCOMPW<'a> {
    w: &'a mut W,
}
impl<'a> _DST_TRANSCOMPW<'a> {
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
pub struct _SRC_DEC_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _SRC_DEC_ERRW<'a> {
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
pub struct _DST_DEC_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _DST_DEC_ERRW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SRC_SLV_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _SRC_SLV_ERRW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DST_SLV_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _DST_SLV_ERRW<'a> {
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
pub struct _LLI_RD_DEC_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _LLI_RD_DEC_ERRW<'a> {
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
pub struct _LLI_WR_DEC_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _LLI_WR_DEC_ERRW<'a> {
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
pub struct _LLI_RD_SLV_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _LLI_RD_SLV_ERRW<'a> {
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
pub struct _LLI_WR_SLV_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _LLI_WR_SLV_ERRW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u64 {
        self.bits
    }
    #[doc = "Bit 0 - Block transfer done"]
    #[inline]
    pub fn block_tfr_done(&self) -> BLOCK_TFR_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        BLOCK_TFR_DONER { bits }
    }
    #[doc = "Bit 1 - Transfer done"]
    #[inline]
    pub fn tfr_done(&self) -> TFR_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        TFR_DONER { bits }
    }
    #[doc = "Bit 3 - Source transaction complete"]
    #[inline]
    pub fn src_transcomp(&self) -> SRC_TRANSCOMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        SRC_TRANSCOMPR { bits }
    }
    #[doc = "Bit 4 - Destination transaction complete"]
    #[inline]
    pub fn dst_transcomp(&self) -> DST_TRANSCOMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        DST_TRANSCOMPR { bits }
    }
    #[doc = "Bit 5 - Source Decode Error"]
    #[inline]
    pub fn src_dec_err(&self) -> SRC_DEC_ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        SRC_DEC_ERRR { bits }
    }
    #[doc = "Bit 6 - Destination Decode Error"]
    #[inline]
    pub fn dst_dec_err(&self) -> DST_DEC_ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        DST_DEC_ERRR { bits }
    }
    #[doc = "Bit 7 - Source Slave Error"]
    #[inline]
    pub fn src_slv_err(&self) -> SRC_SLV_ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        SRC_SLV_ERRR { bits }
    }
    #[doc = "Bit 8 - Destination Slave Error"]
    #[inline]
    pub fn dst_slv_err(&self) -> DST_SLV_ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        DST_SLV_ERRR { bits }
    }
    #[doc = "Bit 9 - LLI Read Decode Error Status Enable"]
    #[inline]
    pub fn lli_rd_dec_err(&self) -> LLI_RD_DEC_ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        LLI_RD_DEC_ERRR { bits }
    }
    #[doc = "Bit 10 - LLI WRITE Decode Error"]
    #[inline]
    pub fn lli_wr_dec_err(&self) -> LLI_WR_DEC_ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        LLI_WR_DEC_ERRR { bits }
    }
    #[doc = "Bit 11 - LLI Read Slave Error"]
    #[inline]
    pub fn lli_rd_slv_err(&self) -> LLI_RD_SLV_ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        LLI_RD_SLV_ERRR { bits }
    }
    #[doc = "Bit 12 - LLI WRITE Slave Error"]
    #[inline]
    pub fn lli_wr_slv_err(&self) -> LLI_WR_SLV_ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        LLI_WR_SLV_ERRR { bits }
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
    #[doc = "Bit 0 - Block transfer done"]
    #[inline]
    pub fn block_tfr_done(&mut self) -> _BLOCK_TFR_DONEW {
        _BLOCK_TFR_DONEW { w: self }
    }
    #[doc = "Bit 1 - Transfer done"]
    #[inline]
    pub fn tfr_done(&mut self) -> _TFR_DONEW {
        _TFR_DONEW { w: self }
    }
    #[doc = "Bit 3 - Source transaction complete"]
    #[inline]
    pub fn src_transcomp(&mut self) -> _SRC_TRANSCOMPW {
        _SRC_TRANSCOMPW { w: self }
    }
    #[doc = "Bit 4 - Destination transaction complete"]
    #[inline]
    pub fn dst_transcomp(&mut self) -> _DST_TRANSCOMPW {
        _DST_TRANSCOMPW { w: self }
    }
    #[doc = "Bit 5 - Source Decode Error"]
    #[inline]
    pub fn src_dec_err(&mut self) -> _SRC_DEC_ERRW {
        _SRC_DEC_ERRW { w: self }
    }
    #[doc = "Bit 6 - Destination Decode Error"]
    #[inline]
    pub fn dst_dec_err(&mut self) -> _DST_DEC_ERRW {
        _DST_DEC_ERRW { w: self }
    }
    #[doc = "Bit 7 - Source Slave Error"]
    #[inline]
    pub fn src_slv_err(&mut self) -> _SRC_SLV_ERRW {
        _SRC_SLV_ERRW { w: self }
    }
    #[doc = "Bit 8 - Destination Slave Error"]
    #[inline]
    pub fn dst_slv_err(&mut self) -> _DST_SLV_ERRW {
        _DST_SLV_ERRW { w: self }
    }
    #[doc = "Bit 9 - LLI Read Decode Error Status Enable"]
    #[inline]
    pub fn lli_rd_dec_err(&mut self) -> _LLI_RD_DEC_ERRW {
        _LLI_RD_DEC_ERRW { w: self }
    }
    #[doc = "Bit 10 - LLI WRITE Decode Error"]
    #[inline]
    pub fn lli_wr_dec_err(&mut self) -> _LLI_WR_DEC_ERRW {
        _LLI_WR_DEC_ERRW { w: self }
    }
    #[doc = "Bit 11 - LLI Read Slave Error"]
    #[inline]
    pub fn lli_rd_slv_err(&mut self) -> _LLI_RD_SLV_ERRW {
        _LLI_RD_SLV_ERRW { w: self }
    }
    #[doc = "Bit 12 - LLI WRITE Slave Error"]
    #[inline]
    pub fn lli_wr_slv_err(&mut self) -> _LLI_WR_SLV_ERRW {
        _LLI_WR_SLV_ERRW { w: self }
    }
}
