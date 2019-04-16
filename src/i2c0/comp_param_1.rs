#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::COMP_PARAM_1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct APB_DATA_WIDTHR {
    bits: u8,
}
impl APB_DATA_WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAX_SPEED_MODER {
    bits: u8,
}
impl MAX_SPEED_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HC_COUNT_VALUESR {
    bits: bool,
}
impl HC_COUNT_VALUESR {
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
pub struct INTR_IOR {
    bits: bool,
}
impl INTR_IOR {
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
pub struct HAS_DMAR {
    bits: bool,
}
impl HAS_DMAR {
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
pub struct ENCODED_PARAMSR {
    bits: bool,
}
impl ENCODED_PARAMSR {
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
pub struct RX_BUFFER_DEPTHR {
    bits: u8,
}
impl RX_BUFFER_DEPTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TX_BUFFER_DEPTHR {
    bits: u8,
}
impl TX_BUFFER_DEPTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - APB_DATA_WIDTH"]
    #[inline]
    pub fn apb_data_width(&self) -> APB_DATA_WIDTHR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        APB_DATA_WIDTHR { bits }
    }
    #[doc = "Bits 2:3 - MAX_SPEED_MODE"]
    #[inline]
    pub fn max_speed_mode(&self) -> MAX_SPEED_MODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAX_SPEED_MODER { bits }
    }
    #[doc = "Bit 4 - HC_COUNT_VALUES"]
    #[inline]
    pub fn hc_count_values(&self) -> HC_COUNT_VALUESR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HC_COUNT_VALUESR { bits }
    }
    #[doc = "Bit 5 - INTR_IO"]
    #[inline]
    pub fn intr_io(&self) -> INTR_IOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTR_IOR { bits }
    }
    #[doc = "Bit 6 - HAS_DMA"]
    #[inline]
    pub fn has_dma(&self) -> HAS_DMAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HAS_DMAR { bits }
    }
    #[doc = "Bit 7 - ENCODED_PARAMS"]
    #[inline]
    pub fn encoded_params(&self) -> ENCODED_PARAMSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENCODED_PARAMSR { bits }
    }
    #[doc = "Bits 8:15 - RX_BUFFER_DEPTH"]
    #[inline]
    pub fn rx_buffer_depth(&self) -> RX_BUFFER_DEPTHR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RX_BUFFER_DEPTHR { bits }
    }
    #[doc = "Bits 16:23 - TX_BUFFER_DEPTH"]
    #[inline]
    pub fn tx_buffer_depth(&self) -> TX_BUFFER_DEPTHR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TX_BUFFER_DEPTHR { bits }
    }
}
