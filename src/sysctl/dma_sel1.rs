#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMA_SEL1 {
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
#[doc = "Possible values of the field `dma_sel5`"]
pub type DMA_SEL5R = super::dma_sel0::DMA_SEL0R;
#[doc = "Values that can be written to the field `dma_sel5`"]
pub type DMA_SEL5W = super::dma_sel0::DMA_SEL0W;
#[doc = r" Proxy"]
pub struct _DMA_SEL5W<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_SEL5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_SEL5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn ssi0_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::SSI0_RX_REQ)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn ssi0_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::SSI0_TX_REQ)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn ssi1_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::SSI1_RX_REQ)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn ssi1_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::SSI1_TX_REQ)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn ssi2_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::SSI2_RX_REQ)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn ssi2_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::SSI2_TX_REQ)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn ssi3_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::SSI3_RX_REQ)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn ssi3_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::SSI3_TX_REQ)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn i2c0_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::I2C0_RX_REQ)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn i2c0_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::I2C0_TX_REQ)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn i2c1_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::I2C1_RX_REQ)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn i2c1_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::I2C1_TX_REQ)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn i2c2_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::I2C2_RX_REQ)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn i2c2_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::I2C2_TX_REQ)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn uart1_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::UART1_RX_REQ)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn uart1_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::UART1_TX_REQ)
    }
    #[doc = "`10000`"]
    #[inline]
    pub fn uart2_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::UART2_RX_REQ)
    }
    #[doc = "`10001`"]
    #[inline]
    pub fn uart2_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::UART2_TX_REQ)
    }
    #[doc = "`10010`"]
    #[inline]
    pub fn uart3_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::UART3_RX_REQ)
    }
    #[doc = "`10011`"]
    #[inline]
    pub fn uart3_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::UART3_TX_REQ)
    }
    #[doc = "`10100`"]
    #[inline]
    pub fn aes_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::AES_REQ)
    }
    #[doc = "`10101`"]
    #[inline]
    pub fn sha_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::SHA_RX_REQ)
    }
    #[doc = "`10110`"]
    #[inline]
    pub fn ai_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::AI_RX_REQ)
    }
    #[doc = "`10111`"]
    #[inline]
    pub fn fft_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::FFT_RX_REQ)
    }
    #[doc = "`11000`"]
    #[inline]
    pub fn fft_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::FFT_TX_REQ)
    }
    #[doc = "`11001`"]
    #[inline]
    pub fn i2s0_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::I2S0_TX_REQ)
    }
    #[doc = "`11010`"]
    #[inline]
    pub fn i2s0_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::I2S0_RX_REQ)
    }
    #[doc = "`11011`"]
    #[inline]
    pub fn i2s1_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::I2S1_TX_REQ)
    }
    #[doc = "`11100`"]
    #[inline]
    pub fn i2s1_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::I2S1_RX_REQ)
    }
    #[doc = "`11101`"]
    #[inline]
    pub fn i2s2_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::I2S2_TX_REQ)
    }
    #[doc = "`11110`"]
    #[inline]
    pub fn i2s2_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::I2S2_RX_REQ)
    }
    #[doc = "`11111`"]
    #[inline]
    pub fn i2s0_bf_dir_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::I2S0_BF_DIR_REQ)
    }
    #[doc = "`100000`"]
    #[inline]
    pub fn i2s0_bf_voice_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0W::I2S0_BF_VOICE_REQ)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
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
    #[doc = "Bits 0:5"]
    #[inline]
    pub fn dma_sel5(&self) -> DMA_SEL5R {
        DMA_SEL5R::_from({
            const MASK: u8 = 63;
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
    #[doc = "Bits 0:5"]
    #[inline]
    pub fn dma_sel5(&mut self) -> _DMA_SEL5W {
        _DMA_SEL5W { w: self }
    }
}
