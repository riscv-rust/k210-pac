#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DVP_CFG {
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
pub struct START_INT_ENABLER {
    bits: bool,
}
impl START_INT_ENABLER {
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
pub struct FINISH_INT_ENABLER {
    bits: bool,
}
impl FINISH_INT_ENABLER {
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
pub struct AI_OUTPUT_ENABLER {
    bits: bool,
}
impl AI_OUTPUT_ENABLER {
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
pub struct DISPLAY_OUTPUT_ENABLER {
    bits: bool,
}
impl DISPLAY_OUTPUT_ENABLER {
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
pub struct AUTO_ENABLER {
    bits: bool,
}
impl AUTO_ENABLER {
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
pub struct BURST_SIZE_4BEATSR {
    bits: bool,
}
impl BURST_SIZE_4BEATSR {
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
#[doc = "Possible values of the field `FORMAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORMATR {
    #[doc = "RGB_FORMAT"]
    RGB,
    #[doc = "YUV_FORMAT"]
    YUV,
    #[doc = "Y_FORMAT"]
    Y,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FORMATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FORMATR::RGB => 0,
            FORMATR::YUV => 1,
            FORMATR::Y => 3,
            FORMATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FORMATR {
        match value {
            0 => FORMATR::RGB,
            1 => FORMATR::YUV,
            3 => FORMATR::Y,
            i => FORMATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RGB`"]
    #[inline]
    pub fn is_rgb(&self) -> bool {
        *self == FORMATR::RGB
    }
    #[doc = "Checks if the value of the field is `YUV`"]
    #[inline]
    pub fn is_yuv(&self) -> bool {
        *self == FORMATR::YUV
    }
    #[doc = "Checks if the value of the field is `Y`"]
    #[inline]
    pub fn is_y(&self) -> bool {
        *self == FORMATR::Y
    }
}
#[doc = r" Value of the field"]
pub struct HREF_BURST_NUMR {
    bits: u8,
}
impl HREF_BURST_NUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LINE_NUMR {
    bits: u16,
}
impl LINE_NUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _START_INT_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _START_INT_ENABLEW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FINISH_INT_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _FINISH_INT_ENABLEW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AI_OUTPUT_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _AI_OUTPUT_ENABLEW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DISPLAY_OUTPUT_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _DISPLAY_OUTPUT_ENABLEW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AUTO_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTO_ENABLEW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BURST_SIZE_4BEATSW<'a> {
    w: &'a mut W,
}
impl<'a> _BURST_SIZE_4BEATSW<'a> {
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
#[doc = "Values that can be written to the field `FORMAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORMATW {
    #[doc = "RGB_FORMAT"]
    RGB,
    #[doc = "YUV_FORMAT"]
    YUV,
    #[doc = "Y_FORMAT"]
    Y,
}
impl FORMATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FORMATW::RGB => 0,
            FORMATW::YUV => 1,
            FORMATW::Y => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FORMATW<'a> {
    w: &'a mut W,
}
impl<'a> _FORMATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FORMATW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "RGB_FORMAT"]
    #[inline]
    pub fn rgb(self) -> &'a mut W {
        self.variant(FORMATW::RGB)
    }
    #[doc = "YUV_FORMAT"]
    #[inline]
    pub fn yuv(self) -> &'a mut W {
        self.variant(FORMATW::YUV)
    }
    #[doc = "Y_FORMAT"]
    #[inline]
    pub fn y(self) -> &'a mut W {
        self.variant(FORMATW::Y)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HREF_BURST_NUMW<'a> {
    w: &'a mut W,
}
impl<'a> _HREF_BURST_NUMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LINE_NUMW<'a> {
    w: &'a mut W,
}
impl<'a> _LINE_NUMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - START_INT_ENABLE"]
    #[inline]
    pub fn start_int_enable(&self) -> START_INT_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        START_INT_ENABLER { bits }
    }
    #[doc = "Bit 1 - FINISH_INT_ENABLE"]
    #[inline]
    pub fn finish_int_enable(&self) -> FINISH_INT_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FINISH_INT_ENABLER { bits }
    }
    #[doc = "Bit 2 - AI_OUTPUT_ENABLE"]
    #[inline]
    pub fn ai_output_enable(&self) -> AI_OUTPUT_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AI_OUTPUT_ENABLER { bits }
    }
    #[doc = "Bit 3 - DISPLAY_OUTPUT_ENABLE"]
    #[inline]
    pub fn display_output_enable(&self) -> DISPLAY_OUTPUT_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISPLAY_OUTPUT_ENABLER { bits }
    }
    #[doc = "Bit 4 - AUTO_ENABLE"]
    #[inline]
    pub fn auto_enable(&self) -> AUTO_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTO_ENABLER { bits }
    }
    #[doc = "Bit 8 - BURST_SIZE_4BEATS"]
    #[inline]
    pub fn burst_size_4beats(&self) -> BURST_SIZE_4BEATSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BURST_SIZE_4BEATSR { bits }
    }
    #[doc = "Bits 9:10 - FORMAT"]
    #[inline]
    pub fn format(&self) -> FORMATR {
        FORMATR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:19 - HREF_BURST_NUM"]
    #[inline]
    pub fn href_burst_num(&self) -> HREF_BURST_NUMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HREF_BURST_NUMR { bits }
    }
    #[doc = "Bits 20:29 - LINE_NUM"]
    #[inline]
    pub fn line_num(&self) -> LINE_NUMR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        LINE_NUMR { bits }
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
    #[doc = "Bit 0 - START_INT_ENABLE"]
    #[inline]
    pub fn start_int_enable(&mut self) -> _START_INT_ENABLEW {
        _START_INT_ENABLEW { w: self }
    }
    #[doc = "Bit 1 - FINISH_INT_ENABLE"]
    #[inline]
    pub fn finish_int_enable(&mut self) -> _FINISH_INT_ENABLEW {
        _FINISH_INT_ENABLEW { w: self }
    }
    #[doc = "Bit 2 - AI_OUTPUT_ENABLE"]
    #[inline]
    pub fn ai_output_enable(&mut self) -> _AI_OUTPUT_ENABLEW {
        _AI_OUTPUT_ENABLEW { w: self }
    }
    #[doc = "Bit 3 - DISPLAY_OUTPUT_ENABLE"]
    #[inline]
    pub fn display_output_enable(&mut self) -> _DISPLAY_OUTPUT_ENABLEW {
        _DISPLAY_OUTPUT_ENABLEW { w: self }
    }
    #[doc = "Bit 4 - AUTO_ENABLE"]
    #[inline]
    pub fn auto_enable(&mut self) -> _AUTO_ENABLEW {
        _AUTO_ENABLEW { w: self }
    }
    #[doc = "Bit 8 - BURST_SIZE_4BEATS"]
    #[inline]
    pub fn burst_size_4beats(&mut self) -> _BURST_SIZE_4BEATSW {
        _BURST_SIZE_4BEATSW { w: self }
    }
    #[doc = "Bits 9:10 - FORMAT"]
    #[inline]
    pub fn format(&mut self) -> _FORMATW {
        _FORMATW { w: self }
    }
    #[doc = "Bits 12:19 - HREF_BURST_NUM"]
    #[inline]
    pub fn href_burst_num(&mut self) -> _HREF_BURST_NUMW {
        _HREF_BURST_NUMW { w: self }
    }
    #[doc = "Bits 20:29 - LINE_NUM"]
    #[inline]
    pub fn line_num(&mut self) -> _LINE_NUMW {
        _LINE_NUMW { w: self }
    }
}
