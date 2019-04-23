#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMA_SEL0 {
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
#[doc = "Possible values of the field `dma_sel0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_SEL0R {
    #[doc = "undocumented"]
    SSI0_RX_REQ,
    #[doc = "undocumented"]
    SSI0_TX_REQ,
    #[doc = "undocumented"]
    SSI1_RX_REQ,
    #[doc = "undocumented"]
    SSI1_TX_REQ,
    #[doc = "undocumented"]
    SSI2_RX_REQ,
    #[doc = "undocumented"]
    SSI2_TX_REQ,
    #[doc = "undocumented"]
    SSI3_RX_REQ,
    #[doc = "undocumented"]
    SSI3_TX_REQ,
    #[doc = "undocumented"]
    I2C0_RX_REQ,
    #[doc = "undocumented"]
    I2C0_TX_REQ,
    #[doc = "undocumented"]
    I2C1_RX_REQ,
    #[doc = "undocumented"]
    I2C1_TX_REQ,
    #[doc = "undocumented"]
    I2C2_RX_REQ,
    #[doc = "undocumented"]
    I2C2_TX_REQ,
    #[doc = "undocumented"]
    UART1_RX_REQ,
    #[doc = "undocumented"]
    UART1_TX_REQ,
    #[doc = "undocumented"]
    UART2_RX_REQ,
    #[doc = "undocumented"]
    UART2_TX_REQ,
    #[doc = "undocumented"]
    UART3_RX_REQ,
    #[doc = "undocumented"]
    UART3_TX_REQ,
    #[doc = "undocumented"]
    AES_REQ,
    #[doc = "undocumented"]
    SHA_RX_REQ,
    #[doc = "undocumented"]
    AI_RX_REQ,
    #[doc = "undocumented"]
    FFT_RX_REQ,
    #[doc = "undocumented"]
    FFT_TX_REQ,
    #[doc = "undocumented"]
    I2S0_TX_REQ,
    #[doc = "undocumented"]
    I2S0_RX_REQ,
    #[doc = "undocumented"]
    I2S1_TX_REQ,
    #[doc = "undocumented"]
    I2S1_RX_REQ,
    #[doc = "undocumented"]
    I2S2_TX_REQ,
    #[doc = "undocumented"]
    I2S2_RX_REQ,
    #[doc = "undocumented"]
    I2S0_BF_DIR_REQ,
    #[doc = "undocumented"]
    I2S0_BF_VOICE_REQ,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DMA_SEL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMA_SEL0R::SSI0_RX_REQ => 0,
            DMA_SEL0R::SSI0_TX_REQ => 1,
            DMA_SEL0R::SSI1_RX_REQ => 2,
            DMA_SEL0R::SSI1_TX_REQ => 3,
            DMA_SEL0R::SSI2_RX_REQ => 4,
            DMA_SEL0R::SSI2_TX_REQ => 5,
            DMA_SEL0R::SSI3_RX_REQ => 6,
            DMA_SEL0R::SSI3_TX_REQ => 7,
            DMA_SEL0R::I2C0_RX_REQ => 8,
            DMA_SEL0R::I2C0_TX_REQ => 9,
            DMA_SEL0R::I2C1_RX_REQ => 10,
            DMA_SEL0R::I2C1_TX_REQ => 11,
            DMA_SEL0R::I2C2_RX_REQ => 12,
            DMA_SEL0R::I2C2_TX_REQ => 13,
            DMA_SEL0R::UART1_RX_REQ => 14,
            DMA_SEL0R::UART1_TX_REQ => 15,
            DMA_SEL0R::UART2_RX_REQ => 16,
            DMA_SEL0R::UART2_TX_REQ => 17,
            DMA_SEL0R::UART3_RX_REQ => 18,
            DMA_SEL0R::UART3_TX_REQ => 19,
            DMA_SEL0R::AES_REQ => 20,
            DMA_SEL0R::SHA_RX_REQ => 21,
            DMA_SEL0R::AI_RX_REQ => 22,
            DMA_SEL0R::FFT_RX_REQ => 23,
            DMA_SEL0R::FFT_TX_REQ => 24,
            DMA_SEL0R::I2S0_TX_REQ => 25,
            DMA_SEL0R::I2S0_RX_REQ => 26,
            DMA_SEL0R::I2S1_TX_REQ => 27,
            DMA_SEL0R::I2S1_RX_REQ => 28,
            DMA_SEL0R::I2S2_TX_REQ => 29,
            DMA_SEL0R::I2S2_RX_REQ => 30,
            DMA_SEL0R::I2S0_BF_DIR_REQ => 31,
            DMA_SEL0R::I2S0_BF_VOICE_REQ => 32,
            DMA_SEL0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMA_SEL0R {
        match value {
            0 => DMA_SEL0R::SSI0_RX_REQ,
            1 => DMA_SEL0R::SSI0_TX_REQ,
            2 => DMA_SEL0R::SSI1_RX_REQ,
            3 => DMA_SEL0R::SSI1_TX_REQ,
            4 => DMA_SEL0R::SSI2_RX_REQ,
            5 => DMA_SEL0R::SSI2_TX_REQ,
            6 => DMA_SEL0R::SSI3_RX_REQ,
            7 => DMA_SEL0R::SSI3_TX_REQ,
            8 => DMA_SEL0R::I2C0_RX_REQ,
            9 => DMA_SEL0R::I2C0_TX_REQ,
            10 => DMA_SEL0R::I2C1_RX_REQ,
            11 => DMA_SEL0R::I2C1_TX_REQ,
            12 => DMA_SEL0R::I2C2_RX_REQ,
            13 => DMA_SEL0R::I2C2_TX_REQ,
            14 => DMA_SEL0R::UART1_RX_REQ,
            15 => DMA_SEL0R::UART1_TX_REQ,
            16 => DMA_SEL0R::UART2_RX_REQ,
            17 => DMA_SEL0R::UART2_TX_REQ,
            18 => DMA_SEL0R::UART3_RX_REQ,
            19 => DMA_SEL0R::UART3_TX_REQ,
            20 => DMA_SEL0R::AES_REQ,
            21 => DMA_SEL0R::SHA_RX_REQ,
            22 => DMA_SEL0R::AI_RX_REQ,
            23 => DMA_SEL0R::FFT_RX_REQ,
            24 => DMA_SEL0R::FFT_TX_REQ,
            25 => DMA_SEL0R::I2S0_TX_REQ,
            26 => DMA_SEL0R::I2S0_RX_REQ,
            27 => DMA_SEL0R::I2S1_TX_REQ,
            28 => DMA_SEL0R::I2S1_RX_REQ,
            29 => DMA_SEL0R::I2S2_TX_REQ,
            30 => DMA_SEL0R::I2S2_RX_REQ,
            31 => DMA_SEL0R::I2S0_BF_DIR_REQ,
            32 => DMA_SEL0R::I2S0_BF_VOICE_REQ,
            i => DMA_SEL0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SSI0_RX_REQ`"]
    #[inline]
    pub fn is_ssi0_rx_req(&self) -> bool {
        *self == DMA_SEL0R::SSI0_RX_REQ
    }
    #[doc = "Checks if the value of the field is `SSI0_TX_REQ`"]
    #[inline]
    pub fn is_ssi0_tx_req(&self) -> bool {
        *self == DMA_SEL0R::SSI0_TX_REQ
    }
    #[doc = "Checks if the value of the field is `SSI1_RX_REQ`"]
    #[inline]
    pub fn is_ssi1_rx_req(&self) -> bool {
        *self == DMA_SEL0R::SSI1_RX_REQ
    }
    #[doc = "Checks if the value of the field is `SSI1_TX_REQ`"]
    #[inline]
    pub fn is_ssi1_tx_req(&self) -> bool {
        *self == DMA_SEL0R::SSI1_TX_REQ
    }
    #[doc = "Checks if the value of the field is `SSI2_RX_REQ`"]
    #[inline]
    pub fn is_ssi2_rx_req(&self) -> bool {
        *self == DMA_SEL0R::SSI2_RX_REQ
    }
    #[doc = "Checks if the value of the field is `SSI2_TX_REQ`"]
    #[inline]
    pub fn is_ssi2_tx_req(&self) -> bool {
        *self == DMA_SEL0R::SSI2_TX_REQ
    }
    #[doc = "Checks if the value of the field is `SSI3_RX_REQ`"]
    #[inline]
    pub fn is_ssi3_rx_req(&self) -> bool {
        *self == DMA_SEL0R::SSI3_RX_REQ
    }
    #[doc = "Checks if the value of the field is `SSI3_TX_REQ`"]
    #[inline]
    pub fn is_ssi3_tx_req(&self) -> bool {
        *self == DMA_SEL0R::SSI3_TX_REQ
    }
    #[doc = "Checks if the value of the field is `I2C0_RX_REQ`"]
    #[inline]
    pub fn is_i2c0_rx_req(&self) -> bool {
        *self == DMA_SEL0R::I2C0_RX_REQ
    }
    #[doc = "Checks if the value of the field is `I2C0_TX_REQ`"]
    #[inline]
    pub fn is_i2c0_tx_req(&self) -> bool {
        *self == DMA_SEL0R::I2C0_TX_REQ
    }
    #[doc = "Checks if the value of the field is `I2C1_RX_REQ`"]
    #[inline]
    pub fn is_i2c1_rx_req(&self) -> bool {
        *self == DMA_SEL0R::I2C1_RX_REQ
    }
    #[doc = "Checks if the value of the field is `I2C1_TX_REQ`"]
    #[inline]
    pub fn is_i2c1_tx_req(&self) -> bool {
        *self == DMA_SEL0R::I2C1_TX_REQ
    }
    #[doc = "Checks if the value of the field is `I2C2_RX_REQ`"]
    #[inline]
    pub fn is_i2c2_rx_req(&self) -> bool {
        *self == DMA_SEL0R::I2C2_RX_REQ
    }
    #[doc = "Checks if the value of the field is `I2C2_TX_REQ`"]
    #[inline]
    pub fn is_i2c2_tx_req(&self) -> bool {
        *self == DMA_SEL0R::I2C2_TX_REQ
    }
    #[doc = "Checks if the value of the field is `UART1_RX_REQ`"]
    #[inline]
    pub fn is_uart1_rx_req(&self) -> bool {
        *self == DMA_SEL0R::UART1_RX_REQ
    }
    #[doc = "Checks if the value of the field is `UART1_TX_REQ`"]
    #[inline]
    pub fn is_uart1_tx_req(&self) -> bool {
        *self == DMA_SEL0R::UART1_TX_REQ
    }
    #[doc = "Checks if the value of the field is `UART2_RX_REQ`"]
    #[inline]
    pub fn is_uart2_rx_req(&self) -> bool {
        *self == DMA_SEL0R::UART2_RX_REQ
    }
    #[doc = "Checks if the value of the field is `UART2_TX_REQ`"]
    #[inline]
    pub fn is_uart2_tx_req(&self) -> bool {
        *self == DMA_SEL0R::UART2_TX_REQ
    }
    #[doc = "Checks if the value of the field is `UART3_RX_REQ`"]
    #[inline]
    pub fn is_uart3_rx_req(&self) -> bool {
        *self == DMA_SEL0R::UART3_RX_REQ
    }
    #[doc = "Checks if the value of the field is `UART3_TX_REQ`"]
    #[inline]
    pub fn is_uart3_tx_req(&self) -> bool {
        *self == DMA_SEL0R::UART3_TX_REQ
    }
    #[doc = "Checks if the value of the field is `AES_REQ`"]
    #[inline]
    pub fn is_aes_req(&self) -> bool {
        *self == DMA_SEL0R::AES_REQ
    }
    #[doc = "Checks if the value of the field is `SHA_RX_REQ`"]
    #[inline]
    pub fn is_sha_rx_req(&self) -> bool {
        *self == DMA_SEL0R::SHA_RX_REQ
    }
    #[doc = "Checks if the value of the field is `AI_RX_REQ`"]
    #[inline]
    pub fn is_ai_rx_req(&self) -> bool {
        *self == DMA_SEL0R::AI_RX_REQ
    }
    #[doc = "Checks if the value of the field is `FFT_RX_REQ`"]
    #[inline]
    pub fn is_fft_rx_req(&self) -> bool {
        *self == DMA_SEL0R::FFT_RX_REQ
    }
    #[doc = "Checks if the value of the field is `FFT_TX_REQ`"]
    #[inline]
    pub fn is_fft_tx_req(&self) -> bool {
        *self == DMA_SEL0R::FFT_TX_REQ
    }
    #[doc = "Checks if the value of the field is `I2S0_TX_REQ`"]
    #[inline]
    pub fn is_i2s0_tx_req(&self) -> bool {
        *self == DMA_SEL0R::I2S0_TX_REQ
    }
    #[doc = "Checks if the value of the field is `I2S0_RX_REQ`"]
    #[inline]
    pub fn is_i2s0_rx_req(&self) -> bool {
        *self == DMA_SEL0R::I2S0_RX_REQ
    }
    #[doc = "Checks if the value of the field is `I2S1_TX_REQ`"]
    #[inline]
    pub fn is_i2s1_tx_req(&self) -> bool {
        *self == DMA_SEL0R::I2S1_TX_REQ
    }
    #[doc = "Checks if the value of the field is `I2S1_RX_REQ`"]
    #[inline]
    pub fn is_i2s1_rx_req(&self) -> bool {
        *self == DMA_SEL0R::I2S1_RX_REQ
    }
    #[doc = "Checks if the value of the field is `I2S2_TX_REQ`"]
    #[inline]
    pub fn is_i2s2_tx_req(&self) -> bool {
        *self == DMA_SEL0R::I2S2_TX_REQ
    }
    #[doc = "Checks if the value of the field is `I2S2_RX_REQ`"]
    #[inline]
    pub fn is_i2s2_rx_req(&self) -> bool {
        *self == DMA_SEL0R::I2S2_RX_REQ
    }
    #[doc = "Checks if the value of the field is `I2S0_BF_DIR_REQ`"]
    #[inline]
    pub fn is_i2s0_bf_dir_req(&self) -> bool {
        *self == DMA_SEL0R::I2S0_BF_DIR_REQ
    }
    #[doc = "Checks if the value of the field is `I2S0_BF_VOICE_REQ`"]
    #[inline]
    pub fn is_i2s0_bf_voice_req(&self) -> bool {
        *self == DMA_SEL0R::I2S0_BF_VOICE_REQ
    }
}
#[doc = "Possible values of the field `dma_sel1`"]
pub type DMA_SEL1R = DMA_SEL0R;
#[doc = "Possible values of the field `dma_sel2`"]
pub type DMA_SEL2R = DMA_SEL0R;
#[doc = "Possible values of the field `dma_sel3`"]
pub type DMA_SEL3R = DMA_SEL0R;
#[doc = "Possible values of the field `dma_sel4`"]
pub type DMA_SEL4R = DMA_SEL0R;
#[doc = "Values that can be written to the field `dma_sel0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_SEL0W {
    #[doc = "`0`"]
    SSI0_RX_REQ,
    #[doc = "`1`"]
    SSI0_TX_REQ,
    #[doc = "`10`"]
    SSI1_RX_REQ,
    #[doc = "`11`"]
    SSI1_TX_REQ,
    #[doc = "`100`"]
    SSI2_RX_REQ,
    #[doc = "`101`"]
    SSI2_TX_REQ,
    #[doc = "`110`"]
    SSI3_RX_REQ,
    #[doc = "`111`"]
    SSI3_TX_REQ,
    #[doc = "`1000`"]
    I2C0_RX_REQ,
    #[doc = "`1001`"]
    I2C0_TX_REQ,
    #[doc = "`1010`"]
    I2C1_RX_REQ,
    #[doc = "`1011`"]
    I2C1_TX_REQ,
    #[doc = "`1100`"]
    I2C2_RX_REQ,
    #[doc = "`1101`"]
    I2C2_TX_REQ,
    #[doc = "`1110`"]
    UART1_RX_REQ,
    #[doc = "`1111`"]
    UART1_TX_REQ,
    #[doc = "`10000`"]
    UART2_RX_REQ,
    #[doc = "`10001`"]
    UART2_TX_REQ,
    #[doc = "`10010`"]
    UART3_RX_REQ,
    #[doc = "`10011`"]
    UART3_TX_REQ,
    #[doc = "`10100`"]
    AES_REQ,
    #[doc = "`10101`"]
    SHA_RX_REQ,
    #[doc = "`10110`"]
    AI_RX_REQ,
    #[doc = "`10111`"]
    FFT_RX_REQ,
    #[doc = "`11000`"]
    FFT_TX_REQ,
    #[doc = "`11001`"]
    I2S0_TX_REQ,
    #[doc = "`11010`"]
    I2S0_RX_REQ,
    #[doc = "`11011`"]
    I2S1_TX_REQ,
    #[doc = "`11100`"]
    I2S1_RX_REQ,
    #[doc = "`11101`"]
    I2S2_TX_REQ,
    #[doc = "`11110`"]
    I2S2_RX_REQ,
    #[doc = "`11111`"]
    I2S0_BF_DIR_REQ,
    #[doc = "`100000`"]
    I2S0_BF_VOICE_REQ,
}
impl DMA_SEL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMA_SEL0W::SSI0_RX_REQ => 0,
            DMA_SEL0W::SSI0_TX_REQ => 1,
            DMA_SEL0W::SSI1_RX_REQ => 2,
            DMA_SEL0W::SSI1_TX_REQ => 3,
            DMA_SEL0W::SSI2_RX_REQ => 4,
            DMA_SEL0W::SSI2_TX_REQ => 5,
            DMA_SEL0W::SSI3_RX_REQ => 6,
            DMA_SEL0W::SSI3_TX_REQ => 7,
            DMA_SEL0W::I2C0_RX_REQ => 8,
            DMA_SEL0W::I2C0_TX_REQ => 9,
            DMA_SEL0W::I2C1_RX_REQ => 10,
            DMA_SEL0W::I2C1_TX_REQ => 11,
            DMA_SEL0W::I2C2_RX_REQ => 12,
            DMA_SEL0W::I2C2_TX_REQ => 13,
            DMA_SEL0W::UART1_RX_REQ => 14,
            DMA_SEL0W::UART1_TX_REQ => 15,
            DMA_SEL0W::UART2_RX_REQ => 16,
            DMA_SEL0W::UART2_TX_REQ => 17,
            DMA_SEL0W::UART3_RX_REQ => 18,
            DMA_SEL0W::UART3_TX_REQ => 19,
            DMA_SEL0W::AES_REQ => 20,
            DMA_SEL0W::SHA_RX_REQ => 21,
            DMA_SEL0W::AI_RX_REQ => 22,
            DMA_SEL0W::FFT_RX_REQ => 23,
            DMA_SEL0W::FFT_TX_REQ => 24,
            DMA_SEL0W::I2S0_TX_REQ => 25,
            DMA_SEL0W::I2S0_RX_REQ => 26,
            DMA_SEL0W::I2S1_TX_REQ => 27,
            DMA_SEL0W::I2S1_RX_REQ => 28,
            DMA_SEL0W::I2S2_TX_REQ => 29,
            DMA_SEL0W::I2S2_RX_REQ => 30,
            DMA_SEL0W::I2S0_BF_DIR_REQ => 31,
            DMA_SEL0W::I2S0_BF_VOICE_REQ => 32,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_SEL0W<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_SEL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_SEL0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn ssi0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI0_RX_REQ)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn ssi0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI0_TX_REQ)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn ssi1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI1_RX_REQ)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn ssi1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI1_TX_REQ)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn ssi2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI2_RX_REQ)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn ssi2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI2_TX_REQ)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn ssi3_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI3_RX_REQ)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn ssi3_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI3_TX_REQ)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn i2c0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C0_RX_REQ)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn i2c0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C0_TX_REQ)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn i2c1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C1_RX_REQ)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn i2c1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C1_TX_REQ)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn i2c2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C2_RX_REQ)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn i2c2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C2_TX_REQ)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn uart1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART1_RX_REQ)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn uart1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART1_TX_REQ)
    }
    #[doc = "`10000`"]
    #[inline]
    pub fn uart2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART2_RX_REQ)
    }
    #[doc = "`10001`"]
    #[inline]
    pub fn uart2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART2_TX_REQ)
    }
    #[doc = "`10010`"]
    #[inline]
    pub fn uart3_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART3_RX_REQ)
    }
    #[doc = "`10011`"]
    #[inline]
    pub fn uart3_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART3_TX_REQ)
    }
    #[doc = "`10100`"]
    #[inline]
    pub fn aes_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::AES_REQ)
    }
    #[doc = "`10101`"]
    #[inline]
    pub fn sha_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SHA_RX_REQ)
    }
    #[doc = "`10110`"]
    #[inline]
    pub fn ai_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::AI_RX_REQ)
    }
    #[doc = "`10111`"]
    #[inline]
    pub fn fft_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::FFT_RX_REQ)
    }
    #[doc = "`11000`"]
    #[inline]
    pub fn fft_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::FFT_TX_REQ)
    }
    #[doc = "`11001`"]
    #[inline]
    pub fn i2s0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S0_TX_REQ)
    }
    #[doc = "`11010`"]
    #[inline]
    pub fn i2s0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S0_RX_REQ)
    }
    #[doc = "`11011`"]
    #[inline]
    pub fn i2s1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S1_TX_REQ)
    }
    #[doc = "`11100`"]
    #[inline]
    pub fn i2s1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S1_RX_REQ)
    }
    #[doc = "`11101`"]
    #[inline]
    pub fn i2s2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S2_TX_REQ)
    }
    #[doc = "`11110`"]
    #[inline]
    pub fn i2s2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S2_RX_REQ)
    }
    #[doc = "`11111`"]
    #[inline]
    pub fn i2s0_bf_dir_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S0_BF_DIR_REQ)
    }
    #[doc = "`100000`"]
    #[inline]
    pub fn i2s0_bf_voice_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S0_BF_VOICE_REQ)
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
#[doc = "Values that can be written to the field `dma_sel1`"]
pub type DMA_SEL1W = DMA_SEL0W;
#[doc = r" Proxy"]
pub struct _DMA_SEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_SEL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_SEL1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn ssi0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI0_RX_REQ)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn ssi0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI0_TX_REQ)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn ssi1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI1_RX_REQ)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn ssi1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI1_TX_REQ)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn ssi2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI2_RX_REQ)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn ssi2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI2_TX_REQ)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn ssi3_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI3_RX_REQ)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn ssi3_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI3_TX_REQ)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn i2c0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C0_RX_REQ)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn i2c0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C0_TX_REQ)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn i2c1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C1_RX_REQ)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn i2c1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C1_TX_REQ)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn i2c2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C2_RX_REQ)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn i2c2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C2_TX_REQ)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn uart1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART1_RX_REQ)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn uart1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART1_TX_REQ)
    }
    #[doc = "`10000`"]
    #[inline]
    pub fn uart2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART2_RX_REQ)
    }
    #[doc = "`10001`"]
    #[inline]
    pub fn uart2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART2_TX_REQ)
    }
    #[doc = "`10010`"]
    #[inline]
    pub fn uart3_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART3_RX_REQ)
    }
    #[doc = "`10011`"]
    #[inline]
    pub fn uart3_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART3_TX_REQ)
    }
    #[doc = "`10100`"]
    #[inline]
    pub fn aes_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::AES_REQ)
    }
    #[doc = "`10101`"]
    #[inline]
    pub fn sha_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SHA_RX_REQ)
    }
    #[doc = "`10110`"]
    #[inline]
    pub fn ai_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::AI_RX_REQ)
    }
    #[doc = "`10111`"]
    #[inline]
    pub fn fft_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::FFT_RX_REQ)
    }
    #[doc = "`11000`"]
    #[inline]
    pub fn fft_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::FFT_TX_REQ)
    }
    #[doc = "`11001`"]
    #[inline]
    pub fn i2s0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S0_TX_REQ)
    }
    #[doc = "`11010`"]
    #[inline]
    pub fn i2s0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S0_RX_REQ)
    }
    #[doc = "`11011`"]
    #[inline]
    pub fn i2s1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S1_TX_REQ)
    }
    #[doc = "`11100`"]
    #[inline]
    pub fn i2s1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S1_RX_REQ)
    }
    #[doc = "`11101`"]
    #[inline]
    pub fn i2s2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S2_TX_REQ)
    }
    #[doc = "`11110`"]
    #[inline]
    pub fn i2s2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S2_RX_REQ)
    }
    #[doc = "`11111`"]
    #[inline]
    pub fn i2s0_bf_dir_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S0_BF_DIR_REQ)
    }
    #[doc = "`100000`"]
    #[inline]
    pub fn i2s0_bf_voice_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S0_BF_VOICE_REQ)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `dma_sel2`"]
