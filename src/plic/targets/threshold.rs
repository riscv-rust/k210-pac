#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::THRESHOLD {
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
#[doc = "Possible values of the field `priority`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRIORITYR {
    #[doc = "Never interrupt"]
    NEVER,
    #[doc = "Priority 1"]
    P1,
    #[doc = "Priority 2"]
    P2,
    #[doc = "Priority 3"]
    P3,
    #[doc = "Priority 4"]
    P4,
    #[doc = "Priority 5"]
    P5,
    #[doc = "Priority 6"]
    P6,
    #[doc = "Priority 7"]
    P7,
}
impl PRIORITYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRIORITYR::NEVER => 0,
            PRIORITYR::P1 => 1,
            PRIORITYR::P2 => 2,
            PRIORITYR::P3 => 3,
            PRIORITYR::P4 => 4,
            PRIORITYR::P5 => 5,
            PRIORITYR::P6 => 6,
            PRIORITYR::P7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRIORITYR {
        match value {
            0 => PRIORITYR::NEVER,
            1 => PRIORITYR::P1,
            2 => PRIORITYR::P2,
            3 => PRIORITYR::P3,
            4 => PRIORITYR::P4,
            5 => PRIORITYR::P5,
            6 => PRIORITYR::P6,
            7 => PRIORITYR::P7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEVER`"]
    #[inline]
    pub fn is_never(&self) -> bool {
        *self == PRIORITYR::NEVER
    }
    #[doc = "Checks if the value of the field is `P1`"]
    #[inline]
    pub fn is_p1(&self) -> bool {
        *self == PRIORITYR::P1
    }
    #[doc = "Checks if the value of the field is `P2`"]
    #[inline]
    pub fn is_p2(&self) -> bool {
        *self == PRIORITYR::P2
    }
    #[doc = "Checks if the value of the field is `P3`"]
    #[inline]
    pub fn is_p3(&self) -> bool {
        *self == PRIORITYR::P3
    }
    #[doc = "Checks if the value of the field is `P4`"]
    #[inline]
    pub fn is_p4(&self) -> bool {
        *self == PRIORITYR::P4
    }
    #[doc = "Checks if the value of the field is `P5`"]
    #[inline]
    pub fn is_p5(&self) -> bool {
        *self == PRIORITYR::P5
    }
    #[doc = "Checks if the value of the field is `P6`"]
    #[inline]
    pub fn is_p6(&self) -> bool {
        *self == PRIORITYR::P6
    }
    #[doc = "Checks if the value of the field is `P7`"]
    #[inline]
    pub fn is_p7(&self) -> bool {
        *self == PRIORITYR::P7
    }
}
#[doc = "Values that can be written to the field `priority`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRIORITYW {
    #[doc = "Never interrupt"]
    NEVER,
    #[doc = "Priority 1"]
    P1,
    #[doc = "Priority 2"]
    P2,
    #[doc = "Priority 3"]
    P3,
    #[doc = "Priority 4"]
    P4,
    #[doc = "Priority 5"]
    P5,
    #[doc = "Priority 6"]
    P6,
    #[doc = "Priority 7"]
    P7,
}
impl PRIORITYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRIORITYW::NEVER => 0,
            PRIORITYW::P1 => 1,
            PRIORITYW::P2 => 2,
            PRIORITYW::P3 => 3,
            PRIORITYW::P4 => 4,
            PRIORITYW::P5 => 5,
            PRIORITYW::P6 => 6,
            PRIORITYW::P7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRIORITYW<'a> {
    w: &'a mut W,
}
impl<'a> _PRIORITYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRIORITYW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Never interrupt"]
    #[inline]
    pub fn never(self) -> &'a mut W {
        self.variant(PRIORITYW::NEVER)
    }
    #[doc = "Priority 1"]
    #[inline]
    pub fn p1(self) -> &'a mut W {
        self.variant(PRIORITYW::P1)
    }
    #[doc = "Priority 2"]
    #[inline]
    pub fn p2(self) -> &'a mut W {
        self.variant(PRIORITYW::P2)
    }
    #[doc = "Priority 3"]
    #[inline]
    pub fn p3(self) -> &'a mut W {
        self.variant(PRIORITYW::P3)
    }
    #[doc = "Priority 4"]
    #[inline]
    pub fn p4(self) -> &'a mut W {
        self.variant(PRIORITYW::P4)
    }
    #[doc = "Priority 5"]
    #[inline]
    pub fn p5(self) -> &'a mut W {
        self.variant(PRIORITYW::P5)
    }
    #[doc = "Priority 6"]
    #[inline]
    pub fn p6(self) -> &'a mut W {
        self.variant(PRIORITYW::P6)
    }
    #[doc = "Priority 7"]
    #[inline]
    pub fn p7(self) -> &'a mut W {
        self.variant(PRIORITYW::P7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 0:2"]
    #[inline]
    pub fn priority(&self) -> PRIORITYR {
        PRIORITYR::_from({
            const MASK: u8 = 7;
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
    #[doc = "Bits 0:2"]
    #[inline]
    pub fn priority(&mut self) -> _PRIORITYW {
        _PRIORITYW { w: self }
    }
}
