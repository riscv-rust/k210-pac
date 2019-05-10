#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DATA_IN_FLAG {
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
#[doc = "Possible values of the field `data_in_flag`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_IN_FLAGR {
    #[doc = "Cannot input"]
    CANNOT_INPUT,
    #[doc = "Can input"]
    CAN_INPUT,
}
impl DATA_IN_FLAGR {
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
            DATA_IN_FLAGR::CANNOT_INPUT => false,
            DATA_IN_FLAGR::CAN_INPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA_IN_FLAGR {
        match value {
            false => DATA_IN_FLAGR::CANNOT_INPUT,
            true => DATA_IN_FLAGR::CAN_INPUT,
        }
    }
    #[doc = "Checks if the value of the field is `CANNOT_INPUT`"]
    #[inline]
    pub fn is_cannot_input(&self) -> bool {
        *self == DATA_IN_FLAGR::CANNOT_INPUT
    }
    #[doc = "Checks if the value of the field is `CAN_INPUT`"]
    #[inline]
    pub fn is_can_input(&self) -> bool {
        *self == DATA_IN_FLAGR::CAN_INPUT
    }
}
#[doc = "Values that can be written to the field `data_in_flag`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_IN_FLAGW {
    #[doc = "Cannot input"]
    CANNOT_INPUT,
    #[doc = "Can input"]
    CAN_INPUT,
}
impl DATA_IN_FLAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA_IN_FLAGW::CANNOT_INPUT => false,
            DATA_IN_FLAGW::CAN_INPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_IN_FLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_IN_FLAGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_IN_FLAGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Cannot input"]
    #[inline]
    pub fn cannot_input(self) -> &'a mut W {
        self.variant(DATA_IN_FLAGW::CANNOT_INPUT)
    }
    #[doc = "Can input"]
    #[inline]
    pub fn can_input(self) -> &'a mut W {
        self.variant(DATA_IN_FLAGW::CAN_INPUT)
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
    #[doc = "Bit 0 - Data can be written to text_data or aad_data when this flag is set"]
    #[inline]
    pub fn data_in_flag(&self) -> DATA_IN_FLAGR {
        DATA_IN_FLAGR::_from({
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
    #[doc = "Bit 0 - Data can be written to text_data or aad_data when this flag is set"]
    #[inline]
    pub fn data_in_flag(&mut self) -> _DATA_IN_FLAGW {
        _DATA_IN_FLAGW { w: self }
    }
}