pub type DMA_SEL2W = DMA_SEL0W;
#[doc = r" Proxy"]
pub struct _DMA_SEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_SEL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_SEL2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn ssi0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI0_RX_REQ)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn ssi0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI0_TX_REQ)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn ssi1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI1_RX_REQ)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn ssi1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI1_TX_REQ)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn ssi2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI2_RX_REQ)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn ssi2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI2_TX_REQ)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn ssi3_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI3_RX_REQ)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn ssi3_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI3_TX_REQ)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn i2c0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C0_RX_REQ)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn i2c0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C0_TX_REQ)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn i2c1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C1_RX_REQ)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn i2c1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C1_TX_REQ)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn i2c2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C2_RX_REQ)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn i2c2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C2_TX_REQ)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn uart1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART1_RX_REQ)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn uart1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART1_TX_REQ)
    }
    #[doc = "`10000`"]
    #[inline]
    pub fn uart2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART2_RX_REQ)
    }
    #[doc = "`10001`"]
    #[inline]
    pub fn uart2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART2_TX_REQ)
    }
    #[doc = "`10010`"]
    #[inline]
    pub fn uart3_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART3_RX_REQ)
    }
    #[doc = "`10011`"]
    #[inline]
    pub fn uart3_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART3_TX_REQ)
    }
    #[doc = "`10100`"]
    #[inline]
    pub fn aes_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::AES_REQ)
    }
    #[doc = "`10101`"]
    #[inline]
    pub fn sha_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SHA_RX_REQ)
    }
    #[doc = "`10110`"]
    #[inline]
    pub fn ai_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::AI_RX_REQ)
    }
    #[doc = "`10111`"]
    #[inline]
    pub fn fft_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::FFT_RX_REQ)
    }
    #[doc = "`11000`"]
    #[inline]
    pub fn fft_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::FFT_TX_REQ)
    }
    #[doc = "`11001`"]
    #[inline]
    pub fn i2s0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S0_TX_REQ)
    }
    #[doc = "`11010`"]
    #[inline]
    pub fn i2s0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S0_RX_REQ)
    }
    #[doc = "`11011`"]
    #[inline]
    pub fn i2s1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S1_TX_REQ)
    }
    #[doc = "`11100`"]
    #[inline]
    pub fn i2s1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S1_RX_REQ)
    }
    #[doc = "`11101`"]
    #[inline]
    pub fn i2s2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S2_TX_REQ)
    }
    #[doc = "`11110`"]
    #[inline]
    pub fn i2s2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S2_RX_REQ)
    }
    #[doc = "`11111`"]
    #[inline]
    pub fn i2s0_bf_dir_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S0_BF_DIR_REQ)
    }
    #[doc = "`100000`"]
    #[inline]
    pub fn i2s0_bf_voice_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S0_BF_VOICE_REQ)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `dma_sel3`"]
