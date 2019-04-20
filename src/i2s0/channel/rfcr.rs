#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RFCR {
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
#[doc = "Possible values of the field `rxchdt`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCHDTR {
    #[doc = "Interrupt trigger when FIFO level is 1"]
    LEVEL1,
    #[doc = "Interrupt trigger when FIFO level is 2"]
    LEVEL2,
    #[doc = "Interrupt trigger when FIFO level is 3"]
    LEVEL3,
    #[doc = "Interrupt trigger when FIFO level is 4"]
    LEVEL4,
    #[doc = "Interrupt trigger when FIFO level is 5"]
    LEVEL5,
    #[doc = "Interrupt trigger when FIFO level is 6"]
    LEVEL6,
    #[doc = "Interrupt trigger when FIFO level is 7"]
    LEVEL7,
    #[doc = "Interrupt trigger when FIFO level is 8"]
    LEVEL8,
    #[doc = "Interrupt trigger when FIFO level is 9"]
    LEVEL9,
    #[doc = "Interrupt trigger when FIFO level is 10"]
    LEVEL10,
    #[doc = "Interrupt trigger when FIFO level is 11"]
    LEVEL11,
    #[doc = "Interrupt trigger when FIFO level is 12"]
    LEVEL12,
    #[doc = "Interrupt trigger when FIFO level is 13"]
    LEVEL13,
    #[doc = "Interrupt trigger when FIFO level is 14"]
    LEVEL14,
    #[doc = "Interrupt trigger when FIFO level is 15"]
    LEVEL15,
    #[doc = "Interrupt trigger when FIFO level is 16"]
    LEVEL16,
}
impl RXCHDTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXCHDTR::LEVEL1 => 0,
            RXCHDTR::LEVEL2 => 1,
            RXCHDTR::LEVEL3 => 2,
            RXCHDTR::LEVEL4 => 3,
            RXCHDTR::LEVEL5 => 4,
            RXCHDTR::LEVEL6 => 5,
            RXCHDTR::LEVEL7 => 6,
            RXCHDTR::LEVEL8 => 7,
            RXCHDTR::LEVEL9 => 8,
            RXCHDTR::LEVEL10 => 9,
            RXCHDTR::LEVEL11 => 10,
            RXCHDTR::LEVEL12 => 11,
            RXCHDTR::LEVEL13 => 12,
            RXCHDTR::LEVEL14 => 13,
            RXCHDTR::LEVEL15 => 14,
            RXCHDTR::LEVEL16 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXCHDTR {
        match value {
            0 => RXCHDTR::LEVEL1,
            1 => RXCHDTR::LEVEL2,
            2 => RXCHDTR::LEVEL3,
            3 => RXCHDTR::LEVEL4,
            4 => RXCHDTR::LEVEL5,
            5 => RXCHDTR::LEVEL6,
            6 => RXCHDTR::LEVEL7,
            7 => RXCHDTR::LEVEL8,
            8 => RXCHDTR::LEVEL9,
            9 => RXCHDTR::LEVEL10,
            10 => RXCHDTR::LEVEL11,
            11 => RXCHDTR::LEVEL12,
            12 => RXCHDTR::LEVEL13,
            13 => RXCHDTR::LEVEL14,
            14 => RXCHDTR::LEVEL15,
            15 => RXCHDTR::LEVEL16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline]
    pub fn is_level1(&self) -> bool {
        *self == RXCHDTR::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline]
    pub fn is_level2(&self) -> bool {
        *self == RXCHDTR::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL3`"]
    #[inline]
    pub fn is_level3(&self) -> bool {
        *self == RXCHDTR::LEVEL3
    }
    #[doc = "Checks if the value of the field is `LEVEL4`"]
    #[inline]
    pub fn is_level4(&self) -> bool {
        *self == RXCHDTR::LEVEL4
    }
    #[doc = "Checks if the value of the field is `LEVEL5`"]
    #[inline]
    pub fn is_level5(&self) -> bool {
        *self == RXCHDTR::LEVEL5
    }
    #[doc = "Checks if the value of the field is `LEVEL6`"]
    #[inline]
    pub fn is_level6(&self) -> bool {
        *self == RXCHDTR::LEVEL6
    }
    #[doc = "Checks if the value of the field is `LEVEL7`"]
    #[inline]
    pub fn is_level7(&self) -> bool {
        *self == RXCHDTR::LEVEL7
    }
    #[doc = "Checks if the value of the field is `LEVEL8`"]
    #[inline]
    pub fn is_level8(&self) -> bool {
        *self == RXCHDTR::LEVEL8
    }
    #[doc = "Checks if the value of the field is `LEVEL9`"]
    #[inline]
    pub fn is_level9(&self) -> bool {
        *self == RXCHDTR::LEVEL9
    }
    #[doc = "Checks if the value of the field is `LEVEL10`"]
    #[inline]
    pub fn is_level10(&self) -> bool {
        *self == RXCHDTR::LEVEL10
    }
    #[doc = "Checks if the value of the field is `LEVEL11`"]
    #[inline]
    pub fn is_level11(&self) -> bool {
        *self == RXCHDTR::LEVEL11
    }
    #[doc = "Checks if the value of the field is `LEVEL12`"]
    #[inline]
    pub fn is_level12(&self) -> bool {
        *self == RXCHDTR::LEVEL12
    }
    #[doc = "Checks if the value of the field is `LEVEL13`"]
    #[inline]
    pub fn is_level13(&self) -> bool {
        *self == RXCHDTR::LEVEL13
    }
    #[doc = "Checks if the value of the field is `LEVEL14`"]
    #[inline]
    pub fn is_level14(&self) -> bool {
        *self == RXCHDTR::LEVEL14
    }
    #[doc = "Checks if the value of the field is `LEVEL15`"]
    #[inline]
    pub fn is_level15(&self) -> bool {
        *self == RXCHDTR::LEVEL15
    }
    #[doc = "Checks if the value of the field is `LEVEL16`"]
    #[inline]
    pub fn is_level16(&self) -> bool {
        *self == RXCHDTR::LEVEL16
    }
}
#[doc = "Values that can be written to the field `rxchdt`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCHDTW {
    #[doc = "Interrupt trigger when FIFO level is 1"]
    LEVEL1,
    #[doc = "Interrupt trigger when FIFO level is 2"]
    LEVEL2,
    #[doc = "Interrupt trigger when FIFO level is 3"]
    LEVEL3,
    #[doc = "Interrupt trigger when FIFO level is 4"]
    LEVEL4,
    #[doc = "Interrupt trigger when FIFO level is 5"]
    LEVEL5,
    #[doc = "Interrupt trigger when FIFO level is 6"]
    LEVEL6,
    #[doc = "Interrupt trigger when FIFO level is 7"]
    LEVEL7,
    #[doc = "Interrupt trigger when FIFO level is 8"]
    LEVEL8,
    #[doc = "Interrupt trigger when FIFO level is 9"]
    LEVEL9,
    #[doc = "Interrupt trigger when FIFO level is 10"]
    LEVEL10,
    #[doc = "Interrupt trigger when FIFO level is 11"]
    LEVEL11,
    #[doc = "Interrupt trigger when FIFO level is 12"]
    LEVEL12,
    #[doc = "Interrupt trigger when FIFO level is 13"]
    LEVEL13,
    #[doc = "Interrupt trigger when FIFO level is 14"]
    LEVEL14,
    #[doc = "Interrupt trigger when FIFO level is 15"]
    LEVEL15,
    #[doc = "Interrupt trigger when FIFO level is 16"]
    LEVEL16,
}
impl RXCHDTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXCHDTW::LEVEL1 => 0,
            RXCHDTW::LEVEL2 => 1,
            RXCHDTW::LEVEL3 => 2,
            RXCHDTW::LEVEL4 => 3,
            RXCHDTW::LEVEL5 => 4,
            RXCHDTW::LEVEL6 => 5,
            RXCHDTW::LEVEL7 => 6,
            RXCHDTW::LEVEL8 => 7,
            RXCHDTW::LEVEL9 => 8,
            RXCHDTW::LEVEL10 => 9,
            RXCHDTW::LEVEL11 => 10,
            RXCHDTW::LEVEL12 => 11,
            RXCHDTW::LEVEL13 => 12,
            RXCHDTW::LEVEL14 => 13,
            RXCHDTW::LEVEL15 => 14,
            RXCHDTW::LEVEL16 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCHDTW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCHDTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCHDTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt trigger when FIFO level is 1"]
    #[inline]
    pub fn level1(self) -> &'a mut W {
        self.variant(RXCHDTW::LEVEL1)
    }
    #[doc = "Interrupt trigger when FIFO level is 2"]
    #[inline]
    pub fn level2(self) -> &'a mut W {
        self.variant(RXCHDTW::LEVEL2)
    }
    #[doc = "Interrupt trigger when FIFO level is 3"]
    #[inline]
    pub fn level3(self) -> &'a mut W {
        self.variant(RXCHDTW::LEVEL3)
    }
    #[doc = "Interrupt trigger when FIFO level is 4"]
    #[inline]
    pub fn level4(self) -> &'a mut W {
        self.variant(RXCHDTW::LEVEL4)
    }
    #[doc = "Interrupt trigger when FIFO level is 5"]
    #[inline]
    pub fn level5(self) -> &'a mut W {
        self.variant(RXCHDTW::LEVEL5)
    }
    #[doc = "Interrupt trigger when FIFO level is 6"]
    #[inline]
    pub fn level6(self) -> &'a mut W {
        self.variant(RXCHDTW::LEVEL6)
    }
    #[doc = "Interrupt trigger when FIFO level is 7"]
    #[inline]
    pub fn level7(self) -> &'a mut W {
        self.variant(RXCHDTW::LEVEL7)
    }
    #[doc = "Interrupt trigger when FIFO level is 8"]
    #[inline]
    pub fn level8(self) -> &'a mut W {
        self.variant(RXCHDTW::LEVEL8)
    }
    #[doc = "Interrupt trigger when FIFO level is 9"]
    #[inline]
    pub fn level9(self) -> &'a mut W {
        self.variant(RXCHDTW::LEVEL9)
    }
    #[doc = "Interrupt trigger when FIFO level is 10"]
    #[inline]
    pub fn level10(self) -> &'a mut W {
        self.variant(RXCHDTW::LEVEL10)
    }
    #[doc = "Interrupt trigger when FIFO level is 11"]
    #[inline]
    pub fn level11(self) -> &'a mut W {
        self.variant(RXCHDTW::LEVEL11)
    }
    #[doc = "Interrupt trigger when FIFO level is 12"]
    #[inline]
    pub fn level12(self) -> &'a mut W {
        self.variant(RXCHDTW::LEVEL12)
    }
    #[doc = "Interrupt trigger when FIFO level is 13"]
    #[inline]
    pub fn level13(self) -> &'a mut W {
        self.variant(RXCHDTW::LEVEL13)
    }
    #[doc = "Interrupt trigger when FIFO level is 14"]
    #[inline]
    pub fn level14(self) -> &'a mut W {
        self.variant(RXCHDTW::LEVEL14)
    }
    #[doc = "Interrupt trigger when FIFO level is 15"]
    #[inline]
    pub fn level15(self) -> &'a mut W {
        self.variant(RXCHDTW::LEVEL15)
    }
    #[doc = "Interrupt trigger when FIFO level is 16"]
    #[inline]
    pub fn level16(self) -> &'a mut W {
        self.variant(RXCHDTW::LEVEL16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Trigger level in the RX FIFO at which the receiver data available interrupt generate"]
    #[inline]
    pub fn rxchdt(&self) -> RXCHDTR {
        RXCHDTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:3 - Trigger level in the RX FIFO at which the receiver data available interrupt generate"]
    #[inline]
    pub fn rxchdt(&mut self) -> _RXCHDTW {
        _RXCHDTW { w: self }
    }
}
