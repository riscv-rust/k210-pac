#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DATA_OUT_FLAG {
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
#[doc = "Possible values of the field `data_out_flag`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_OUT_FLAGR {
    #[doc = "Data cannot output"]
    CANNOT_OUTPUT,
    #[doc = "Data can output"]
    CAN_OUTPUT,
}
impl DATA_OUT_FLAGR {
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
            DATA_OUT_FLAGR::CANNOT_OUTPUT => false,
            DATA_OUT_FLAGR::CAN_OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA_OUT_FLAGR {
        match value {
            false => DATA_OUT_FLAGR::CANNOT_OUTPUT,
            true => DATA_OUT_FLAGR::CAN_OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `CANNOT_OUTPUT`"]
    #[inline]
    pub fn is_cannot_output(&self) -> bool {
        *self == DATA_OUT_FLAGR::CANNOT_OUTPUT
    }
    #[doc = "Checks if the value of the field is `CAN_OUTPUT`"]
    #[inline]
    pub fn is_can_output(&self) -> bool {
        *self == DATA_OUT_FLAGR::CAN_OUTPUT
    }
}
#[doc = "Values that can be written to the field `data_out_flag`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_OUT_FLAGW {
    #[doc = "Data cannot output"]
    CANNOT_OUTPUT,
    #[doc = "Data can output"]
    CAN_OUTPUT,
}
impl DATA_OUT_FLAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA_OUT_FLAGW::CANNOT_OUTPUT => false,
            DATA_OUT_FLAGW::CAN_OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_OUT_FLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_OUT_FLAGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_OUT_FLAGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data cannot output"]
    #[inline]
    pub fn cannot_output(self) -> &'a mut W {
        self.variant(DATA_OUT_FLAGW::CANNOT_OUTPUT)
    }
    #[doc = "Data can output"]
    #[inline]
    pub fn can_output(self) -> &'a mut W {
        self.variant(DATA_OUT_FLAGW::CAN_OUTPUT)
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
    #[doc = "Bit 0 - Data can be read from out_data when this flag is set"]
    #[inline]
    pub fn data_out_flag(&self) -> DATA_OUT_FLAGR {
        DATA_OUT_FLAGR::_from({
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
    #[doc = "Bit 0 - Data can be read from out_data when this flag is set"]
    #[inline]
    pub fn data_out_flag(&mut self) -> _DATA_OUT_FLAGW {
        _DATA_OUT_FLAGW { w: self }
    }
}
