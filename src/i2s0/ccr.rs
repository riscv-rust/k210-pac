#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCR {
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
#[doc = "Possible values of the field `clk_gate`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_GATER {
    #[doc = "Clock gating is disabled"]
    NO,
    #[doc = "Gating after 12 sclk cycles"]
    CYCLES12,
    #[doc = "Gating after 16 sclk cycles"]
    CYCLES16,
    #[doc = "Gating after 20 sclk cycles"]
    CYCLES20,
    #[doc = "Gating after 24 sclk cycles"]
    CYCLES24,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLK_GATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLK_GATER::NO => 0,
            CLK_GATER::CYCLES12 => 1,
            CLK_GATER::CYCLES16 => 2,
            CLK_GATER::CYCLES20 => 3,
            CLK_GATER::CYCLES24 => 4,
            CLK_GATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLK_GATER {
        match value {
            0 => CLK_GATER::NO,
            1 => CLK_GATER::CYCLES12,
            2 => CLK_GATER::CYCLES16,
            3 => CLK_GATER::CYCLES20,
            4 => CLK_GATER::CYCLES24,
            i => CLK_GATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == CLK_GATER::NO
    }
    #[doc = "Checks if the value of the field is `CYCLES12`"]
    #[inline]
    pub fn is_cycles12(&self) -> bool {
        *self == CLK_GATER::CYCLES12
    }
    #[doc = "Checks if the value of the field is `CYCLES16`"]
    #[inline]
    pub fn is_cycles16(&self) -> bool {
        *self == CLK_GATER::CYCLES16
    }
    #[doc = "Checks if the value of the field is `CYCLES20`"]
    #[inline]
    pub fn is_cycles20(&self) -> bool {
        *self == CLK_GATER::CYCLES20
    }
    #[doc = "Checks if the value of the field is `CYCLES24`"]
    #[inline]
    pub fn is_cycles24(&self) -> bool {
        *self == CLK_GATER::CYCLES24
    }
}
#[doc = "Possible values of the field `clk_word_size`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_WORD_SIZER {
    #[doc = "16 sclk cycles"]
    CYCLES16,
    #[doc = "24 sclk cycles"]
    CYCLES24,
    #[doc = "32 sclk cycles"]
    CYCLES32,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLK_WORD_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLK_WORD_SIZER::CYCLES16 => 0,
            CLK_WORD_SIZER::CYCLES24 => 1,
            CLK_WORD_SIZER::CYCLES32 => 2,
            CLK_WORD_SIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLK_WORD_SIZER {
        match value {
            0 => CLK_WORD_SIZER::CYCLES16,
            1 => CLK_WORD_SIZER::CYCLES24,
            2 => CLK_WORD_SIZER::CYCLES32,
            i => CLK_WORD_SIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES16`"]
    #[inline]
    pub fn is_cycles16(&self) -> bool {
        *self == CLK_WORD_SIZER::CYCLES16
    }
    #[doc = "Checks if the value of the field is `CYCLES24`"]
    #[inline]
    pub fn is_cycles24(&self) -> bool {
        *self == CLK_WORD_SIZER::CYCLES24
    }
    #[doc = "Checks if the value of the field is `CYCLES32`"]
    #[inline]
    pub fn is_cycles32(&self) -> bool {
        *self == CLK_WORD_SIZER::CYCLES32
    }
}
#[doc = "Possible values of the field `align_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIGN_MODER {
    #[doc = "Standard I2S format"]
    STANDARD,
    #[doc = "Right aligned format"]
    RIGHT,
    #[doc = "Left aligned format"]
    LEFT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ALIGN_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALIGN_MODER::STANDARD => 1,
            ALIGN_MODER::RIGHT => 2,
            ALIGN_MODER::LEFT => 4,
            ALIGN_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALIGN_MODER {
        match value {
            1 => ALIGN_MODER::STANDARD,
            2 => ALIGN_MODER::RIGHT,
            4 => ALIGN_MODER::LEFT,
            i => ALIGN_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline]
    pub fn is_standard(&self) -> bool {
        *self == ALIGN_MODER::STANDARD
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline]
    pub fn is_right(&self) -> bool {
        *self == ALIGN_MODER::RIGHT
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline]
    pub fn is_left(&self) -> bool {
        *self == ALIGN_MODER::LEFT
    }
}
#[doc = r" Value of the field"]
pub struct DMA_TX_ENR {
    bits: bool,
}
impl DMA_TX_ENR {
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
pub struct DMA_RX_ENR {
    bits: bool,
}
impl DMA_RX_ENR {
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
pub struct DMA_DIVIDE_16R {
    bits: bool,
}
impl DMA_DIVIDE_16R {
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
pub struct SIGN_EXPAND_ENR {
    bits: bool,
}
impl SIGN_EXPAND_ENR {
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
#[doc = "Values that can be written to the field `clk_gate`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_GATEW {
    #[doc = "Clock gating is disabled"]
    NO,
    #[doc = "Gating after 12 sclk cycles"]
    CYCLES12,
    #[doc = "Gating after 16 sclk cycles"]
    CYCLES16,
    #[doc = "Gating after 20 sclk cycles"]
    CYCLES20,
    #[doc = "Gating after 24 sclk cycles"]
    CYCLES24,
}
impl CLK_GATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLK_GATEW::NO => 0,
            CLK_GATEW::CYCLES12 => 1,
            CLK_GATEW::CYCLES16 => 2,
            CLK_GATEW::CYCLES20 => 3,
            CLK_GATEW::CYCLES24 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLK_GATEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_GATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLK_GATEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clock gating is disabled"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(CLK_GATEW::NO)
    }
    #[doc = "Gating after 12 sclk cycles"]
    #[inline]
    pub fn cycles12(self) -> &'a mut W {
        self.variant(CLK_GATEW::CYCLES12)
    }
    #[doc = "Gating after 16 sclk cycles"]
    #[inline]
    pub fn cycles16(self) -> &'a mut W {
        self.variant(CLK_GATEW::CYCLES16)
    }
    #[doc = "Gating after 20 sclk cycles"]
    #[inline]
    pub fn cycles20(self) -> &'a mut W {
        self.variant(CLK_GATEW::CYCLES20)
    }
    #[doc = "Gating after 24 sclk cycles"]
    #[inline]
    pub fn cycles24(self) -> &'a mut W {
        self.variant(CLK_GATEW::CYCLES24)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `clk_word_size`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_WORD_SIZEW {
    #[doc = "16 sclk cycles"]
    CYCLES16,
    #[doc = "24 sclk cycles"]
    CYCLES24,
    #[doc = "32 sclk cycles"]
    CYCLES32,
}
impl CLK_WORD_SIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLK_WORD_SIZEW::CYCLES16 => 0,
            CLK_WORD_SIZEW::CYCLES24 => 1,
            CLK_WORD_SIZEW::CYCLES32 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLK_WORD_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_WORD_SIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLK_WORD_SIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "16 sclk cycles"]
    #[inline]
    pub fn cycles16(self) -> &'a mut W {
        self.variant(CLK_WORD_SIZEW::CYCLES16)
    }
    #[doc = "24 sclk cycles"]
    #[inline]
    pub fn cycles24(self) -> &'a mut W {
        self.variant(CLK_WORD_SIZEW::CYCLES24)
    }
    #[doc = "32 sclk cycles"]
    #[inline]
    pub fn cycles32(self) -> &'a mut W {
        self.variant(CLK_WORD_SIZEW::CYCLES32)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `align_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIGN_MODEW {
    #[doc = "Standard I2S format"]
    STANDARD,
    #[doc = "Right aligned format"]
    RIGHT,
    #[doc = "Left aligned format"]
    LEFT,
}
impl ALIGN_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ALIGN_MODEW::STANDARD => 1,
            ALIGN_MODEW::RIGHT => 2,
            ALIGN_MODEW::LEFT => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALIGN_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ALIGN_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALIGN_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Standard I2S format"]
    #[inline]
    pub fn standard(self) -> &'a mut W {
        self.variant(ALIGN_MODEW::STANDARD)
    }
    #[doc = "Right aligned format"]
    #[inline]
    pub fn right(self) -> &'a mut W {
        self.variant(ALIGN_MODEW::RIGHT)
    }
    #[doc = "Left aligned format"]
    #[inline]
    pub fn left(self) -> &'a mut W {
        self.variant(ALIGN_MODEW::LEFT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA_TX_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_TX_ENW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA_RX_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_RX_ENW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA_DIVIDE_16W<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_DIVIDE_16W<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SIGN_EXPAND_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SIGN_EXPAND_ENW<'a> {
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
    #[doc = "Bits 0:2 - Gating of sclk"]
    #[inline]
    pub fn clk_gate(&self) -> CLK_GATER {
        CLK_GATER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:4 - The number of sclk cycles for which the word select line stayd in the left aligned or right aligned mode"]
    #[inline]
    pub fn clk_word_size(&self) -> CLK_WORD_SIZER {
        CLK_WORD_SIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:7 - Alignment mode setting"]
    #[inline]
    pub fn align_mode(&self) -> ALIGN_MODER {
        ALIGN_MODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - DMA transmit enable control"]
    #[inline]
    pub fn dma_tx_en(&self) -> DMA_TX_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA_TX_ENR { bits }
    }
    #[doc = "Bit 9 - DMA receive enable control"]
    #[inline]
    pub fn dma_rx_en(&self) -> DMA_RX_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA_RX_ENR { bits }
    }
    #[doc = "Bit 10 - Split 32bit data to two 16 bit data and filled in left and right channel. Used with dma_tx_en or dma_rx_en"]
    #[inline]
    pub fn dma_divide_16(&self) -> DMA_DIVIDE_16R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA_DIVIDE_16R { bits }
    }
    #[doc = "Bit 11 - SIGN_EXPAND_EN"]
    #[inline]
    pub fn sign_expand_en(&self) -> SIGN_EXPAND_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SIGN_EXPAND_ENR { bits }
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
    #[doc = "Bits 0:2 - Gating of sclk"]
    #[inline]
    pub fn clk_gate(&mut self) -> _CLK_GATEW {
        _CLK_GATEW { w: self }
    }
    #[doc = "Bits 3:4 - The number of sclk cycles for which the word select line stayd in the left aligned or right aligned mode"]
    #[inline]
    pub fn clk_word_size(&mut self) -> _CLK_WORD_SIZEW {
        _CLK_WORD_SIZEW { w: self }
    }
    #[doc = "Bits 5:7 - Alignment mode setting"]
    #[inline]
    pub fn align_mode(&mut self) -> _ALIGN_MODEW {
        _ALIGN_MODEW { w: self }
    }
    #[doc = "Bit 8 - DMA transmit enable control"]
    #[inline]
    pub fn dma_tx_en(&mut self) -> _DMA_TX_ENW {
        _DMA_TX_ENW { w: self }
    }
    #[doc = "Bit 9 - DMA receive enable control"]
    #[inline]
    pub fn dma_rx_en(&mut self) -> _DMA_RX_ENW {
        _DMA_RX_ENW { w: self }
    }
    #[doc = "Bit 10 - Split 32bit data to two 16 bit data and filled in left and right channel. Used with dma_tx_en or dma_rx_en"]
    #[inline]
    pub fn dma_divide_16(&mut self) -> _DMA_DIVIDE_16W {
        _DMA_DIVIDE_16W { w: self }
    }
    #[doc = "Bit 11 - SIGN_EXPAND_EN"]
    #[inline]
    pub fn sign_expand_en(&mut self) -> _SIGN_EXPAND_ENW {
        _SIGN_EXPAND_ENW { w: self }
    }
}
