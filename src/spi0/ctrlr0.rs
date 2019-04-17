#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRLR0 {
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
#[doc = "Possible values of the field `iatm`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IATMR {
    #[doc = "STANDARD"]
    STANDARD,
    #[doc = "ADDR_STANDARD"]
    ADDR_STANDARD,
    #[doc = "AS_FRAME_FORMAT"]
    AS_FRAME_FORMAT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IATMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IATMR::STANDARD => 0,
            IATMR::ADDR_STANDARD => 1,
            IATMR::AS_FRAME_FORMAT => 2,
            IATMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IATMR {
        match value {
            0 => IATMR::STANDARD,
            1 => IATMR::ADDR_STANDARD,
            2 => IATMR::AS_FRAME_FORMAT,
            i => IATMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline]
    pub fn is_standard(&self) -> bool {
        *self == IATMR::STANDARD
    }
    #[doc = "Checks if the value of the field is `ADDR_STANDARD`"]
    #[inline]
    pub fn is_addr_standard(&self) -> bool {
        *self == IATMR::ADDR_STANDARD
    }
    #[doc = "Checks if the value of the field is `AS_FRAME_FORMAT`"]
    #[inline]
    pub fn is_as_frame_format(&self) -> bool {
        *self == IATMR::AS_FRAME_FORMAT
    }
}
#[doc = r" Value of the field"]
pub struct ADDR_LENGTHR {
    bits: u8,
}
impl ADDR_LENGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `work_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WORK_MODER {
    #[doc = "MODE_0"]
    MODE0,
    #[doc = "MODE_1"]
    MODE1,
    #[doc = "MODE_2"]
    MODE2,
    #[doc = "MODE_3"]
    MODE3,
}
impl WORK_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WORK_MODER::MODE0 => 0,
            WORK_MODER::MODE1 => 1,
            WORK_MODER::MODE2 => 2,
            WORK_MODER::MODE3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WORK_MODER {
        match value {
            0 => WORK_MODER::MODE0,
            1 => WORK_MODER::MODE1,
            2 => WORK_MODER::MODE2,
            3 => WORK_MODER::MODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline]
    pub fn is_mode0(&self) -> bool {
        *self == WORK_MODER::MODE0
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline]
    pub fn is_mode1(&self) -> bool {
        *self == WORK_MODER::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline]
    pub fn is_mode2(&self) -> bool {
        *self == WORK_MODER::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline]
    pub fn is_mode3(&self) -> bool {
        *self == WORK_MODER::MODE3
    }
}
#[doc = "Possible values of the field `tmod`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMODR {
    #[doc = "TRANS_RECV"]
    TRANS_RECV,
    #[doc = "TRANS"]
    TRANS,
    #[doc = "RECV"]
    RECV,
    #[doc = "EEROM"]
    EEROM,
}
impl TMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMODR::TRANS_RECV => 0,
            TMODR::TRANS => 1,
            TMODR::RECV => 2,
            TMODR::EEROM => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMODR {
        match value {
            0 => TMODR::TRANS_RECV,
            1 => TMODR::TRANS,
            2 => TMODR::RECV,
            3 => TMODR::EEROM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRANS_RECV`"]
    #[inline]
    pub fn is_trans_recv(&self) -> bool {
        *self == TMODR::TRANS_RECV
    }
    #[doc = "Checks if the value of the field is `TRANS`"]
    #[inline]
    pub fn is_trans(&self) -> bool {
        *self == TMODR::TRANS
    }
    #[doc = "Checks if the value of the field is `RECV`"]
    #[inline]
    pub fn is_recv(&self) -> bool {
        *self == TMODR::RECV
    }
    #[doc = "Checks if the value of the field is `EEROM`"]
    #[inline]
    pub fn is_eerom(&self) -> bool {
        *self == TMODR::EEROM
    }
}
#[doc = r" Value of the field"]
pub struct WAIT_CYCLESR {
    bits: u8,
}
impl WAIT_CYCLESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `frame_format`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAME_FORMATR {
    #[doc = "STANDARD"]
    STANDARD,
    #[doc = "DUAL"]
    DUAL,
    #[doc = "QUAD"]
    QUAD,
    #[doc = "OCTAL"]
    OCTAL,
}
impl FRAME_FORMATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FRAME_FORMATR::STANDARD => 0,
            FRAME_FORMATR::DUAL => 1,
            FRAME_FORMATR::QUAD => 2,
            FRAME_FORMATR::OCTAL => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FRAME_FORMATR {
        match value {
            0 => FRAME_FORMATR::STANDARD,
            1 => FRAME_FORMATR::DUAL,
            2 => FRAME_FORMATR::QUAD,
            3 => FRAME_FORMATR::OCTAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline]
    pub fn is_standard(&self) -> bool {
        *self == FRAME_FORMATR::STANDARD
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline]
    pub fn is_dual(&self) -> bool {
        *self == FRAME_FORMATR::DUAL
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline]
    pub fn is_quad(&self) -> bool {
        *self == FRAME_FORMATR::QUAD
    }
    #[doc = "Checks if the value of the field is `OCTAL`"]
    #[inline]
    pub fn is_octal(&self) -> bool {
        *self == FRAME_FORMATR::OCTAL
    }
}
#[doc = r" Value of the field"]
pub struct DATA_LENGTHR {
    bits: u8,
}
impl DATA_LENGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `iatm`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IATMW {
    #[doc = "STANDARD"]
    STANDARD,
    #[doc = "ADDR_STANDARD"]
    ADDR_STANDARD,
    #[doc = "AS_FRAME_FORMAT"]
    AS_FRAME_FORMAT,
}
impl IATMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IATMW::STANDARD => 0,
            IATMW::ADDR_STANDARD => 1,
            IATMW::AS_FRAME_FORMAT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IATMW<'a> {
    w: &'a mut W,
}
impl<'a> _IATMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IATMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "STANDARD"]
    #[inline]
    pub fn standard(self) -> &'a mut W {
        self.variant(IATMW::STANDARD)
    }
    #[doc = "ADDR_STANDARD"]
    #[inline]
    pub fn addr_standard(self) -> &'a mut W {
        self.variant(IATMW::ADDR_STANDARD)
    }
    #[doc = "AS_FRAME_FORMAT"]
    #[inline]
    pub fn as_frame_format(self) -> &'a mut W {
        self.variant(IATMW::AS_FRAME_FORMAT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADDR_LENGTHW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDR_LENGTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `work_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WORK_MODEW {
    #[doc = "MODE_0"]
    MODE0,
    #[doc = "MODE_1"]
    MODE1,
    #[doc = "MODE_2"]
    MODE2,
    #[doc = "MODE_3"]
    MODE3,
}
impl WORK_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WORK_MODEW::MODE0 => 0,
            WORK_MODEW::MODE1 => 1,
            WORK_MODEW::MODE2 => 2,
            WORK_MODEW::MODE3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WORK_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _WORK_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WORK_MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "MODE_0"]
    #[inline]
    pub fn mode0(self) -> &'a mut W {
        self.variant(WORK_MODEW::MODE0)
    }
    #[doc = "MODE_1"]
    #[inline]
    pub fn mode1(self) -> &'a mut W {
        self.variant(WORK_MODEW::MODE1)
    }
    #[doc = "MODE_2"]
    #[inline]
    pub fn mode2(self) -> &'a mut W {
        self.variant(WORK_MODEW::MODE2)
    }
    #[doc = "MODE_3"]
    #[inline]
    pub fn mode3(self) -> &'a mut W {
        self.variant(WORK_MODEW::MODE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `tmod`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMODW {
    #[doc = "TRANS_RECV"]
    TRANS_RECV,
    #[doc = "TRANS"]
    TRANS,
    #[doc = "RECV"]
    RECV,
    #[doc = "EEROM"]
    EEROM,
}
impl TMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMODW::TRANS_RECV => 0,
            TMODW::TRANS => 1,
            TMODW::RECV => 2,
            TMODW::EEROM => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMODW<'a> {
    w: &'a mut W,
}
impl<'a> _TMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "TRANS_RECV"]
    #[inline]
    pub fn trans_recv(self) -> &'a mut W {
        self.variant(TMODW::TRANS_RECV)
    }
    #[doc = "TRANS"]
    #[inline]
    pub fn trans(self) -> &'a mut W {
        self.variant(TMODW::TRANS)
    }
    #[doc = "RECV"]
    #[inline]
    pub fn recv(self) -> &'a mut W {
        self.variant(TMODW::RECV)
    }
    #[doc = "EEROM"]
    #[inline]
    pub fn eerom(self) -> &'a mut W {
        self.variant(TMODW::EEROM)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WAIT_CYCLESW<'a> {
    w: &'a mut W,
}
impl<'a> _WAIT_CYCLESW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `frame_format`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAME_FORMATW {
    #[doc = "STANDARD"]
    STANDARD,
    #[doc = "DUAL"]
    DUAL,
    #[doc = "QUAD"]
    QUAD,
    #[doc = "OCTAL"]
    OCTAL,
}
impl FRAME_FORMATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FRAME_FORMATW::STANDARD => 0,
            FRAME_FORMATW::DUAL => 1,
            FRAME_FORMATW::QUAD => 2,
            FRAME_FORMATW::OCTAL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRAME_FORMATW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAME_FORMATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRAME_FORMATW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "STANDARD"]
    #[inline]
    pub fn standard(self) -> &'a mut W {
        self.variant(FRAME_FORMATW::STANDARD)
    }
    #[doc = "DUAL"]
    #[inline]
    pub fn dual(self) -> &'a mut W {
        self.variant(FRAME_FORMATW::DUAL)
    }
    #[doc = "QUAD"]
    #[inline]
    pub fn quad(self) -> &'a mut W {
        self.variant(FRAME_FORMATW::QUAD)
    }
    #[doc = "OCTAL"]
    #[inline]
    pub fn octal(self) -> &'a mut W {
        self.variant(FRAME_FORMATW::OCTAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATA_LENGTHW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_LENGTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    #[doc = "Bits 0:1 - instruction_address_trans_mode"]
    #[inline]
    pub fn iatm(&self) -> IATMR {
        IATMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:5 - ADDR_LENGTH"]
    #[inline]
    pub fn addr_length(&self) -> ADDR_LENGTHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADDR_LENGTHR { bits }
    }
    #[doc = "Bits 6:7 - WORK_MODE"]
    #[inline]
    pub fn work_mode(&self) -> WORK_MODER {
        WORK_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - TRANSFER_MODE"]
    #[inline]
    pub fn tmod(&self) -> TMODR {
        TMODR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:15 - WAIT_CYCLES"]
    #[inline]
    pub fn wait_cycles(&self) -> WAIT_CYCLESR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WAIT_CYCLESR { bits }
    }
    #[doc = "Bits 21:22 - FRAME_FORMAT"]
    #[inline]
    pub fn frame_format(&self) -> FRAME_FORMATR {
        FRAME_FORMATR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:20 - DATA_BIT_LENGTH"]
    #[inline]
    pub fn data_length(&self) -> DATA_LENGTHR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATA_LENGTHR { bits }
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
    #[doc = "Bits 0:1 - instruction_address_trans_mode"]
    #[inline]
    pub fn iatm(&mut self) -> _IATMW {
        _IATMW { w: self }
    }
    #[doc = "Bits 2:5 - ADDR_LENGTH"]
    #[inline]
    pub fn addr_length(&mut self) -> _ADDR_LENGTHW {
        _ADDR_LENGTHW { w: self }
    }
    #[doc = "Bits 6:7 - WORK_MODE"]
    #[inline]
    pub fn work_mode(&mut self) -> _WORK_MODEW {
        _WORK_MODEW { w: self }
    }
    #[doc = "Bits 8:9 - TRANSFER_MODE"]
    #[inline]
    pub fn tmod(&mut self) -> _TMODW {
        _TMODW { w: self }
    }
    #[doc = "Bits 11:15 - WAIT_CYCLES"]
    #[inline]
    pub fn wait_cycles(&mut self) -> _WAIT_CYCLESW {
        _WAIT_CYCLESW { w: self }
    }
    #[doc = "Bits 21:22 - FRAME_FORMAT"]
    #[inline]
    pub fn frame_format(&mut self) -> _FRAME_FORMATW {
        _FRAME_FORMATW { w: self }
    }
    #[doc = "Bits 16:20 - DATA_BIT_LENGTH"]
    #[inline]
    pub fn data_length(&mut self) -> _DATA_LENGTHW {
        _DATA_LENGTHW { w: self }
    }
}
