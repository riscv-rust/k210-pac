#[doc = r" Value read from the register"]
pub struct R {
    bits: u64,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u64,
}
impl super::CFG {
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
#[doc = "Possible values of the field `src_multblk_type`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_MULTBLK_TYPER {
    #[doc = "Continuous multi-block type"]
    CONTIGUOUS,
    #[doc = "Reload multi-block type"]
    RELOAD,
    #[doc = "Shadow register based multi-block type"]
    SHADOWREGISTER,
    #[doc = "Linked list based multi-block type"]
    LINKEDLIST,
}
impl SRC_MULTBLK_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRC_MULTBLK_TYPER::CONTIGUOUS => 0,
            SRC_MULTBLK_TYPER::RELOAD => 1,
            SRC_MULTBLK_TYPER::SHADOWREGISTER => 2,
            SRC_MULTBLK_TYPER::LINKEDLIST => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRC_MULTBLK_TYPER {
        match value {
            0 => SRC_MULTBLK_TYPER::CONTIGUOUS,
            1 => SRC_MULTBLK_TYPER::RELOAD,
            2 => SRC_MULTBLK_TYPER::SHADOWREGISTER,
            3 => SRC_MULTBLK_TYPER::LINKEDLIST,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONTIGUOUS`"]
    #[inline]
    pub fn is_contiguous(&self) -> bool {
        *self == SRC_MULTBLK_TYPER::CONTIGUOUS
    }
    #[doc = "Checks if the value of the field is `RELOAD`"]
    #[inline]
    pub fn is_reload(&self) -> bool {
        *self == SRC_MULTBLK_TYPER::RELOAD
    }
    #[doc = "Checks if the value of the field is `SHADOWREGISTER`"]
    #[inline]
    pub fn is_shadowregister(&self) -> bool {
        *self == SRC_MULTBLK_TYPER::SHADOWREGISTER
    }
    #[doc = "Checks if the value of the field is `LINKEDLIST`"]
    #[inline]
    pub fn is_linkedlist(&self) -> bool {
        *self == SRC_MULTBLK_TYPER::LINKEDLIST
    }
}
#[doc = "Possible values of the field `dst_multblk_type`"]
pub type DST_MULTBLK_TYPER = SRC_MULTBLK_TYPER;
#[doc = "Possible values of the field `tt_fc`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TT_FCR {
    #[doc = "Transfer memory to memory and flow controller is DMAC"]
    MEM2MEM_DMA,
    #[doc = "Transfer memory to peripheral and flow controller is DMAC"]
    MEM2PRF_DMA,
    #[doc = "Transfer peripheral to memory and flow controller is DMAC"]
    PRF2MEM_DMA,
    #[doc = "Transfer peripheral to peripheral and flow controller is DMAC"]
    PRF2PRF_DMA,
    #[doc = "Transfer peripheral to memory and flow controller is source peripheral"]
    PRF2MEM_PRF,
    #[doc = "Transfer peripheral to peripheral and flow controller is source peripheral"]
    PRF2PRF_SRCPRF,
    #[doc = "Transfer memory to peripheral and flow controller is destination peripheral"]
    MEM2PRF_PRF,
    #[doc = "Transfer peripheral to peripheral and flow controller is destination peripheral"]
    PRF2PRF_DSTPRF,
}
impl TT_FCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TT_FCR::MEM2MEM_DMA => 0,
            TT_FCR::MEM2PRF_DMA => 1,
            TT_FCR::PRF2MEM_DMA => 2,
            TT_FCR::PRF2PRF_DMA => 3,
            TT_FCR::PRF2MEM_PRF => 4,
            TT_FCR::PRF2PRF_SRCPRF => 5,
            TT_FCR::MEM2PRF_PRF => 6,
            TT_FCR::PRF2PRF_DSTPRF => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TT_FCR {
        match value {
            0 => TT_FCR::MEM2MEM_DMA,
            1 => TT_FCR::MEM2PRF_DMA,
            2 => TT_FCR::PRF2MEM_DMA,
            3 => TT_FCR::PRF2PRF_DMA,
            4 => TT_FCR::PRF2MEM_PRF,
            5 => TT_FCR::PRF2PRF_SRCPRF,
            6 => TT_FCR::MEM2PRF_PRF,
            7 => TT_FCR::PRF2PRF_DSTPRF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MEM2MEM_DMA`"]
    #[inline]
    pub fn is_mem2mem_dma(&self) -> bool {
        *self == TT_FCR::MEM2MEM_DMA
    }
    #[doc = "Checks if the value of the field is `MEM2PRF_DMA`"]
    #[inline]
    pub fn is_mem2prf_dma(&self) -> bool {
        *self == TT_FCR::MEM2PRF_DMA
    }
    #[doc = "Checks if the value of the field is `PRF2MEM_DMA`"]
    #[inline]
    pub fn is_prf2mem_dma(&self) -> bool {
        *self == TT_FCR::PRF2MEM_DMA
    }
    #[doc = "Checks if the value of the field is `PRF2PRF_DMA`"]
    #[inline]
    pub fn is_prf2prf_dma(&self) -> bool {
        *self == TT_FCR::PRF2PRF_DMA
    }
    #[doc = "Checks if the value of the field is `PRF2MEM_PRF`"]
    #[inline]
    pub fn is_prf2mem_prf(&self) -> bool {
        *self == TT_FCR::PRF2MEM_PRF
    }
    #[doc = "Checks if the value of the field is `PRF2PRF_SRCPRF`"]
    #[inline]
    pub fn is_prf2prf_srcprf(&self) -> bool {
        *self == TT_FCR::PRF2PRF_SRCPRF
    }
    #[doc = "Checks if the value of the field is `MEM2PRF_PRF`"]
    #[inline]
    pub fn is_mem2prf_prf(&self) -> bool {
        *self == TT_FCR::MEM2PRF_PRF
    }
    #[doc = "Checks if the value of the field is `PRF2PRF_DSTPRF`"]
    #[inline]
    pub fn is_prf2prf_dstprf(&self) -> bool {
        *self == TT_FCR::PRF2PRF_DSTPRF
    }
}
#[doc = "Possible values of the field `hs_sel_src`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_SEL_SRCR {
    #[doc = "Hardware handshaking is used"]
    HARDWARE,
    #[doc = "Software handshaking is used"]
    SOFTWARE,
}
impl HS_SEL_SRCR {
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
            HS_SEL_SRCR::HARDWARE => false,
            HS_SEL_SRCR::SOFTWARE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HS_SEL_SRCR {
        match value {
            false => HS_SEL_SRCR::HARDWARE,
            true => HS_SEL_SRCR::SOFTWARE,
        }
    }
    #[doc = "Checks if the value of the field is `HARDWARE`"]
    #[inline]
    pub fn is_hardware(&self) -> bool {
        *self == HS_SEL_SRCR::HARDWARE
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline]
    pub fn is_software(&self) -> bool {
        *self == HS_SEL_SRCR::SOFTWARE
    }
}
#[doc = "Possible values of the field `hs_sel_dst`"]
pub type HS_SEL_DSTR = HS_SEL_SRCR;
#[doc = "Possible values of the field `src_hwhs_pol`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_HWHS_POLR {
    #[doc = "Active high"]
    ACTIVE_HIGH,
    #[doc = "Active low"]
    ACTIVE_LOW,
}
impl SRC_HWHS_POLR {
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
            SRC_HWHS_POLR::ACTIVE_HIGH => false,
            SRC_HWHS_POLR::ACTIVE_LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRC_HWHS_POLR {
        match value {
            false => SRC_HWHS_POLR::ACTIVE_HIGH,
            true => SRC_HWHS_POLR::ACTIVE_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline]
    pub fn is_active_high(&self) -> bool {
        *self == SRC_HWHS_POLR::ACTIVE_HIGH
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline]
    pub fn is_active_low(&self) -> bool {
        *self == SRC_HWHS_POLR::ACTIVE_LOW
    }
}
#[doc = "Possible values of the field `dst_hwhs_pol`"]
pub type DST_HWHS_POLR = SRC_HWHS_POLR;
#[doc = r" Value of the field"]
pub struct SRC_PERR {
    bits: u8,
}
impl SRC_PERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DST_PERR {
    bits: u8,
}
impl DST_PERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH_PRIORR {
    bits: u8,
}
impl CH_PRIORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LOCK_CHR {
    bits: bool,
}
impl LOCK_CHR {
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
#[doc = "Possible values of the field `lock_ch_l`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_CH_LR {
    #[doc = "Duration of channel is locked for entire DMA transfer"]
    DMA_TRANSFER,
    #[doc = "Duration of channel is locked for current block transfer"]
    BLOCK_TRANSFER,
    #[doc = "Duration of channel is locked for current transaction"]
    TRANSACTION,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LOCK_CH_LR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LOCK_CH_LR::DMA_TRANSFER => 0,
            LOCK_CH_LR::BLOCK_TRANSFER => 1,
            LOCK_CH_LR::TRANSACTION => 2,
            LOCK_CH_LR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LOCK_CH_LR {
        match value {
            0 => LOCK_CH_LR::DMA_TRANSFER,
            1 => LOCK_CH_LR::BLOCK_TRANSFER,
            2 => LOCK_CH_LR::TRANSACTION,
            i => LOCK_CH_LR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_TRANSFER`"]
    #[inline]
    pub fn is_dma_transfer(&self) -> bool {
        *self == LOCK_CH_LR::DMA_TRANSFER
    }
    #[doc = "Checks if the value of the field is `BLOCK_TRANSFER`"]
    #[inline]
    pub fn is_block_transfer(&self) -> bool {
        *self == LOCK_CH_LR::BLOCK_TRANSFER
    }
    #[doc = "Checks if the value of the field is `TRANSACTION`"]
    #[inline]
    pub fn is_transaction(&self) -> bool {
        *self == LOCK_CH_LR::TRANSACTION
    }
}
#[doc = r" Value of the field"]
pub struct SRC_OSR_LMTR {
    bits: u8,
}
impl SRC_OSR_LMTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DST_OSR_LMTR {
    bits: u8,
}
impl DST_OSR_LMTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `src_multblk_type`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_MULTBLK_TYPEW {
    #[doc = "Continuous multi-block type"]
    CONTIGUOUS,
    #[doc = "Reload multi-block type"]
    RELOAD,
    #[doc = "Shadow register based multi-block type"]
    SHADOWREGISTER,
    #[doc = "Linked list based multi-block type"]
    LINKEDLIST,
}
impl SRC_MULTBLK_TYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRC_MULTBLK_TYPEW::CONTIGUOUS => 0,
            SRC_MULTBLK_TYPEW::RELOAD => 1,
            SRC_MULTBLK_TYPEW::SHADOWREGISTER => 2,
            SRC_MULTBLK_TYPEW::LINKEDLIST => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRC_MULTBLK_TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _SRC_MULTBLK_TYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRC_MULTBLK_TYPEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Continuous multi-block type"]
    #[inline]
    pub fn contiguous(self) -> &'a mut W {
        self.variant(SRC_MULTBLK_TYPEW::CONTIGUOUS)
    }
    #[doc = "Reload multi-block type"]
    #[inline]
    pub fn reload(self) -> &'a mut W {
        self.variant(SRC_MULTBLK_TYPEW::RELOAD)
    }
    #[doc = "Shadow register based multi-block type"]
    #[inline]
    pub fn shadowregister(self) -> &'a mut W {
        self.variant(SRC_MULTBLK_TYPEW::SHADOWREGISTER)
    }
    #[doc = "Linked list based multi-block type"]
    #[inline]
    pub fn linkedlist(self) -> &'a mut W {
        self.variant(SRC_MULTBLK_TYPEW::LINKEDLIST)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `dst_multblk_type`"]
pub type DST_MULTBLK_TYPEW = SRC_MULTBLK_TYPEW;
#[doc = r" Proxy"]
pub struct _DST_MULTBLK_TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _DST_MULTBLK_TYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DST_MULTBLK_TYPEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Continuous multi-block type"]
    #[inline]
    pub fn contiguous(self) -> &'a mut W {
        self.variant(SRC_MULTBLK_TYPEW::CONTIGUOUS)
    }
    #[doc = "Reload multi-block type"]
    #[inline]
    pub fn reload(self) -> &'a mut W {
        self.variant(SRC_MULTBLK_TYPEW::RELOAD)
    }
    #[doc = "Shadow register based multi-block type"]
    #[inline]
    pub fn shadowregister(self) -> &'a mut W {
        self.variant(SRC_MULTBLK_TYPEW::SHADOWREGISTER)
    }
    #[doc = "Linked list based multi-block type"]
    #[inline]
    pub fn linkedlist(self) -> &'a mut W {
        self.variant(SRC_MULTBLK_TYPEW::LINKEDLIST)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `tt_fc`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TT_FCW {
    #[doc = "Transfer memory to memory and flow controller is DMAC"]
    MEM2MEM_DMA,
    #[doc = "Transfer memory to peripheral and flow controller is DMAC"]
    MEM2PRF_DMA,
    #[doc = "Transfer peripheral to memory and flow controller is DMAC"]
    PRF2MEM_DMA,
    #[doc = "Transfer peripheral to peripheral and flow controller is DMAC"]
    PRF2PRF_DMA,
    #[doc = "Transfer peripheral to memory and flow controller is source peripheral"]
    PRF2MEM_PRF,
    #[doc = "Transfer peripheral to peripheral and flow controller is source peripheral"]
    PRF2PRF_SRCPRF,
    #[doc = "Transfer memory to peripheral and flow controller is destination peripheral"]
    MEM2PRF_PRF,
    #[doc = "Transfer peripheral to peripheral and flow controller is destination peripheral"]
    PRF2PRF_DSTPRF,
}
impl TT_FCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TT_FCW::MEM2MEM_DMA => 0,
            TT_FCW::MEM2PRF_DMA => 1,
            TT_FCW::PRF2MEM_DMA => 2,
            TT_FCW::PRF2PRF_DMA => 3,
            TT_FCW::PRF2MEM_PRF => 4,
            TT_FCW::PRF2PRF_SRCPRF => 5,
            TT_FCW::MEM2PRF_PRF => 6,
            TT_FCW::PRF2PRF_DSTPRF => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TT_FCW<'a> {
    w: &'a mut W,
}
impl<'a> _TT_FCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TT_FCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Transfer memory to memory and flow controller is DMAC"]
    #[inline]
    pub fn mem2mem_dma(self) -> &'a mut W {
        self.variant(TT_FCW::MEM2MEM_DMA)
    }
    #[doc = "Transfer memory to peripheral and flow controller is DMAC"]
    #[inline]
    pub fn mem2prf_dma(self) -> &'a mut W {
        self.variant(TT_FCW::MEM2PRF_DMA)
    }
    #[doc = "Transfer peripheral to memory and flow controller is DMAC"]
    #[inline]
    pub fn prf2mem_dma(self) -> &'a mut W {
        self.variant(TT_FCW::PRF2MEM_DMA)
    }
    #[doc = "Transfer peripheral to peripheral and flow controller is DMAC"]
    #[inline]
    pub fn prf2prf_dma(self) -> &'a mut W {
        self.variant(TT_FCW::PRF2PRF_DMA)
    }
    #[doc = "Transfer peripheral to memory and flow controller is source peripheral"]
    #[inline]
    pub fn prf2mem_prf(self) -> &'a mut W {
        self.variant(TT_FCW::PRF2MEM_PRF)
    }
    #[doc = "Transfer peripheral to peripheral and flow controller is source peripheral"]
    #[inline]
    pub fn prf2prf_srcprf(self) -> &'a mut W {
        self.variant(TT_FCW::PRF2PRF_SRCPRF)
    }
    #[doc = "Transfer memory to peripheral and flow controller is destination peripheral"]
    #[inline]
    pub fn mem2prf_prf(self) -> &'a mut W {
        self.variant(TT_FCW::MEM2PRF_PRF)
    }
    #[doc = "Transfer peripheral to peripheral and flow controller is destination peripheral"]
    #[inline]
    pub fn prf2prf_dstprf(self) -> &'a mut W {
        self.variant(TT_FCW::PRF2PRF_DSTPRF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 32;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `hs_sel_src`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_SEL_SRCW {
    #[doc = "Hardware handshaking is used"]
    HARDWARE,
    #[doc = "Software handshaking is used"]
    SOFTWARE,
}
impl HS_SEL_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HS_SEL_SRCW::HARDWARE => false,
            HS_SEL_SRCW::SOFTWARE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HS_SEL_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _HS_SEL_SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HS_SEL_SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware handshaking is used"]
    #[inline]
    pub fn hardware(self) -> &'a mut W {
        self.variant(HS_SEL_SRCW::HARDWARE)
    }
    #[doc = "Software handshaking is used"]
    #[inline]
    pub fn software(self) -> &'a mut W {
        self.variant(HS_SEL_SRCW::SOFTWARE)
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
        const OFFSET: u8 = 35;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `hs_sel_dst`"]
pub type HS_SEL_DSTW = HS_SEL_SRCW;
#[doc = r" Proxy"]
pub struct _HS_SEL_DSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HS_SEL_DSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HS_SEL_DSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware handshaking is used"]
    #[inline]
    pub fn hardware(self) -> &'a mut W {
        self.variant(HS_SEL_SRCW::HARDWARE)
    }
    #[doc = "Software handshaking is used"]
    #[inline]
    pub fn software(self) -> &'a mut W {
        self.variant(HS_SEL_SRCW::SOFTWARE)
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
        const OFFSET: u8 = 36;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `src_hwhs_pol`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_HWHS_POLW {
    #[doc = "Active high"]
    ACTIVE_HIGH,
    #[doc = "Active low"]
    ACTIVE_LOW,
}
impl SRC_HWHS_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRC_HWHS_POLW::ACTIVE_HIGH => false,
            SRC_HWHS_POLW::ACTIVE_LOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRC_HWHS_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _SRC_HWHS_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRC_HWHS_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active high"]
    #[inline]
    pub fn active_high(self) -> &'a mut W {
        self.variant(SRC_HWHS_POLW::ACTIVE_HIGH)
    }
    #[doc = "Active low"]
    #[inline]
    pub fn active_low(self) -> &'a mut W {
        self.variant(SRC_HWHS_POLW::ACTIVE_LOW)
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
        const OFFSET: u8 = 37;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `dst_hwhs_pol`"]
pub type DST_HWHS_POLW = SRC_HWHS_POLW;
#[doc = r" Proxy"]
pub struct _DST_HWHS_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _DST_HWHS_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DST_HWHS_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active high"]
    #[inline]
    pub fn active_high(self) -> &'a mut W {
        self.variant(SRC_HWHS_POLW::ACTIVE_HIGH)
    }
    #[doc = "Active low"]
    #[inline]
    pub fn active_low(self) -> &'a mut W {
        self.variant(SRC_HWHS_POLW::ACTIVE_LOW)
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
        const OFFSET: u8 = 38;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SRC_PERW<'a> {
    w: &'a mut W,
}
impl<'a> _SRC_PERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 39;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DST_PERW<'a> {
    w: &'a mut W,
}
impl<'a> _DST_PERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 44;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH_PRIORW<'a> {
    w: &'a mut W,
}
impl<'a> _CH_PRIORW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 49;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_CHW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_CHW<'a> {
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
        const OFFSET: u8 = 52;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `lock_ch_l`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_CH_LW {
    #[doc = "Duration of channel is locked for entire DMA transfer"]
    DMA_TRANSFER,
    #[doc = "Duration of channel is locked for current block transfer"]
    BLOCK_TRANSFER,
    #[doc = "Duration of channel is locked for current transaction"]
    TRANSACTION,
}
impl LOCK_CH_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LOCK_CH_LW::DMA_TRANSFER => 0,
            LOCK_CH_LW::BLOCK_TRANSFER => 1,
            LOCK_CH_LW::TRANSACTION => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_CH_LW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_CH_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCK_CH_LW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Duration of channel is locked for entire DMA transfer"]
    #[inline]
    pub fn dma_transfer(self) -> &'a mut W {
        self.variant(LOCK_CH_LW::DMA_TRANSFER)
    }
    #[doc = "Duration of channel is locked for current block transfer"]
    #[inline]
    pub fn block_transfer(self) -> &'a mut W {
        self.variant(LOCK_CH_LW::BLOCK_TRANSFER)
    }
    #[doc = "Duration of channel is locked for current transaction"]
    #[inline]
    pub fn transaction(self) -> &'a mut W {
        self.variant(LOCK_CH_LW::TRANSACTION)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 53;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SRC_OSR_LMTW<'a> {
    w: &'a mut W,
}
impl<'a> _SRC_OSR_LMTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 55;
        self.w.bits &= !((MASK as u64) << OFFSET);
        self.w.bits |= ((value & MASK) as u64) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DST_OSR_LMTW<'a> {
    w: &'a mut W,
}
impl<'a> _DST_OSR_LMTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 59;
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
    #[doc = "Bits 0:1 - Source multi-block transfer type"]
    #[inline]
    pub fn src_multblk_type(&self) -> SRC_MULTBLK_TYPER {
        SRC_MULTBLK_TYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u64) as u8
        })
    }
    #[doc = "Bits 2:3 - Destination multi-block transfer type"]
    #[inline]
    pub fn dst_multblk_type(&self) -> DST_MULTBLK_TYPER {
        DST_MULTBLK_TYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u64) as u8
        })
    }
    #[doc = "Bits 32:34 - Transfer type and flow control"]
    #[inline]
    pub fn tt_fc(&self) -> TT_FCR {
        TT_FCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 32;
            ((self.bits >> OFFSET) & MASK as u64) as u8
        })
    }
    #[doc = "Bit 35 - Source software or hardware handshaking select"]
    #[inline]
    pub fn hs_sel_src(&self) -> HS_SEL_SRCR {
        HS_SEL_SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 35;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        })
    }
    #[doc = "Bit 36 - Destination software or hardware handshaking select"]
    #[inline]
    pub fn hs_sel_dst(&self) -> HS_SEL_DSTR {
        HS_SEL_DSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 36;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        })
    }
    #[doc = "Bit 37 - Source hardware handshaking interface polarity"]
    #[inline]
    pub fn src_hwhs_pol(&self) -> SRC_HWHS_POLR {
        SRC_HWHS_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 37;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        })
    }
    #[doc = "Bit 38 - Destination hardware handshaking interface polarity"]
    #[inline]
    pub fn dst_hwhs_pol(&self) -> DST_HWHS_POLR {
        DST_HWHS_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 38;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        })
    }
    #[doc = "Bits 39:42 - Assign a hardware handshaking interface to source of channel"]
    #[inline]
    pub fn src_per(&self) -> SRC_PERR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 39;
            ((self.bits >> OFFSET) & MASK as u64) as u8
        };
        SRC_PERR { bits }
    }
    #[doc = "Bits 44:47 - Assign a hardware handshaking interface to destination of channel"]
    #[inline]
    pub fn dst_per(&self) -> DST_PERR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 44;
            ((self.bits >> OFFSET) & MASK as u64) as u8
        };
        DST_PERR { bits }
    }
    #[doc = "Bits 49:51 - Channel priority (7 is highest, 0 is lowest)"]
    #[inline]
    pub fn ch_prior(&self) -> CH_PRIORR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 49;
            ((self.bits >> OFFSET) & MASK as u64) as u8
        };
        CH_PRIORR { bits }
    }
    #[doc = "Bit 52 - Channel lock bit"]
    #[inline]
    pub fn lock_ch(&self) -> LOCK_CHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 52;
            ((self.bits >> OFFSET) & MASK as u64) != 0
        };
        LOCK_CHR { bits }
    }
    #[doc = "Bits 53:54 - Channel lock level"]
    #[inline]
    pub fn lock_ch_l(&self) -> LOCK_CH_LR {
        LOCK_CH_LR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 53;
            ((self.bits >> OFFSET) & MASK as u64) as u8
        })
    }
    #[doc = "Bits 55:58 - Source outstanding request limit"]
    #[inline]
    pub fn src_osr_lmt(&self) -> SRC_OSR_LMTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 55;
            ((self.bits >> OFFSET) & MASK as u64) as u8
        };
        SRC_OSR_LMTR { bits }
    }
    #[doc = "Bits 59:62 - Destination outstanding request limit"]
    #[inline]
    pub fn dst_osr_lmt(&self) -> DST_OSR_LMTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 59;
            ((self.bits >> OFFSET) & MASK as u64) as u8
        };
        DST_OSR_LMTR { bits }
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
    #[doc = "Bits 0:1 - Source multi-block transfer type"]
    #[inline]
    pub fn src_multblk_type(&mut self) -> _SRC_MULTBLK_TYPEW {
        _SRC_MULTBLK_TYPEW { w: self }
    }
    #[doc = "Bits 2:3 - Destination multi-block transfer type"]
    #[inline]
    pub fn dst_multblk_type(&mut self) -> _DST_MULTBLK_TYPEW {
        _DST_MULTBLK_TYPEW { w: self }
    }
    #[doc = "Bits 32:34 - Transfer type and flow control"]
    #[inline]
    pub fn tt_fc(&mut self) -> _TT_FCW {
        _TT_FCW { w: self }
    }
    #[doc = "Bit 35 - Source software or hardware handshaking select"]
    #[inline]
    pub fn hs_sel_src(&mut self) -> _HS_SEL_SRCW {
        _HS_SEL_SRCW { w: self }
    }
    #[doc = "Bit 36 - Destination software or hardware handshaking select"]
    #[inline]
    pub fn hs_sel_dst(&mut self) -> _HS_SEL_DSTW {
        _HS_SEL_DSTW { w: self }
    }
    #[doc = "Bit 37 - Source hardware handshaking interface polarity"]
    #[inline]
    pub fn src_hwhs_pol(&mut self) -> _SRC_HWHS_POLW {
        _SRC_HWHS_POLW { w: self }
    }
    #[doc = "Bit 38 - Destination hardware handshaking interface polarity"]
    #[inline]
    pub fn dst_hwhs_pol(&mut self) -> _DST_HWHS_POLW {
        _DST_HWHS_POLW { w: self }
    }
    #[doc = "Bits 39:42 - Assign a hardware handshaking interface to source of channel"]
    #[inline]
    pub fn src_per(&mut self) -> _SRC_PERW {
        _SRC_PERW { w: self }
    }
    #[doc = "Bits 44:47 - Assign a hardware handshaking interface to destination of channel"]
    #[inline]
    pub fn dst_per(&mut self) -> _DST_PERW {
        _DST_PERW { w: self }
    }
    #[doc = "Bits 49:51 - Channel priority (7 is highest, 0 is lowest)"]
    #[inline]
    pub fn ch_prior(&mut self) -> _CH_PRIORW {
        _CH_PRIORW { w: self }
    }
    #[doc = "Bit 52 - Channel lock bit"]
    #[inline]
    pub fn lock_ch(&mut self) -> _LOCK_CHW {
        _LOCK_CHW { w: self }
    }
    #[doc = "Bits 53:54 - Channel lock level"]
    #[inline]
    pub fn lock_ch_l(&mut self) -> _LOCK_CH_LW {
        _LOCK_CH_LW { w: self }
    }
    #[doc = "Bits 55:58 - Source outstanding request limit"]
    #[inline]
    pub fn src_osr_lmt(&mut self) -> _SRC_OSR_LMTW {
        _SRC_OSR_LMTW { w: self }
    }
    #[doc = "Bits 59:62 - Destination outstanding request limit"]
    #[inline]
    pub fn dst_osr_lmt(&mut self) -> _DST_OSR_LMTW {
        _DST_OSR_LMTW { w: self }
    }
}
