#[doc = r" Value read from the register"]
pub struct R {
    bits: u64,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u64,
}
impl super::LLP {
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
#[doc = "Possible values of the field `lms`"]
pub type LMSR = super::ctl::SMSR;
#[doc = r" Value of the field"]
pub struct LOCR {
    bits: u64,
}
impl LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u64 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `lms`"]
pub type LMSW = super::ctl::SMSW;
#[doc = r" Proxy"]
pub struct _LMSW<'a> {
    w: &'a mut W,
}
impl<'a> _LMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LMSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AXI master 1"]
    #[inline]
    pub fn axi_master_1(self) -> &'a mut W {
        self.variant(super::ctl::SMSW::AXI_MASTER_1)
    }
    #[doc = "AXI master 2"]
    #[inline]
    pub fn axi_master_2(self) -> &'a mut W {
        self.variant(super::ctl::SMSW::AXI_MASTER_2)
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
#[doc = r" Proxy"]
pub struct _LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u64) -> &'a mut W {
        const MASK: u64 = 4294967295;
        const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - LLI master select"]
    #[inline]
    pub fn lms(&self) -> LMSR {
        LMSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        })
    }
    #[doc = "Bits 6:63 - Starting address memeory of LLI block"]
    #[inline]
    pub fn loc(&self) -> LOCR {
        let bits = {
            const MASK: u64 = 4294967295;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u64) as u64
        };
        LOCR { bits }
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
    #[doc = "Bit 0 - LLI master select"]
    #[inline]
    pub fn lms(&mut self) -> _LMSW {
        _LMSW { w: self }
    }
    #[doc = "Bits 6:63 - Starting address memeory of LLI block"]
    #[inline]
    pub fn loc(&mut self) -> _LOCW {
        _LOCW { w: self }
    }
}
