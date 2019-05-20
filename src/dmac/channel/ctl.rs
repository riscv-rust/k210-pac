#[doc = r" Value read from the register"]
pub struct R {
    bits: u64,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u64,
}
impl super::CTL {
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
#[doc = "Possible values of the field `sms`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMSR {
    #[doc = "AXI master 1"]
    AXI_MASTER_1,
    #[doc = "AXI master 2"]
    AXI_MASTER_2,
}
impl SMSR {
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
            SMSR::AXI_MASTER_1 => false,
            SMSR::AXI_MASTER_2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMSR {
        match value {
            false => SMSR::AXI_MASTER_1,
            true => SMSR::AXI_MASTER_2,
        }
    }
    #[doc = "Checks if the value of the field is `AXI_MASTER_1`"]
    #[inline]
    pub fn is_axi_master_1(&self) -> bool {
        *self == SMSR::AXI_MASTER_1
    }
    #[doc = "Checks if the value of the field is `AXI_MASTER_2`"]
    #[inline]
    pub fn is_axi_master_2(&self) -> bool {
        *self == SMSR::AXI_MASTER_2
    }
}
#[doc = "Possible values of the field `dms`"]
pub type DMSR = SMSR;
#[doc = "Possible values of the field `sinc`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SINCR {
    #[doc = "Increment address"]
    INCREMENT,
    #[doc = "Don't increment address"]
    NOCHANGE,
}
impl SINCR {
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
            SINCR::INCREMENT => false,
            SINCR::NOCHANGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SINCR {
        match value {
            false => SINCR::INCREMENT,
            true => SINCR::NOCHANGE,
        }
    }
    #[doc = "Checks if the value of the field is `INCREMENT`"]
    #[inline]
    pub fn is_increment(&self) -> bool {
        *self == SINCR::INCREMENT
    }
    #[doc = "Checks if the value of the field is `NOCHANGE`"]
    #[inline]
    pub fn is_nochange(&self) -> bool {
        *self == SINCR::NOCHANGE
    }
}
#[doc = "Possible values of the field `dinc`"]
pub type DINCR = SINCR;
#[doc = "Possible values of the field `src_tr_width`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_TR_WIDTHR {
    #[doc = "8 bits"]
    WIDTH_8,
    #[doc = "16 bits"]
    WIDTH_16,
    #[doc = "32 bits"]
    WIDTH_32,
    #[doc = "64 bits"]
    WIDTH_64,
    #[doc = "128 bits"]
    WIDTH_128,
    #[doc = "256 bits"]
    WIDTH_256,
    #[doc = "512 bits"]
    WIDTH_512,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRC_TR_WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRC_TR_WIDTHR::WIDTH_8 => 0,
            SRC_TR_WIDTHR::WIDTH_16 => 1,
            SRC_TR_WIDTHR::WIDTH_32 => 2,
            SRC_TR_WIDTHR::WIDTH_64 => 3,
            SRC_TR_WIDTHR::WIDTH_128 => 4,
            SRC_TR_WIDTHR::WIDTH_256 => 5,
            SRC_TR_WIDTHR::WIDTH_512 => 6,
            SRC_TR_WIDTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRC_TR_WIDTHR {
        match value {
            0 => SRC_TR_WIDTHR::WIDTH_8,
            1 => SRC_TR_WIDTHR::WIDTH_16,
            2 => SRC_TR_WIDTHR::WIDTH_32,
            3 => SRC_TR_WIDTHR::WIDTH_64,
            4 => SRC_TR_WIDTHR::WIDTH_128,
            5 => SRC_TR_WIDTHR::WIDTH_256,
            6 => SRC_TR_WIDTHR::WIDTH_512,
            i => SRC_TR_WIDTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `WIDTH_8`"]
    #[inline]
    pub fn is_width_8(&self) -> bool {
        *self == SRC_TR_WIDTHR::WIDTH_8
    }
    #[doc = "Checks if the value of the field is `WIDTH_16`"]
    #[inline]
    pub fn is_width_16(&self) -> bool {
        *self == SRC_TR_WIDTHR::WIDTH_16
    }
    #[doc = "Checks if the value of the field is `WIDTH_32`"]
    #[inline]
    pub fn is_width_32(&self) -> bool {
        *self == SRC_TR_WIDTHR::WIDTH_32
    }
    #[doc = "Checks if the value of the field is `WIDTH_64`"]
    #[inline]
    pub fn is_width_64(&self) -> bool {
        *self == SRC_TR_WIDTHR::WIDTH_64
    }
    #[doc = "Checks if the value of the field is `WIDTH_128`"]
    #[inline]
    pub fn is_width_128(&self) -> bool {
        *self == SRC_TR_WIDTHR::WIDTH_128
    }
    #[doc = "Checks if the value of the field is `WIDTH_256`"]
    #[inline]
    pub fn is_width_256(&self) -> bool {
        *self == SRC_TR_WIDTHR::WIDTH_256
    }
    #[doc = "Checks if the value of the field is `WIDTH_512`"]
    #[inline]
    pub fn is_width_512(&self) -> bool {
        *self == SRC_TR_WIDTHR::WIDTH_512
    }
}
#[doc = "Possible values of the field `dst_tr_width`"]
pub type DST_TR_WIDTHR = SRC_TR_WIDTHR;
#[doc = "Possible values of the field `src_msize`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_MSIZER {
    #[doc = "1 data item"]
    LENGTH_1,
    #[doc = "4 data items"]
    LENGTH_4,
    #[doc = "8 data items"]
    LENGTH_8,
    #[doc = "16 data items"]
    LENGTH_16,
    #[doc = "32 data items"]
    LENGTH_32,
    #[doc = "64 data items"]
    LENGTH_64,
    #[doc = "128 data items"]
    LENGTH_128,
    #[doc = "256 data items"]
    LENGTH_256,
    #[doc = "512 data items"]
    LENGTH_512,
    #[doc = "1024 data items"]
    LENGTH_1024,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRC_MSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRC_MSIZER::LENGTH_1 => 0,
            SRC_MSIZER::LENGTH_4 => 1,
            SRC_MSIZER::LENGTH_8 => 2,
            SRC_MSIZER::LENGTH_16 => 3,
            SRC_MSIZER::LENGTH_32 => 4,
            SRC_MSIZER::LENGTH_64 => 5,
            SRC_MSIZER::LENGTH_128 => 6,
            SRC_MSIZER::LENGTH_256 => 7,
            SRC_MSIZER::LENGTH_512 => 8,
            SRC_MSIZER::LENGTH_1024 => 9,
            SRC_MSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRC_MSIZER {
        match value {
            0 => SRC_MSIZER::LENGTH_1,
            1 => SRC_MSIZER::LENGTH_4,
            2 => SRC_MSIZER::LENGTH_8,
            3 => SRC_MSIZER::LENGTH_16,
            4 => SRC_MSIZER::LENGTH_32,
            5 => SRC_MSIZER::LENGTH_64,
            6 => SRC_MSIZER::LENGTH_128,
            7 => SRC_MSIZER::LENGTH_256,
            8 => SRC_MSIZER::LENGTH_512,
            9 => SRC_MSIZER::LENGTH_1024,
            i => SRC_MSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LENGTH_1`"]
    #[inline]
    pub fn is_length_1(&self) -> bool {
        *self == SRC_MSIZER::LENGTH_1
    }
    #[doc = "Checks if the value of the field is `LENGTH_4`"]
    #[inline]
    pub fn is_length_4(&self) -> bool {
        *self == SRC_MSIZER::LENGTH_4
    }
    #[doc = "Checks if the value of the field is `LENGTH_8`"]
    #[inline]
    pub fn is_length_8(&self) -> bool {
        *self == SRC_MSIZER::LENGTH_8
    }
    #[doc = "Checks if the value of the field is `LENGTH_16`"]
    #[inline]
    pub fn is_length_16(&self) -> bool {
        *self == SRC_MSIZER::LENGTH_16
    }
    #[doc = "Checks if the value of the field is `LENGTH_32`"]
    #[inline]
    pub fn is_length_32(&self) -> bool {
        *self == SRC_MSIZER::LENGTH_32
    }
    #[doc = "Checks if the value of the field is `LENGTH_64`"]
    #[inline]
    pub fn is_length_64(&self) -> bool {
        *self == SRC_MSIZER::LENGTH_64
    }
    #[doc = "Checks if the value of the field is `LENGTH_128`"]
    #[inline]
    pub fn is_length_128(&self) -> bool {
        *self == SRC_MSIZER::LENGTH_128
    }
    #[doc = "Checks if the value of the field is `LENGTH_256`"]
    #[inline]
    pub fn is_length_256(&self) -> bool {
        *self == SRC_MSIZER::LENGTH_256
    }
    #[doc = "Checks if the value of the field is `LENGTH_512`"]
    #[inline]
    pub fn is_length_512(&self) -> bool {
        *self == SRC_MSIZER::LENGTH_512
    }
    #[doc = "Checks if the value of the field is `LENGTH_1024`"]
    #[inline]
    pub fn is_length_1024(&self) -> bool {
        *self == SRC_MSIZER::LENGTH_1024
    }
}
#[doc = "Possible values of the field `dst_msize`"]
pub type DST_MSIZER = SRC_MSIZER;
#[doc = r" Value of the field"]
pub struct NONPOSTED_LASTWRITE_ENR {
    bits: bool,
}
impl NONPOSTED_LASTWRITE_ENR {
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
pub struct ARLEN_ENR {
    bits: bool,
}
impl ARLEN_ENR {
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
#[doc = "Possible values of the field `arlen`"]
pub type ARLENR = SRC_MSIZER;
#[doc = r" Value of the field"]
pub struct AWLEN_ENR {
    bits: bool,
}
impl AWLEN_ENR {
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
#[doc = "Possible values of the field `awlen`"]
pub type AWLENR = SRC_MSIZER;
#[doc = r" Value of the field"]
pub struct SRC_STAT_ENR {
    bits: bool,
}
impl SRC_STAT_ENR {
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
pub struct DST_STAT_ENR {
    bits: bool,
}
impl DST_STAT_ENR {
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
pub struct IOC_BLKTFRR {
    bits: bool,
}
impl IOC_BLKTFRR {
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
pub struct SHADOWREG_OR_LLI_LASTR {
    bits: bool,
}
impl SHADOWREG_OR_LLI_LASTR {
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
pub struct SHADOWREG_OR_LLI_VALIDR {
    bits: bool,
}
impl SHADOWREG_OR_LLI_VALIDR {
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
#[doc = "Values that can be written to the field `sms`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMSW {
    #[doc = "AXI master 1"]
    AXI_MASTER_1,
    #[doc = "AXI master 2"]
    AXI_MASTER_2,
}
impl SMSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMSW::AXI_MASTER_1 => false,
            SMSW::AXI_MASTER_2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMSW<'a> {
    w: &'a mut W,
}
impl<'a> _SMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AXI master 1"]
    #[inline]
    pub fn axi_master_1(self) -> &'a mut W {
        self.variant(SMSW::AXI_MASTER_1)
    }
    #[doc = "AXI master 2"]
    #[inline]
    pub fn axi_master_2(self) -> &'a mut W {
        self.variant(SMSW::AXI_MASTER_2)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `dms`"]
pub type DMSW = SMSW;
#[doc = r" Proxy"]
pub struct _DMSW<'a> {
    w: &'a mut W,
}
impl<'a> _DMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AXI master 1"]
    #[inline]
    pub fn axi_master_1(self) -> &'a mut W {
        self.variant(SMSW::AXI_MASTER_1)
    }
    #[doc = "AXI master 2"]
    #[inline]
    pub fn axi_master_2(self) -> &'a mut W {
        self.variant(SMSW::AXI_MASTER_2)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `sinc`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SINCW {
    #[doc = "Increment address"]
    INCREMENT,
    #[doc = "Don't increment address"]
    NOCHANGE,
}
impl SINCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SINCW::INCREMENT => false,
            SINCW::NOCHANGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SINCW<'a> {
    w: &'a mut W,
}
impl<'a> _SINCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SINCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Increment address"]
    #[inline]
    pub fn increment(self) -> &'a mut W {
        self.variant(SINCW::INCREMENT)
    }
    #[doc = "Don't increment address"]
    #[inline]
    pub fn nochange(self) -> &'a mut W {
        self.variant(SINCW::NOCHANGE)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `dinc`"]
pub type DINCW = SINCW;
#[doc = r" Proxy"]
pub struct _DINCW<'a> {
    w: &'a mut W,
}
impl<'a> _DINCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DINCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Increment address"]
    #[inline]
    pub fn increment(self) -> &'a mut W {
        self.variant(SINCW::INCREMENT)
    }
    #[doc = "Don't increment address"]
    #[inline]
    pub fn nochange(self) -> &'a mut W {
        self.variant(SINCW::NOCHANGE)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `src_tr_width`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_TR_WIDTHW {
    #[doc = "8 bits"]
    WIDTH_8,
    #[doc = "16 bits"]
    WIDTH_16,
    #[doc = "32 bits"]
    WIDTH_32,
    #[doc = "64 bits"]
    WIDTH_64,
    #[doc = "128 bits"]
    WIDTH_128,
    #[doc = "256 bits"]
    WIDTH_256,
    #[doc = "512 bits"]
    WIDTH_512,
}
impl SRC_TR_WIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRC_TR_WIDTHW::WIDTH_8 => 0,
            SRC_TR_WIDTHW::WIDTH_16 => 1,
            SRC_TR_WIDTHW::WIDTH_32 => 2,
            SRC_TR_WIDTHW::WIDTH_64 => 3,
            SRC_TR_WIDTHW::WIDTH_128 => 4,
            SRC_TR_WIDTHW::WIDTH_256 => 5,
            SRC_TR_WIDTHW::WIDTH_512 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRC_TR_WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _SRC_TR_WIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRC_TR_WIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8 bits"]
    #[inline]
    pub fn width_8(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTHW::WIDTH_8)
    }
    #[doc = "16 bits"]
    #[inline]
    pub fn width_16(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTHW::WIDTH_16)
    }
    #[doc = "32 bits"]
    #[inline]
    pub fn width_32(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTHW::WIDTH_32)
    }
    #[doc = "64 bits"]
    #[inline]
    pub fn width_64(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTHW::WIDTH_64)
    }
    #[doc = "128 bits"]
    #[inline]
    pub fn width_128(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTHW::WIDTH_128)
    }
    #[doc = "256 bits"]
    #[inline]
    pub fn width_256(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTHW::WIDTH_256)
    }
    #[doc = "512 bits"]
    #[inline]
    pub fn width_512(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTHW::WIDTH_512)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `dst_tr_width`"]
pub type DST_TR_WIDTHW = SRC_TR_WIDTHW;
#[doc = r" Proxy"]
pub struct _DST_TR_WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _DST_TR_WIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DST_TR_WIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8 bits"]
    #[inline]
    pub fn width_8(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTHW::WIDTH_8)
    }
    #[doc = "16 bits"]
    #[inline]
    pub fn width_16(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTHW::WIDTH_16)
    }
    #[doc = "32 bits"]
    #[inline]
    pub fn width_32(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTHW::WIDTH_32)
    }
    #[doc = "64 bits"]
    #[inline]
    pub fn width_64(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTHW::WIDTH_64)
    }
    #[doc = "128 bits"]
    #[inline]
    pub fn width_128(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTHW::WIDTH_128)
    }
    #[doc = "256 bits"]
    #[inline]
    pub fn width_256(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTHW::WIDTH_256)
    }
    #[doc = "512 bits"]
    #[inline]
    pub fn width_512(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTHW::WIDTH_512)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `src_msize`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_MSIZEW {
    #[doc = "1 data item"]
    LENGTH_1,
    #[doc = "4 data items"]
    LENGTH_4,
    #[doc = "8 data items"]
    LENGTH_8,
    #[doc = "16 data items"]
    LENGTH_16,
    #[doc = "32 data items"]
    LENGTH_32,
    #[doc = "64 data items"]
    LENGTH_64,
    #[doc = "128 data items"]
    LENGTH_128,
    #[doc = "256 data items"]
    LENGTH_256,
    #[doc = "512 data items"]
    LENGTH_512,
    #[doc = "1024 data items"]
    LENGTH_1024,
}
impl SRC_MSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRC_MSIZEW::LENGTH_1 => 0,
            SRC_MSIZEW::LENGTH_4 => 1,
            SRC_MSIZEW::LENGTH_8 => 2,
            SRC_MSIZEW::LENGTH_16 => 3,
            SRC_MSIZEW::LENGTH_32 => 4,
            SRC_MSIZEW::LENGTH_64 => 5,
            SRC_MSIZEW::LENGTH_128 => 6,
            SRC_MSIZEW::LENGTH_256 => 7,
            SRC_MSIZEW::LENGTH_512 => 8,
            SRC_MSIZEW::LENGTH_1024 => 9,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRC_MSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _SRC_MSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRC_MSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 data item"]
    #[inline]
    pub fn length_1(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_1)
    }
    #[doc = "4 data items"]
    #[inline]
    pub fn length_4(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_4)
    }
    #[doc = "8 data items"]
    #[inline]
    pub fn length_8(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_8)
    }
    #[doc = "16 data items"]
    #[inline]
    pub fn length_16(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_16)
    }
    #[doc = "32 data items"]
    #[inline]
    pub fn length_32(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_32)
    }
    #[doc = "64 data items"]
    #[inline]
    pub fn length_64(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_64)
    }
    #[doc = "128 data items"]
    #[inline]
    pub fn length_128(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_128)
    }
    #[doc = "256 data items"]
    #[inline]
    pub fn length_256(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_256)
    }
    #[doc = "512 data items"]
    #[inline]
    pub fn length_512(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_512)
    }
    #[doc = "1024 data items"]
    #[inline]
    pub fn length_1024(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_1024)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `dst_msize`"]
pub type DST_MSIZEW = SRC_MSIZEW;
#[doc = r" Proxy"]
pub struct _DST_MSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _DST_MSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DST_MSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 data item"]
    #[inline]
    pub fn length_1(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_1)
    }
    #[doc = "4 data items"]
    #[inline]
    pub fn length_4(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_4)
    }
    #[doc = "8 data items"]
    #[inline]
    pub fn length_8(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_8)
    }
    #[doc = "16 data items"]
    #[inline]
    pub fn length_16(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_16)
    }
    #[doc = "32 data items"]
    #[inline]
    pub fn length_32(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_32)
    }
    #[doc = "64 data items"]
    #[inline]
    pub fn length_64(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_64)
    }
    #[doc = "128 data items"]
    #[inline]
    pub fn length_128(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_128)
    }
    #[doc = "256 data items"]
    #[inline]
    pub fn length_256(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_256)
    }
    #[doc = "512 data items"]
    #[inline]
    pub fn length_512(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_512)
    }
    #[doc = "1024 data items"]
    #[inline]
    pub fn length_1024(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_1024)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NONPOSTED_LASTWRITE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _NONPOSTED_LASTWRITE_ENW<'a> {
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
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ARLEN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ARLEN_ENW<'a> {
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
        const OFFSET: u8 = 38;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `arlen`"]
pub type ARLENW = SRC_MSIZEW;
#[doc = r" Proxy"]
pub struct _ARLENW<'a> {
    w: &'a mut W,
}
impl<'a> _ARLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARLENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 data item"]
    #[inline]
    pub fn length_1(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_1)
    }
    #[doc = "4 data items"]
    #[inline]
    pub fn length_4(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_4)
    }
    #[doc = "8 data items"]
    #[inline]
    pub fn length_8(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_8)
    }
    #[doc = "16 data items"]
    #[inline]
    pub fn length_16(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_16)
    }
    #[doc = "32 data items"]
    #[inline]
    pub fn length_32(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_32)
    }
    #[doc = "64 data items"]
    #[inline]
    pub fn length_64(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_64)
    }
    #[doc = "128 data items"]
    #[inline]
    pub fn length_128(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_128)
    }
    #[doc = "256 data items"]
    #[inline]
    pub fn length_256(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_256)
    }
    #[doc = "512 data items"]
    #[inline]
    pub fn length_512(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_512)
    }
    #[doc = "1024 data items"]
    #[inline]
    pub fn length_1024(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_1024)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 39;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AWLEN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AWLEN_ENW<'a> {
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
        const OFFSET: u8 = 47;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `awlen`"]
pub type AWLENW = SRC_MSIZEW;
#[doc = r" Proxy"]
pub struct _AWLENW<'a> {
    w: &'a mut W,
}
impl<'a> _AWLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AWLENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 data item"]
    #[inline]
    pub fn length_1(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_1)
    }
    #[doc = "4 data items"]
    #[inline]
    pub fn length_4(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_4)
    }
    #[doc = "8 data items"]
    #[inline]
    pub fn length_8(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_8)
    }
    #[doc = "16 data items"]
    #[inline]
    pub fn length_16(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_16)
    }
    #[doc = "32 data items"]
    #[inline]
    pub fn length_32(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_32)
    }
    #[doc = "64 data items"]
    #[inline]
    pub fn length_64(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_64)
    }
    #[doc = "128 data items"]
    #[inline]
    pub fn length_128(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_128)
    }
    #[doc = "256 data items"]
    #[inline]
    pub fn length_256(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_256)
    }
    #[doc = "512 data items"]
    #[inline]
    pub fn length_512(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_512)
    }
    #[doc = "1024 data items"]
    #[inline]
    pub fn length_1024(self) -> &'a mut W {
        self.variant(SRC_MSIZEW::LENGTH_1024)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 48;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SRC_STAT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SRC_STAT_ENW<'a> {
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
        const OFFSET: u8 = 56;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DST_STAT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DST_STAT_ENW<'a> {
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
        const OFFSET: u8 = 57;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IOC_BLKTFRW<'a> {
    w: &'a mut W,
}
impl<'a> _IOC_BLKTFRW<'a> {
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
        const OFFSET: u8 = 58;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SHADOWREG_OR_LLI_LASTW<'a> {
    w: &'a mut W,
}
impl<'a> _SHADOWREG_OR_LLI_LASTW<'a> {
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
        const OFFSET: u8 = 62;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SHADOWREG_OR_LLI_VALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _SHADOWREG_OR_LLI_VALIDW<'a> {
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
        const OFFSET: u8 = 63;
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
    #[doc = "Bit 0 - Source master select"]
    #[inline]
    pub fn sms(&self) -> SMSR {
        SMSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        })
    }
    #[doc = "Bit 2 - Destination master select"]
    #[inline]
    pub fn dms(&self) -> DMSR {
        DMSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        })
    }
    #[doc = "Bit 4 - Source address increment"]
    #[inline]
    pub fn sinc(&self) -> SINCR {
        SINCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        })
    }
    #[doc = "Bit 6 - Destination address increment"]
    #[inline]
    pub fn dinc(&self) -> DINCR {
        DINCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        })
    }
    #[doc = "Bits 8:10 - Source transfer width"]
    #[inline]
    pub fn src_tr_width(&self) -> SRC_TR_WIDTHR {
        SRC_TR_WIDTHR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u64) as u8
        })
    }
    #[doc = "Bits 11:13 - Destination transfer width"]
    #[inline]
    pub fn dst_tr_width(&self) -> DST_TR_WIDTHR {
        DST_TR_WIDTHR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u64) as u8
        })
    }
    #[doc = "Bits 14:17 - Source burst transaction length"]
    #[inline]
    pub fn src_msize(&self) -> SRC_MSIZER {
        SRC_MSIZER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u64) as u8
        })
    }
    #[doc = "Bits 18:21 - Destination burst transaction length"]
    #[inline]
    pub fn dst_msize(&self) -> DST_MSIZER {
        DST_MSIZER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u64) as u8
        })
    }
    #[doc = "Bit 30 - Non Posted Last Write Enable (posted writes may be used till the end of the block)"]
    #[inline]
    pub fn nonposted_lastwrite_en(&self) -> NONPOSTED_LASTWRITE_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        NONPOSTED_LASTWRITE_ENR { bits }
    }
    #[doc = "Bit 38 - Source burst length enable"]
    #[inline]
    pub fn arlen_en(&self) -> ARLEN_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 38;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        ARLEN_ENR { bits }
    }
    #[doc = "Bits 39:46 - Source burst length"]
    #[inline]
    pub fn arlen(&self) -> ARLENR {
        ARLENR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 39;
            ((self.bits >> OFFSET) & MASK as u64) as u8
        })
    }
    #[doc = "Bit 47 - Destination burst length enable"]
    #[inline]
    pub fn awlen_en(&self) -> AWLEN_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 47;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        AWLEN_ENR { bits }
    }
    #[doc = "Bits 48:55 - Destination burst length"]
    #[inline]
    pub fn awlen(&self) -> AWLENR {
        AWLENR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 48;
            ((self.bits >> OFFSET) & MASK as u64) as u8
        })
    }
    #[doc = "Bit 56 - Source status enable"]
    #[inline]
    pub fn src_stat_en(&self) -> SRC_STAT_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 56;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        SRC_STAT_ENR { bits }
    }
    #[doc = "Bit 57 - Destination status enable"]
    #[inline]
    pub fn dst_stat_en(&self) -> DST_STAT_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 57;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        DST_STAT_ENR { bits }
    }
    #[doc = "Bit 58 - Interrupt completion of block transfer"]
    #[inline]
    pub fn ioc_blktfr(&self) -> IOC_BLKTFRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 58;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        IOC_BLKTFRR { bits }
    }
    #[doc = "Bit 62 - Last shadow linked list item (indicates shadowreg/LLI content is the last one)"]
    #[inline]
    pub fn shadowreg_or_lli_last(&self) -> SHADOWREG_OR_LLI_LASTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 62;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        SHADOWREG_OR_LLI_LASTR { bits }
    }
    #[doc = "Bit 63 - last shadow linked list item valid (indicate shadowreg/LLI content is valid)"]
    #[inline]
    pub fn shadowreg_or_lli_valid(&self) -> SHADOWREG_OR_LLI_VALIDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 63;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        SHADOWREG_OR_LLI_VALIDR { bits }
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
    #[doc = "Bit 0 - Source master select"]
    #[inline]
    pub fn sms(&mut self) -> _SMSW {
        _SMSW { w: self }
    }
    #[doc = "Bit 2 - Destination master select"]
    #[inline]
    pub fn dms(&mut self) -> _DMSW {
        _DMSW { w: self }
    }
    #[doc = "Bit 4 - Source address increment"]
    #[inline]
    pub fn sinc(&mut self) -> _SINCW {
        _SINCW { w: self }
    }
    #[doc = "Bit 6 - Destination address increment"]
    #[inline]
    pub fn dinc(&mut self) -> _DINCW {
        _DINCW { w: self }
    }
    #[doc = "Bits 8:10 - Source transfer width"]
    #[inline]
    pub fn src_tr_width(&mut self) -> _SRC_TR_WIDTHW {
        _SRC_TR_WIDTHW { w: self }
    }
    #[doc = "Bits 11:13 - Destination transfer width"]
    #[inline]
    pub fn dst_tr_width(&mut self) -> _DST_TR_WIDTHW {
        _DST_TR_WIDTHW { w: self }
    }
    #[doc = "Bits 14:17 - Source burst transaction length"]
    #[inline]
    pub fn src_msize(&mut self) -> _SRC_MSIZEW {
        _SRC_MSIZEW { w: self }
    }
    #[doc = "Bits 18:21 - Destination burst transaction length"]
    #[inline]
    pub fn dst_msize(&mut self) -> _DST_MSIZEW {
        _DST_MSIZEW { w: self }
    }
    #[doc = "Bit 30 - Non Posted Last Write Enable (posted writes may be used till the end of the block)"]
    #[inline]
    pub fn nonposted_lastwrite_en(&mut self) -> _NONPOSTED_LASTWRITE_ENW {
        _NONPOSTED_LASTWRITE_ENW { w: self }
    }
    #[doc = "Bit 38 - Source burst length enable"]
    #[inline]
    pub fn arlen_en(&mut self) -> _ARLEN_ENW {
        _ARLEN_ENW { w: self }
    }
    #[doc = "Bits 39:46 - Source burst length"]
    #[inline]
    pub fn arlen(&mut self) -> _ARLENW {
        _ARLENW { w: self }
    }
    #[doc = "Bit 47 - Destination burst length enable"]
    #[inline]
    pub fn awlen_en(&mut self) -> _AWLEN_ENW {
        _AWLEN_ENW { w: self }
    }
    #[doc = "Bits 48:55 - Destination burst length"]
    #[inline]
    pub fn awlen(&mut self) -> _AWLENW {
        _AWLENW { w: self }
    }
    #[doc = "Bit 56 - Source status enable"]
    #[inline]
    pub fn src_stat_en(&mut self) -> _SRC_STAT_ENW {
        _SRC_STAT_ENW { w: self }
    }
    #[doc = "Bit 57 - Destination status enable"]
    #[inline]
    pub fn dst_stat_en(&mut self) -> _DST_STAT_ENW {
        _DST_STAT_ENW { w: self }
    }
    #[doc = "Bit 58 - Interrupt completion of block transfer"]
    #[inline]
    pub fn ioc_blktfr(&mut self) -> _IOC_BLKTFRW {
        _IOC_BLKTFRW { w: self }
    }
    #[doc = "Bit 62 - Last shadow linked list item (indicates shadowreg/LLI content is the last one)"]
    #[inline]
    pub fn shadowreg_or_lli_last(&mut self) -> _SHADOWREG_OR_LLI_LASTW {
        _SHADOWREG_OR_LLI_LASTW { w: self }
    }
    #[doc = "Bit 63 - last shadow linked list item valid (indicate shadowreg/LLI content is valid)"]
    #[inline]
    pub fn shadowreg_or_lli_valid(&mut self) -> _SHADOWREG_OR_LLI_VALIDW {
        _SHADOWREG_OR_LLI_VALIDW { w: self }
    }
}