pub type DMA_SEL3W = DMA_SEL0W;
#[doc = r" Proxy"]
pub struct _DMA_SEL3W<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_SEL3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_SEL3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn ssi0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI0_RX_REQ)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn ssi0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI0_TX_REQ)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn ssi1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI1_RX_REQ)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn ssi1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI1_TX_REQ)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn ssi2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI2_RX_REQ)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn ssi2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI2_TX_REQ)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn ssi3_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI3_RX_REQ)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn ssi3_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI3_TX_REQ)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn i2c0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C0_RX_REQ)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn i2c0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C0_TX_REQ)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn i2c1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C1_RX_REQ)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn i2c1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C1_TX_REQ)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn i2c2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C2_RX_REQ)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn i2c2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C2_TX_REQ)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn uart1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART1_RX_REQ)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn uart1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART1_TX_REQ)
    }
    #[doc = "`10000`"]
    #[inline]
    pub fn uart2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART2_RX_REQ)
    }
    #[doc = "`10001`"]
    #[inline]
    pub fn uart2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART2_TX_REQ)
    }
    #[doc = "`10010`"]
    #[inline]
    pub fn uart3_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART3_RX_REQ)
    }
    #[doc = "`10011`"]
    #[inline]
    pub fn uart3_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART3_TX_REQ)
    }
    #[doc = "`10100`"]
    #[inline]
    pub fn aes_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::AES_REQ)
    }
    #[doc = "`10101`"]
    #[inline]
    pub fn sha_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SHA_RX_REQ)
    }
    #[doc = "`10110`"]
    #[inline]
    pub fn ai_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::AI_RX_REQ)
    }
    #[doc = "`10111`"]
    #[inline]
    pub fn fft_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::FFT_RX_REQ)
    }
    #[doc = "`11000`"]
    #[inline]
    pub fn fft_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::FFT_TX_REQ)
    }
    #[doc = "`11001`"]
    #[inline]
    pub fn i2s0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S0_TX_REQ)
    }
    #[doc = "`11010`"]
    #[inline]
    pub fn i2s0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S0_RX_REQ)
    }
    #[doc = "`11011`"]
    #[inline]
    pub fn i2s1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S1_TX_REQ)
    }
    #[doc = "`11100`"]
    #[inline]
    pub fn i2s1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S1_RX_REQ)
    }
    #[doc = "`11101`"]
    #[inline]
    pub fn i2s2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S2_TX_REQ)
    }
    #[doc = "`11110`"]
    #[inline]
    pub fn i2s2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S2_RX_REQ)
    }
    #[doc = "`11111`"]
    #[inline]
    pub fn i2s0_bf_dir_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S0_BF_DIR_REQ)
    }
    #[doc = "`100000`"]
    #[inline]
    pub fn i2s0_bf_voice_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S0_BF_VOICE_REQ)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `dma_sel4`"]
