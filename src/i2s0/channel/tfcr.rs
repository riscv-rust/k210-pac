#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TFCR {
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
#[doc = "Possible values of the field `txchet`"]
pub type TXCHETR = super::rfcr::RXCHDTR;
#[doc = "Values that can be written to the field `txchet`"]
pub type TXCHETW = super::rfcr::RXCHDTW;
#[doc = r" Proxy"]
pub struct _TXCHETW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCHETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXCHETW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Interrupt trigger when FIFO level is 1"]
    #[inline]
    pub fn level1(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDTW::LEVEL1)
    }
    #[doc = "Interrupt trigger when FIFO level is 2"]
    #[inline]
    pub fn level2(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDTW::LEVEL2)
    }
    #[doc = "Interrupt trigger when FIFO level is 3"]
    #[inline]
    pub fn level3(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDTW::LEVEL3)
    }
    #[doc = "Interrupt trigger when FIFO level is 4"]
    #[inline]
    pub fn level4(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDTW::LEVEL4)
    }
    #[doc = "Interrupt trigger when FIFO level is 5"]
    #[inline]
    pub fn level5(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDTW::LEVEL5)
    }
    #[doc = "Interrupt trigger when FIFO level is 6"]
    #[inline]
    pub fn level6(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDTW::LEVEL6)
    }
    #[doc = "Interrupt trigger when FIFO level is 7"]
    #[inline]
    pub fn level7(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDTW::LEVEL7)
    }
    #[doc = "Interrupt trigger when FIFO level is 8"]
    #[inline]
    pub fn level8(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDTW::LEVEL8)
    }
    #[doc = "Interrupt trigger when FIFO level is 9"]
    #[inline]
    pub fn level9(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDTW::LEVEL9)
    }
    #[doc = "Interrupt trigger when FIFO level is 10"]
    #[inline]
    pub fn level10(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDTW::LEVEL10)
    }
    #[doc = "Interrupt trigger when FIFO level is 11"]
    #[inline]
    pub fn level11(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDTW::LEVEL11)
    }
    #[doc = "Interrupt trigger when FIFO level is 12"]
    #[inline]
    pub fn level12(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDTW::LEVEL12)
    }
    #[doc = "Interrupt trigger when FIFO level is 13"]
    #[inline]
    pub fn level13(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDTW::LEVEL13)
    }
    #[doc = "Interrupt trigger when FIFO level is 14"]
    #[inline]
    pub fn level14(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDTW::LEVEL14)
    }
    #[doc = "Interrupt trigger when FIFO level is 15"]
    #[inline]
    pub fn level15(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDTW::LEVEL15)
    }
    #[doc = "Interrupt trigger when FIFO level is 16"]
    #[inline]
    pub fn level16(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDTW::LEVEL16)
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
    #[doc = "Bits 0:3 - Trigger level in the TX FIFO at which the transmitter data available interrupt generate"]
    #[inline]
    pub fn txchet(&self) -> TXCHETR {
        TXCHETR::_from({
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
    #[doc = "Bits 0:3 - Trigger level in the TX FIFO at which the transmitter data available interrupt generate"]
    #[inline]
    pub fn txchet(&mut self) -> _TXCHETW {
        _TXCHETW { w: self }
    }
}
