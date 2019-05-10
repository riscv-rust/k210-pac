#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODE_CTL {
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
#[doc = "Possible values of the field `cipher_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIPHER_MODER {
    #[doc = "Electronic Codebook"]
    ECB,
    #[doc = "Cipher Block Chaining"]
    CBC,
    #[doc = "Galois/Counter Mode"]
    GCM,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CIPHER_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CIPHER_MODER::ECB => 0,
            CIPHER_MODER::CBC => 1,
            CIPHER_MODER::GCM => 2,
            CIPHER_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CIPHER_MODER {
        match value {
            0 => CIPHER_MODER::ECB,
            1 => CIPHER_MODER::CBC,
            2 => CIPHER_MODER::GCM,
            i => CIPHER_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ECB`"]
    #[inline]
    pub fn is_ecb(&self) -> bool {
        *self == CIPHER_MODER::ECB
    }
    #[doc = "Checks if the value of the field is `CBC`"]
    #[inline]
    pub fn is_cbc(&self) -> bool {
        *self == CIPHER_MODER::CBC
    }
    #[doc = "Checks if the value of the field is `GCM`"]
    #[inline]
    pub fn is_gcm(&self) -> bool {
        *self == CIPHER_MODER::GCM
    }
}
#[doc = "Possible values of the field `key_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEY_MODER {
    #[doc = "AES-128"]
    AES128,
    #[doc = "AES-192"]
    AES192,
    #[doc = "AES-256"]
    AES256,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl KEY_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            KEY_MODER::AES128 => 0,
            KEY_MODER::AES192 => 1,
            KEY_MODER::AES256 => 2,
            KEY_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> KEY_MODER {
        match value {
            0 => KEY_MODER::AES128,
            1 => KEY_MODER::AES192,
            2 => KEY_MODER::AES256,
            i => KEY_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AES128`"]
    #[inline]
    pub fn is_aes128(&self) -> bool {
        *self == KEY_MODER::AES128
    }
    #[doc = "Checks if the value of the field is `AES192`"]
    #[inline]
    pub fn is_aes192(&self) -> bool {
        *self == KEY_MODER::AES192
    }
    #[doc = "Checks if the value of the field is `AES256`"]
    #[inline]
    pub fn is_aes256(&self) -> bool {
        *self == KEY_MODER::AES256
    }
}
#[doc = "Possible values of the field `key_order`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEY_ORDERR {
    #[doc = "Big Endian"]
    BE,
    #[doc = "Little Endian"]
    LE,
}
impl KEY_ORDERR {
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
            KEY_ORDERR::BE => false,
            KEY_ORDERR::LE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KEY_ORDERR {
        match value {
            false => KEY_ORDERR::BE,
            true => KEY_ORDERR::LE,
        }
    }
    #[doc = "Checks if the value of the field is `BE`"]
    #[inline]
    pub fn is_be(&self) -> bool {
        *self == KEY_ORDERR::BE
    }
    #[doc = "Checks if the value of the field is `LE`"]
    #[inline]
    pub fn is_le(&self) -> bool {
        *self == KEY_ORDERR::LE
    }
}
#[doc = "Possible values of the field `input_order`"]
pub type INPUT_ORDERR = KEY_ORDERR;
#[doc = "Possible values of the field `output_order`"]
pub type OUTPUT_ORDERR = KEY_ORDERR;
#[doc = "Values that can be written to the field `cipher_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIPHER_MODEW {
    #[doc = "Electronic Codebook"]
    ECB,
    #[doc = "Cipher Block Chaining"]
    CBC,
    #[doc = "Galois/Counter Mode"]
    GCM,
}
impl CIPHER_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CIPHER_MODEW::ECB => 0,
            CIPHER_MODEW::CBC => 1,
            CIPHER_MODEW::GCM => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CIPHER_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CIPHER_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CIPHER_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Electronic Codebook"]
    #[inline]
    pub fn ecb(self) -> &'a mut W {
        self.variant(CIPHER_MODEW::ECB)
    }
    #[doc = "Cipher Block Chaining"]
    #[inline]
    pub fn cbc(self) -> &'a mut W {
        self.variant(CIPHER_MODEW::CBC)
    }
    #[doc = "Galois/Counter Mode"]
    #[inline]
    pub fn gcm(self) -> &'a mut W {
        self.variant(CIPHER_MODEW::GCM)
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
#[doc = "Values that can be written to the field `key_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEY_MODEW {
    #[doc = "AES-128"]
    AES128,
    #[doc = "AES-192"]
    AES192,
    #[doc = "AES-256"]
    AES256,
}
impl KEY_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            KEY_MODEW::AES128 => 0,
            KEY_MODEW::AES192 => 1,
            KEY_MODEW::AES256 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KEY_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _KEY_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KEY_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "AES-128"]
    #[inline]
    pub fn aes128(self) -> &'a mut W {
        self.variant(KEY_MODEW::AES128)
    }
    #[doc = "AES-192"]
    #[inline]
    pub fn aes192(self) -> &'a mut W {
        self.variant(KEY_MODEW::AES192)
    }
    #[doc = "AES-256"]
    #[inline]
    pub fn aes256(self) -> &'a mut W {
        self.variant(KEY_MODEW::AES256)
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
#[doc = "Values that can be written to the field `key_order`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEY_ORDERW {
    #[doc = "Big Endian"]
    BE,
    #[doc = "Little Endian"]
    LE,
}
impl KEY_ORDERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            KEY_ORDERW::BE => false,
            KEY_ORDERW::LE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KEY_ORDERW<'a> {
    w: &'a mut W,
}
impl<'a> _KEY_ORDERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KEY_ORDERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Big Endian"]
    #[inline]
    pub fn be(self) -> &'a mut W {
        self.variant(KEY_ORDERW::BE)
    }
    #[doc = "Little Endian"]
    #[inline]
    pub fn le(self) -> &'a mut W {
        self.variant(KEY_ORDERW::LE)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `input_order`"]
