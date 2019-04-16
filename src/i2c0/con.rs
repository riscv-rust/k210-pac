#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CON {
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
pub struct MASTER_MODER {
    bits: bool,
}
impl MASTER_MODER {
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
#[doc = "Possible values of the field `speed`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEEDR {
    #[doc = "STANDARD"]
    STANDARD,
    #[doc = "FAST"]
    FAST,
    #[doc = "HIGHSPEED"]
    HIGHSPEED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPEEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPEEDR::STANDARD => 0,
            SPEEDR::FAST => 1,
            SPEEDR::HIGHSPEED => 2,
            SPEEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPEEDR {
        match value {
            0 => SPEEDR::STANDARD,
            1 => SPEEDR::FAST,
            2 => SPEEDR::HIGHSPEED,
            i => SPEEDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline]
    pub fn is_standard(&self) -> bool {
        *self == SPEEDR::STANDARD
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline]
    pub fn is_fast(&self) -> bool {
        *self == SPEEDR::FAST
    }
    #[doc = "Checks if the value of the field is `HIGHSPEED`"]
    #[inline]
    pub fn is_highspeed(&self) -> bool {
        *self == SPEEDR::HIGHSPEED
    }
}
#[doc = r" Value of the field"]
pub struct ADDR_SLAVER {
    bits: bool,
}
impl ADDR_SLAVER {
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
pub struct RESTART_ENR {
    bits: bool,
}
impl RESTART_ENR {
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
pub struct SLAVE_DISABLER {
    bits: bool,
}
impl SLAVE_DISABLER {
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
pub struct STOP_DETR {
    bits: bool,
}
impl STOP_DETR {
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
pub struct TX_EMPTYR {
    bits: bool,
}
impl TX_EMPTYR {
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
#[doc = r" Proxy"]
pub struct _MASTER_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MASTER_MODEW<'a> {
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
#[doc = "Values that can be written to the field `speed`"]
pub enum SPEEDW {
    #[doc = "STANDARD"]
    STANDARD,
    #[doc = "FAST"]
    FAST,
    #[doc = "HIGHSPEED"]
    HIGHSPEED,
}
impl SPEEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SPEEDW::STANDARD => 0,
            SPEEDW::FAST => 1,
            SPEEDW::HIGHSPEED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPEEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SPEEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPEEDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "STANDARD"]
    #[inline]
    pub fn standard(self) -> &'a mut W {
        self.variant(SPEEDW::STANDARD)
    }
    #[doc = "FAST"]
    #[inline]
    pub fn fast(self) -> &'a mut W {
        self.variant(SPEEDW::FAST)
    }
    #[doc = "HIGHSPEED"]
    #[inline]
    pub fn highspeed(self) -> &'a mut W {
        self.variant(SPEEDW::HIGHSPEED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADDR_SLAVEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDR_SLAVEW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESTART_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RESTART_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _SLAVE_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _SLAVE_DISABLEW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STOP_DETW<'a> {
    w: &'a mut W,
}
impl<'a> _STOP_DETW<'a> {
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
#[doc = r" Proxy"]
pub struct _TX_EMPTYW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_EMPTYW<'a> {
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
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Master Mode"]
    #[inline]
    pub fn master_mode(&self) -> MASTER_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MASTER_MODER { bits }
    }
    #[doc = "Bits 1:2 - Speed"]
    #[inline]
    pub fn speed(&self) -> SPEEDR {
        SPEEDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - 10 bit Slave address"]
    #[inline]
    pub fn addr_slave(&self) -> ADDR_SLAVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADDR_SLAVER { bits }
    }
    #[doc = "Bit 5 - Enable Restart"]
    #[inline]
    pub fn restart_en(&self) -> RESTART_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESTART_ENR { bits }
    }
    #[doc = "Bit 6 - Disable Slave"]
    #[inline]
    pub fn slave_disable(&self) -> SLAVE_DISABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SLAVE_DISABLER { bits }
    }
    #[doc = "Bit 7 - STOP_DET_IFADDRESSED"]
    #[inline]
    pub fn stop_det(&self) -> STOP_DETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STOP_DETR { bits }
    }
    #[doc = "Bit 8 - TX_EMPTY_CTRL"]
    #[inline]
    pub fn tx_empty(&self) -> TX_EMPTYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_EMPTYR { bits }
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
    #[doc = "Bit 0 - Master Mode"]
    #[inline]
    pub fn master_mode(&mut self) -> _MASTER_MODEW {
        _MASTER_MODEW { w: self }
    }
    #[doc = "Bits 1:2 - Speed"]
    #[inline]
    pub fn speed(&mut self) -> _SPEEDW {
        _SPEEDW { w: self }
    }
    #[doc = "Bit 3 - 10 bit Slave address"]
    #[inline]
    pub fn addr_slave(&mut self) -> _ADDR_SLAVEW {
        _ADDR_SLAVEW { w: self }
    }
    #[doc = "Bit 5 - Enable Restart"]
    #[inline]
    pub fn restart_en(&mut self) -> _RESTART_ENW {
        _RESTART_ENW { w: self }
    }
    #[doc = "Bit 6 - Disable Slave"]
    #[inline]
    pub fn slave_disable(&mut self) -> _SLAVE_DISABLEW {
        _SLAVE_DISABLEW { w: self }
    }
    #[doc = "Bit 7 - STOP_DET_IFADDRESSED"]
    #[inline]
    pub fn stop_det(&mut self) -> _STOP_DETW {
        _STOP_DETW { w: self }
    }
    #[doc = "Bit 8 - TX_EMPTY_CTRL"]
    #[inline]
    pub fn tx_empty(&mut self) -> _TX_EMPTYW {
        _TX_EMPTYW { w: self }
    }
}
