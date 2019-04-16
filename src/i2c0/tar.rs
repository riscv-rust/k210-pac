#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TAR {
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
pub struct ADDRESSR {
    bits: u16,
}
impl ADDRESSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GCR {
    bits: bool,
}
impl GCR {
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
pub struct SPECIALR {
    bits: bool,
}
impl SPECIALR {
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
#[doc = "Possible values of the field `addr_master_width`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR_MASTER_WIDTHR {
    #[doc = "7-bit address"]
    B7,
    #[doc = "10-bit address"]
    B10,
}
impl ADDR_MASTER_WIDTHR {
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
            ADDR_MASTER_WIDTHR::B7 => false,
            ADDR_MASTER_WIDTHR::B10 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADDR_MASTER_WIDTHR {
        match value {
            false => ADDR_MASTER_WIDTHR::B7,
            true => ADDR_MASTER_WIDTHR::B10,
        }
    }
    #[doc = "Checks if the value of the field is `B7`"]
    #[inline]
    pub fn is_b7(&self) -> bool {
        *self == ADDR_MASTER_WIDTHR::B7
    }
    #[doc = "Checks if the value of the field is `B10`"]
    #[inline]
    pub fn is_b10(&self) -> bool {
        *self == ADDR_MASTER_WIDTHR::B10
    }
}
#[doc = r" Proxy"]
pub struct _ADDRESSW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRESSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GCW<'a> {
    w: &'a mut W,
}
impl<'a> _GCW<'a> {
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
pub struct _SPECIALW<'a> {
    w: &'a mut W,
}
impl<'a> _SPECIALW<'a> {
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
#[doc = "Values that can be written to the field `addr_master_width`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR_MASTER_WIDTHW {
    #[doc = "7-bit address"]
    B7,
    #[doc = "10-bit address"]
    B10,
}
impl ADDR_MASTER_WIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADDR_MASTER_WIDTHW::B7 => false,
            ADDR_MASTER_WIDTHW::B10 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDR_MASTER_WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDR_MASTER_WIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDR_MASTER_WIDTHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "7-bit address"]
    #[inline]
    pub fn b7(self) -> &'a mut W {
        self.variant(ADDR_MASTER_WIDTHW::B7)
    }
    #[doc = "10-bit address"]
    #[inline]
    pub fn b10(self) -> &'a mut W {
        self.variant(ADDR_MASTER_WIDTHW::B10)
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
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:9 - Target Address"]
    #[inline]
    pub fn address(&self) -> ADDRESSR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        ADDRESSR { bits }
    }
    #[doc = "Bit 10 - GC_OR_START"]
    #[inline]
    pub fn gc(&self) -> GCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GCR { bits }
    }
    #[doc = "Bit 11 - SPECIAL"]
    #[inline]
    pub fn special(&self) -> SPECIALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SPECIALR { bits }
    }
    #[doc = "Bit 12 - Master Address"]
    #[inline]
    pub fn addr_master_width(&self) -> ADDR_MASTER_WIDTHR {
        ADDR_MASTER_WIDTHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:9 - Target Address"]
    #[inline]
    pub fn address(&mut self) -> _ADDRESSW {
        _ADDRESSW { w: self }
    }
    #[doc = "Bit 10 - GC_OR_START"]
    #[inline]
    pub fn gc(&mut self) -> _GCW {
        _GCW { w: self }
    }
    #[doc = "Bit 11 - SPECIAL"]
    #[inline]
    pub fn special(&mut self) -> _SPECIALW {
        _SPECIALW { w: self }
    }
    #[doc = "Bit 12 - Master Address"]
    #[inline]
    pub fn addr_master_width(&mut self) -> _ADDR_MASTER_WIDTHW {
        _ADDR_MASTER_WIDTHW { w: self }
    }
}