pub type INPUT_ORDERW = KEY_ORDERW;
#[doc = r" Proxy"]
pub struct _INPUT_ORDERW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT_ORDERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT_ORDERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Big Endian"]
    #[inline]
    pub fn be(self) -> &'a mut W {
        self.variant(KEY_ORDERW::BE)
    }
    #[doc = "Little Endian"]
    #[inline]
    pub fn le(self) -> &'a mut W {
        self.variant(KEY_ORDERW::LE)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `output_order`"]
pub type OUTPUT_ORDERW = KEY_ORDERW;
#[doc = r" Proxy"]
pub struct _OUTPUT_ORDERW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTPUT_ORDERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTPUT_ORDERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Big Endian"]
    #[inline]
    pub fn be(self) -> &'a mut W {
        self.variant(KEY_ORDERW::BE)
    }
    #[doc = "Little Endian"]
    #[inline]
    pub fn le(self) -> &'a mut W {
        self.variant(KEY_ORDERW::LE)
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
        const OFFSET: u8 = 9;
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
    #[doc = "Bits 0:2 - Cipher mode"]
    #[inline]
    pub fn cipher_mode(&self) -> CIPHER_MODER {
        CIPHER_MODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:4 - Key mode"]
    #[inline]
    pub fn key_mode(&self) -> KEY_MODER {
        KEY_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Input key order"]
    #[inline]
    pub fn key_order(&self) -> KEY_ORDERR {
        KEY_ORDERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Input data order"]
    #[inline]
    pub fn input_order(&self) -> INPUT_ORDERR {
        INPUT_ORDERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Output data order"]
    #[inline]
    pub fn output_order(&self) -> OUTPUT_ORDERR {
        OUTPUT_ORDERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Cipher mode"]
    #[inline]
    pub fn cipher_mode(&mut self) -> _CIPHER_MODEW {
        _CIPHER_MODEW { w: self }
    }
    #[doc = "Bits 3:4 - Key mode"]
    #[inline]
    pub fn key_mode(&mut self) -> _KEY_MODEW {
        _KEY_MODEW { w: self }
    }
    #[doc = "Bit 5 - Input key order"]
    #[inline]
    pub fn key_order(&mut self) -> _KEY_ORDERW {
        _KEY_ORDERW { w: self }
    }
    #[doc = "Bit 7 - Input data order"]
    #[inline]
    pub fn input_order(&mut self) -> _INPUT_ORDERW {
        _INPUT_ORDERW { w: self }
    }
    #[doc = "Bit 9 - Output data order"]
    #[inline]
    pub fn output_order(&mut self) -> _OUTPUT_ORDERW {
        _OUTPUT_ORDERW { w: self }
    }
}