pub type DMA_SEL4W = DMA_SEL0W;
#[doc = r" Proxy"]
pub struct _DMA_SEL4W<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_SEL4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_SEL4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn ssi0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI0_RX_REQ)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn ssi0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI0_TX_REQ)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn ssi1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI1_RX_REQ)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn ssi1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI1_TX_REQ)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn ssi2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI2_RX_REQ)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn ssi2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI2_TX_REQ)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn ssi3_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI3_RX_REQ)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn ssi3_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SSI3_TX_REQ)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn i2c0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C0_RX_REQ)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn i2c0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C0_TX_REQ)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn i2c1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C1_RX_REQ)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn i2c1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C1_TX_REQ)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn i2c2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C2_RX_REQ)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn i2c2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2C2_TX_REQ)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn uart1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART1_RX_REQ)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn uart1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART1_TX_REQ)
    }
    #[doc = "`10000`"]
    #[inline]
    pub fn uart2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART2_RX_REQ)
    }
    #[doc = "`10001`"]
    #[inline]
    pub fn uart2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART2_TX_REQ)
    }
    #[doc = "`10010`"]
    #[inline]
    pub fn uart3_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART3_RX_REQ)
    }
    #[doc = "`10011`"]
    #[inline]
    pub fn uart3_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::UART3_TX_REQ)
    }
    #[doc = "`10100`"]
    #[inline]
    pub fn aes_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::AES_REQ)
    }
    #[doc = "`10101`"]
    #[inline]
    pub fn sha_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::SHA_RX_REQ)
    }
    #[doc = "`10110`"]
    #[inline]
    pub fn ai_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::AI_RX_REQ)
    }
    #[doc = "`10111`"]
    #[inline]
    pub fn fft_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::FFT_RX_REQ)
    }
    #[doc = "`11000`"]
    #[inline]
    pub fn fft_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::FFT_TX_REQ)
    }
    #[doc = "`11001`"]
    #[inline]
    pub fn i2s0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S0_TX_REQ)
    }
    #[doc = "`11010`"]
    #[inline]
    pub fn i2s0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S0_RX_REQ)
    }
    #[doc = "`11011`"]
    #[inline]
    pub fn i2s1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S1_TX_REQ)
    }
    #[doc = "`11100`"]
    #[inline]
    pub fn i2s1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S1_RX_REQ)
    }
    #[doc = "`11101`"]
    #[inline]
    pub fn i2s2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S2_TX_REQ)
    }
    #[doc = "`11110`"]
    #[inline]
    pub fn i2s2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S2_RX_REQ)
    }
    #[doc = "`11111`"]
    #[inline]
    pub fn i2s0_bf_dir_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S0_BF_DIR_REQ)
    }
    #[doc = "`100000`"]
    #[inline]
    pub fn i2s0_bf_voice_req(self) -> &'a mut W {
        self.variant(DMA_SEL0W::I2S0_BF_VOICE_REQ)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
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
    pub fn dma_sel0(&self) -> DMA_SEL0R {
        DMA_SEL0R::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:11"]
    #[inline]
    pub fn dma_sel1(&self) -> DMA_SEL1R {
        DMA_SEL1R::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:17"]
    #[inline]
    pub fn dma_sel2(&self) -> DMA_SEL2R {
        DMA_SEL2R::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:23"]
    #[inline]
    pub fn dma_sel3(&self) -> DMA_SEL3R {
        DMA_SEL3R::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:29"]
    #[inline]
    pub fn dma_sel4(&self) -> DMA_SEL4R {
        DMA_SEL4R::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
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
    pub fn dma_sel0(&mut self) -> _DMA_SEL0W {
        _DMA_SEL0W { w: self }
    }
    #[doc = "Bits 6:11"]
    #[inline]
    pub fn dma_sel1(&mut self) -> _DMA_SEL1W {
        _DMA_SEL1W { w: self }
    }
    #[doc = "Bits 12:17"]
    #[inline]
    pub fn dma_sel2(&mut self) -> _DMA_SEL2W {
        _DMA_SEL2W { w: self }
    }
    #[doc = "Bits 18:23"]
    #[inline]
    pub fn dma_sel3(&mut self) -> _DMA_SEL3W {
        _DMA_SEL3W { w: self }
    }
    #[doc = "Bits 24:29"]
    #[inline]
    pub fn dma_sel4(&mut self) -> _DMA_SEL4W {
        _DMA_SEL4W { w: self }
    }
}
