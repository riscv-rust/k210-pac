#[doc = r" Value read from the register"]
pub struct R {
    bits: u64,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u64,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `point`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POINTR {
    #[doc = "512 point"]
    P512,
    #[doc = "256 point"]
    P256,
    #[doc = "128 point"]
    P128,
    #[doc = "64 point"]
    P64,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl POINTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            POINTR::P512 => 0,
            POINTR::P256 => 1,
            POINTR::P128 => 2,
            POINTR::P64 => 3,
            POINTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> POINTR {
        match value {
            0 => POINTR::P512,
            1 => POINTR::P256,
            2 => POINTR::P128,
            3 => POINTR::P64,
            i => POINTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `P512`"]
    #[inline]
    pub fn is_p512(&self) -> bool {
        *self == POINTR::P512
    }
    #[doc = "Checks if the value of the field is `P256`"]
    #[inline]
    pub fn is_p256(&self) -> bool {
        *self == POINTR::P256
    }
    #[doc = "Checks if the value of the field is `P128`"]
    #[inline]
    pub fn is_p128(&self) -> bool {
        *self == POINTR::P128
    }
    #[doc = "Checks if the value of the field is `P64`"]
    #[inline]
    pub fn is_p64(&self) -> bool {
        *self == POINTR::P64
    }
}
#[doc = "Possible values of the field `mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "FFT mode"]
    BACKWARD,
    #[doc = "Inverse FFT mode"]
    FORWARD,
}
impl MODER {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MODER::BACKWARD => false,
            MODER::FORWARD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODER {
        match value {
            false => MODER::BACKWARD,
            true => MODER::FORWARD,
        }
    }
    #[doc = "Checks if the value of the field is `BACKWARD`"]
    #[inline]
    pub fn is_backward(&self) -> bool {
        *self == MODER::BACKWARD
    }
    #[doc = "Checks if the value of the field is `FORWARD`"]
    #[inline]
    pub fn is_forward(&self) -> bool {
        *self == MODER::FORWARD
    }
}
#[doc = r" Value of the field"]
pub struct SHIFTR {
    bits: u16,
}
impl SHIFTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ENABLER {
    bits: bool,
}
impl ENABLER {
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
pub struct DMA_SENDR {
    bits: bool,
}
impl DMA_SENDR {
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
#[doc = "Possible values of the field `input_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT_MODER {
    #[doc = "RIRI (real imaginary interleaved)"]
    RIRI,
    #[doc = "RRRR (only real part)"]
    RRRR,
    #[doc = "First input the real part and then input the imaginary part"]
    RRII,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INPUT_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT_MODER::RIRI => 0,
            INPUT_MODER::RRRR => 1,
            INPUT_MODER::RRII => 2,
            INPUT_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT_MODER {
        match value {
            0 => INPUT_MODER::RIRI,
            1 => INPUT_MODER::RRRR,
            2 => INPUT_MODER::RRII,
            i => INPUT_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RIRI`"]
    #[inline]
    pub fn is_riri(&self) -> bool {
        *self == INPUT_MODER::RIRI
    }
    #[doc = "Checks if the value of the field is `RRRR`"]
    #[inline]
    pub fn is_rrrr(&self) -> bool {
        *self == INPUT_MODER::RRRR
    }
    #[doc = "Checks if the value of the field is `RRII`"]
    #[inline]
    pub fn is_rrii(&self) -> bool {
        *self == INPUT_MODER::RRII
    }
}
#[doc = "Possible values of the field `data_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_MODER {
    #[doc = "64 bit effective"]
    WIDTH_64,
    #[doc = "128 bit effective"]
    WIDTH_128,
}
impl DATA_MODER {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DATA_MODER::WIDTH_64 => false,
            DATA_MODER::WIDTH_128 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA_MODER {
        match value {
            false => DATA_MODER::WIDTH_64,
            true => DATA_MODER::WIDTH_128,
        }
    }
    #[doc = "Checks if the value of the field is `WIDTH_64`"]
    #[inline]
    pub fn is_width_64(&self) -> bool {
        *self == DATA_MODER::WIDTH_64
    }
    #[doc = "Checks if the value of the field is `WIDTH_128`"]
    #[inline]
    pub fn is_width_128(&self) -> bool {
        *self == DATA_MODER::WIDTH_128
    }
}
#[doc = "Values that can be written to the field `point`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POINTW {
    #[doc = "512 point"]
    P512,
    #[doc = "256 point"]
    P256,
    #[doc = "128 point"]
    P128,
    #[doc = "64 point"]
    P64,
}
impl POINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            POINTW::P512 => 0,
            POINTW::P256 => 1,
            POINTW::P128 => 2,
            POINTW::P64 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POINTW<'a> {
    w: &'a mut W,
}
impl<'a> _POINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POINTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "512 point"]
    #[inline]
    pub fn p512(self) -> &'a mut W {
        self.variant(POINTW::P512)
    }
    #[doc = "256 point"]
    #[inline]
    pub fn p256(self) -> &'a mut W {
        self.variant(POINTW::P256)
    }
    #[doc = "128 point"]
    #[inline]
    pub fn p128(self) -> &'a mut W {
        self.variant(POINTW::P128)
    }
    #[doc = "64 point"]
    #[inline]
    pub fn p64(self) -> &'a mut W {
        self.variant(POINTW::P64)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODEW {
    #[doc = "FFT mode"]
    BACKWARD,
    #[doc = "Inverse FFT mode"]
    FORWARD,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODEW::BACKWARD => false,
            MODEW::FORWARD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FFT mode"]
    #[inline]
    pub fn backward(self) -> &'a mut W {
        self.variant(MODEW::BACKWARD)
    }
    #[doc = "Inverse FFT mode"]
    #[inline]
    pub fn forward(self) -> &'a mut W {
        self.variant(MODEW::FORWARD)
    }
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
pub struct _SHIFTW<'a> {
    w: &'a mut W,
}
impl<'a> _SHIFTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
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
pub struct _DMA_SENDW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_SENDW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `input_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT_MODEW {
    #[doc = "RIRI (real imaginary interleaved)"]
    RIRI,
    #[doc = "RRRR (only real part)"]
    RRRR,
    #[doc = "First input the real part and then input the imaginary part"]
    RRII,
}
impl INPUT_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT_MODEW::RIRI => 0,
            INPUT_MODEW::RRRR => 1,
            INPUT_MODEW::RRII => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "RIRI (real imaginary interleaved)"]
    #[inline]
    pub fn riri(self) -> &'a mut W {
        self.variant(INPUT_MODEW::RIRI)
    }
    #[doc = "RRRR (only real part)"]
    #[inline]
    pub fn rrrr(self) -> &'a mut W {
        self.variant(INPUT_MODEW::RRRR)
    }
    #[doc = "First input the real part and then input the imaginary part"]
    #[inline]
    pub fn rrii(self) -> &'a mut W {
        self.variant(INPUT_MODEW::RRII)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `data_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_MODEW {
    #[doc = "64 bit effective"]
    WIDTH_64,
    #[doc = "128 bit effective"]
    WIDTH_128,
}
impl DATA_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA_MODEW::WIDTH_64 => false,
            DATA_MODEW::WIDTH_128 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "64 bit effective"]
    #[inline]
    pub fn width_64(self) -> &'a mut W {
        self.variant(DATA_MODEW::WIDTH_64)
    }
    #[doc = "128 bit effective"]
    #[inline]
    pub fn width_128(self) -> &'a mut W {
        self.variant(DATA_MODEW::WIDTH_128)
    }
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u64 {
        self.bits
    }
    #[doc = "Bits 0:2 - FFT calculation data length"]
    #[inline]
    pub fn point(&self) -> POINTR {
        POINTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u64) as u8
        })
    }
    #[doc = "Bit 3 - FFT mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        })
    }
    #[doc = "Bits 4:12 - Corresponding to the nine layer butterfly shift operation, 0x0: does not shift; 0x1: shift 1st layer. ..."]
    #[inline]
    pub fn shift(&self) -> SHIFTR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u64) as u16
        };
        SHIFTR { bits }
    }
    #[doc = "Bit 13 - FFT enable"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        ENABLER { bits }
    }
    #[doc = "Bit 14 - FFT DMA enable"]
    #[inline]
    pub fn dma_send(&self) -> DMA_SENDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        DMA_SENDR { bits }
    }
    #[doc = "Bits 15:16 - Input data arrangement"]
    #[inline]
    pub fn input_mode(&self) -> INPUT_MODER {
        INPUT_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u64) as u8
        })
    }
    #[doc = "Bit 17 - Effective width of input data"]
    #[inline]
    pub fn data_mode(&self) -> DATA_MODER {
        DATA_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        })
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
    #[doc = "Bits 0:2 - FFT calculation data length"]
    #[inline]
    pub fn point(&mut self) -> _POINTW {
        _POINTW { w: self }
    }
    #[doc = "Bit 3 - FFT mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bits 4:12 - Corresponding to the nine layer butterfly shift operation, 0x0: does not shift; 0x1: shift 1st layer. ..."]
    #[inline]
    pub fn shift(&mut self) -> _SHIFTW {
        _SHIFTW { w: self }
    }
    #[doc = "Bit 13 - FFT enable"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 14 - FFT DMA enable"]
    #[inline]
    pub fn dma_send(&mut self) -> _DMA_SENDW {
        _DMA_SENDW { w: self }
    }
    #[doc = "Bits 15:16 - Input data arrangement"]
    #[inline]
    pub fn input_mode(&mut self) -> _INPUT_MODEW {
        _INPUT_MODEW { w: self }
    }
    #[doc = "Bit 17 - Effective width of input data"]
    #[inline]
    pub fn data_mode(&mut self) -> _DATA_MODEW {
        _DATA_MODEW { w: self }
    }
}
