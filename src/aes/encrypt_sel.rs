#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ENCRYPT_SEL {
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
#[doc = "Possible values of the field `encrypt_sel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENCRYPT_SELR {
    #[doc = "Sets encryption mode"]
    ENCRYPTION,
    #[doc = "Sets decryption mode"]
    DECRYPTION,
}
impl ENCRYPT_SELR {
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
            ENCRYPT_SELR::ENCRYPTION => false,
            ENCRYPT_SELR::DECRYPTION => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENCRYPT_SELR {
        match value {
            false => ENCRYPT_SELR::ENCRYPTION,
            true => ENCRYPT_SELR::DECRYPTION,
        }
    }
    #[doc = "Checks if the value of the field is `ENCRYPTION`"]
    #[inline]
    pub fn is_encryption(&self) -> bool {
        *self == ENCRYPT_SELR::ENCRYPTION
    }
    #[doc = "Checks if the value of the field is `DECRYPTION`"]
    #[inline]
    pub fn is_decryption(&self) -> bool {
        *self == ENCRYPT_SELR::DECRYPTION
    }
}
#[doc = "Values that can be written to the field `encrypt_sel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENCRYPT_SELW {
    #[doc = "Sets encryption mode"]
    ENCRYPTION,
    #[doc = "Sets decryption mode"]
    DECRYPTION,
}
impl ENCRYPT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENCRYPT_SELW::ENCRYPTION => false,
            ENCRYPT_SELW::DECRYPTION => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENCRYPT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _ENCRYPT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENCRYPT_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Sets encryption mode"]
    #[inline]
    pub fn encryption(self) -> &'a mut W {
        self.variant(ENCRYPT_SELW::ENCRYPTION)
    }
    #[doc = "Sets decryption mode"]
    #[inline]
    pub fn decryption(self) -> &'a mut W {
        self.variant(ENCRYPT_SELW::DECRYPTION)
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
    #[doc = "Bit 0 - Select encryption or decryption mode"]
    #[inline]
    pub fn encrypt_sel(&self) -> ENCRYPT_SELR {
        ENCRYPT_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 0 - Select encryption or decryption mode"]
    #[inline]
    pub fn encrypt_sel(&mut self) -> _ENCRYPT_SELW {
        _ENCRYPT_SELW { w: self }
    }
}
