#[doc = "Register `DMA` reader"]
pub type R = crate::R<DMA_SPEC>;
#[doc = "Register `DMA` writer"]
pub type W = crate::W<DMA_SPEC>;
#[doc = "Field `TX_THD_VAL` reader - Transmit FIFO level that will trigger a DMA request, also level for threshold status. When TX FIFO has fewer than this many bytes, the associated events and conditions are triggered."]
pub type TX_THD_VAL_R = crate::FieldReader;
#[doc = "Field `TX_THD_VAL` writer - Transmit FIFO level that will trigger a DMA request, also level for threshold status. When TX FIFO has fewer than this many bytes, the associated events and conditions are triggered."]
pub type TX_THD_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TX_FIFO_EN` reader - Transmit FIFO enabled for SPI transactions."]
pub type TX_FIFO_EN_R = crate::BitReader<TX_FIFO_EN_A>;
#[doc = "Transmit FIFO enabled for SPI transactions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_FIFO_EN_A {
    #[doc = "0: Transmit FIFO is not enabled."]
    DIS = 0,
    #[doc = "1: Transmit FIFO is enabled."]
    EN = 1,
}
impl From<TX_FIFO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_FIFO_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_FIFO_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_FIFO_EN_A {
        match self.bits {
            false => TX_FIFO_EN_A::DIS,
            true => TX_FIFO_EN_A::EN,
        }
    }
    #[doc = "Transmit FIFO is not enabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TX_FIFO_EN_A::DIS
    }
    #[doc = "Transmit FIFO is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TX_FIFO_EN_A::EN
    }
}
#[doc = "Field `TX_FIFO_EN` writer - Transmit FIFO enabled for SPI transactions."]
pub type TX_FIFO_EN_W<'a, REG> = crate::BitWriter<'a, REG, TX_FIFO_EN_A>;
impl<'a, REG> TX_FIFO_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit FIFO is not enabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FIFO_EN_A::DIS)
    }
    #[doc = "Transmit FIFO is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FIFO_EN_A::EN)
    }
}
#[doc = "Field `TX_FLUSH` reader - Clear TX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side."]
pub type TX_FLUSH_R = crate::BitReader<TX_FLUSH_A>;
#[doc = "Clear TX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_FLUSH_A {
    #[doc = "1: Clear the Transmit FIFO, clears any pending TX FIFO status."]
    CLEAR = 1,
}
impl From<TX_FLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: TX_FLUSH_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_FLUSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TX_FLUSH_A> {
        match self.bits {
            true => Some(TX_FLUSH_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Clear the Transmit FIFO, clears any pending TX FIFO status."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TX_FLUSH_A::CLEAR
    }
}
#[doc = "Field `TX_FLUSH` writer - Clear TX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side."]
pub type TX_FLUSH_W<'a, REG> = crate::BitWriter<'a, REG, TX_FLUSH_A>;
impl<'a, REG> TX_FLUSH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the Transmit FIFO, clears any pending TX FIFO status."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FLUSH_A::CLEAR)
    }
}
#[doc = "Field `TX_LVL` reader - Count of entries in TX FIFO."]
pub type TX_LVL_R = crate::FieldReader;
#[doc = "Field `DMA_TX_EN` reader - TX DMA Enable."]
pub type DMA_TX_EN_R = crate::BitReader<DMA_TX_EN_A>;
#[doc = "TX DMA Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_TX_EN_A {
    #[doc = "0: TX DMA requests are disabled, andy pending DMA requests are cleared."]
    DIS = 0,
    #[doc = "1: TX DMA requests are enabled."]
    EN = 1,
}
impl From<DMA_TX_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_TX_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_TX_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_TX_EN_A {
        match self.bits {
            false => DMA_TX_EN_A::DIS,
            true => DMA_TX_EN_A::EN,
        }
    }
    #[doc = "TX DMA requests are disabled, andy pending DMA requests are cleared."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DMA_TX_EN_A::DIS
    }
    #[doc = "TX DMA requests are enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DMA_TX_EN_A::EN
    }
}
#[doc = "Field `DMA_TX_EN` writer - TX DMA Enable."]
pub type DMA_TX_EN_W<'a, REG> = crate::BitWriter<'a, REG, DMA_TX_EN_A>;
impl<'a, REG> DMA_TX_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX DMA requests are disabled, andy pending DMA requests are cleared."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_TX_EN_A::DIS)
    }
    #[doc = "TX DMA requests are enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_TX_EN_A::EN)
    }
}
#[doc = "Field `RX_THD_VAL` reader - Receive FIFO level that will trigger a DMA request, also level for threshold status. When RX FIFO has more than this many bytes, the associated events and conditions are triggered."]
pub type RX_THD_VAL_R = crate::FieldReader;
#[doc = "Field `RX_THD_VAL` writer - Receive FIFO level that will trigger a DMA request, also level for threshold status. When RX FIFO has more than this many bytes, the associated events and conditions are triggered."]
pub type RX_THD_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RX_FIFO_EN` reader - Receive FIFO enabled for SPI transactions."]
pub type RX_FIFO_EN_R = crate::BitReader<RX_FIFO_EN_A>;
#[doc = "Receive FIFO enabled for SPI transactions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FIFO_EN_A {
    #[doc = "0: Receive FIFO is not enabled."]
    DIS = 0,
    #[doc = "1: Receive FIFO is enabled."]
    EN = 1,
}
impl From<RX_FIFO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_FIFO_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_FIFO_EN_A {
        match self.bits {
            false => RX_FIFO_EN_A::DIS,
            true => RX_FIFO_EN_A::EN,
        }
    }
    #[doc = "Receive FIFO is not enabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RX_FIFO_EN_A::DIS
    }
    #[doc = "Receive FIFO is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RX_FIFO_EN_A::EN
    }
}
#[doc = "Field `RX_FIFO_EN` writer - Receive FIFO enabled for SPI transactions."]
pub type RX_FIFO_EN_W<'a, REG> = crate::BitWriter<'a, REG, RX_FIFO_EN_A>;
impl<'a, REG> RX_FIFO_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive FIFO is not enabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FIFO_EN_A::DIS)
    }
    #[doc = "Receive FIFO is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FIFO_EN_A::EN)
    }
}
#[doc = "Field `RX_FLUSH` reader - Clear RX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side."]
pub type RX_FLUSH_R = crate::BitReader<RX_FLUSH_A>;
#[doc = "Clear RX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FLUSH_A {
    #[doc = "1: Clear the Receive FIFO, clears any pending RX FIFO status."]
    CLEAR = 1,
}
impl From<RX_FLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FLUSH_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_FLUSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RX_FLUSH_A> {
        match self.bits {
            true => Some(RX_FLUSH_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Clear the Receive FIFO, clears any pending RX FIFO status."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RX_FLUSH_A::CLEAR
    }
}
#[doc = "Field `RX_FLUSH` writer - Clear RX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side."]
pub type RX_FLUSH_W<'a, REG> = crate::BitWriter<'a, REG, RX_FLUSH_A>;
impl<'a, REG> RX_FLUSH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the Receive FIFO, clears any pending RX FIFO status."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FLUSH_A::CLEAR)
    }
}
#[doc = "Field `RX_LVL` reader - Count of entries in RX FIFO."]
pub type RX_LVL_R = crate::FieldReader;
#[doc = "Field `DMA_RX_EN` reader - RX DMA Enable."]
pub type DMA_RX_EN_R = crate::BitReader<DMA_RX_EN_A>;
#[doc = "RX DMA Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_RX_EN_A {
    #[doc = "0: RX DMA requests are disabled, any pending DMA requests are cleared."]
    DIS = 0,
    #[doc = "1: RX DMA requests are enabled."]
    EN = 1,
}
impl From<DMA_RX_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_RX_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_RX_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_RX_EN_A {
        match self.bits {
            false => DMA_RX_EN_A::DIS,
            true => DMA_RX_EN_A::EN,
        }
    }
    #[doc = "RX DMA requests are disabled, any pending DMA requests are cleared."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DMA_RX_EN_A::DIS
    }
    #[doc = "RX DMA requests are enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DMA_RX_EN_A::EN
    }
}
#[doc = "Field `DMA_RX_EN` writer - RX DMA Enable."]
pub type DMA_RX_EN_W<'a, REG> = crate::BitWriter<'a, REG, DMA_RX_EN_A>;
impl<'a, REG> DMA_RX_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX DMA requests are disabled, any pending DMA requests are cleared."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_RX_EN_A::DIS)
    }
    #[doc = "RX DMA requests are enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_RX_EN_A::EN)
    }
}
impl R {
    #[doc = "Bits 0:4 - Transmit FIFO level that will trigger a DMA request, also level for threshold status. When TX FIFO has fewer than this many bytes, the associated events and conditions are triggered."]
    #[inline(always)]
    pub fn tx_thd_val(&self) -> TX_THD_VAL_R {
        TX_THD_VAL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Transmit FIFO enabled for SPI transactions."]
    #[inline(always)]
    pub fn tx_fifo_en(&self) -> TX_FIFO_EN_R {
        TX_FIFO_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clear TX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side."]
    #[inline(always)]
    pub fn tx_flush(&self) -> TX_FLUSH_R {
        TX_FLUSH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Count of entries in TX FIFO."]
    #[inline(always)]
    pub fn tx_lvl(&self) -> TX_LVL_R {
        TX_LVL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - TX DMA Enable."]
    #[inline(always)]
    pub fn dma_tx_en(&self) -> DMA_TX_EN_R {
        DMA_TX_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Receive FIFO level that will trigger a DMA request, also level for threshold status. When RX FIFO has more than this many bytes, the associated events and conditions are triggered."]
    #[inline(always)]
    pub fn rx_thd_val(&self) -> RX_THD_VAL_R {
        RX_THD_VAL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - Receive FIFO enabled for SPI transactions."]
    #[inline(always)]
    pub fn rx_fifo_en(&self) -> RX_FIFO_EN_R {
        RX_FIFO_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Clear RX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side."]
    #[inline(always)]
    pub fn rx_flush(&self) -> RX_FLUSH_R {
        RX_FLUSH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Count of entries in RX FIFO."]
    #[inline(always)]
    pub fn rx_lvl(&self) -> RX_LVL_R {
        RX_LVL_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - RX DMA Enable."]
    #[inline(always)]
    pub fn dma_rx_en(&self) -> DMA_RX_EN_R {
        DMA_RX_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Transmit FIFO level that will trigger a DMA request, also level for threshold status. When TX FIFO has fewer than this many bytes, the associated events and conditions are triggered."]
    #[inline(always)]
    #[must_use]
    pub fn tx_thd_val(&mut self) -> TX_THD_VAL_W<DMA_SPEC> {
        TX_THD_VAL_W::new(self, 0)
    }
    #[doc = "Bit 6 - Transmit FIFO enabled for SPI transactions."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_en(&mut self) -> TX_FIFO_EN_W<DMA_SPEC> {
        TX_FIFO_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear TX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side."]
    #[inline(always)]
    #[must_use]
    pub fn tx_flush(&mut self) -> TX_FLUSH_W<DMA_SPEC> {
        TX_FLUSH_W::new(self, 7)
    }
    #[doc = "Bit 15 - TX DMA Enable."]
    #[inline(always)]
    #[must_use]
    pub fn dma_tx_en(&mut self) -> DMA_TX_EN_W<DMA_SPEC> {
        DMA_TX_EN_W::new(self, 15)
    }
    #[doc = "Bits 16:20 - Receive FIFO level that will trigger a DMA request, also level for threshold status. When RX FIFO has more than this many bytes, the associated events and conditions are triggered."]
    #[inline(always)]
    #[must_use]
    pub fn rx_thd_val(&mut self) -> RX_THD_VAL_W<DMA_SPEC> {
        RX_THD_VAL_W::new(self, 16)
    }
    #[doc = "Bit 22 - Receive FIFO enabled for SPI transactions."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_en(&mut self) -> RX_FIFO_EN_W<DMA_SPEC> {
        RX_FIFO_EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Clear RX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side."]
    #[inline(always)]
    #[must_use]
    pub fn rx_flush(&mut self) -> RX_FLUSH_W<DMA_SPEC> {
        RX_FLUSH_W::new(self, 23)
    }
    #[doc = "Bit 31 - RX DMA Enable."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_en(&mut self) -> DMA_RX_EN_W<DMA_SPEC> {
        DMA_RX_EN_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Register for controlling DMA.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_SPEC;
impl crate::RegisterSpec for DMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma::R`](R) reader structure"]
impl crate::Readable for DMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma::W`](W) writer structure"]
impl crate::Writable for DMA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA to value 0"]
impl crate::Resettable for DMA_SPEC {
    const RESET_VALUE: u32 = 0;
}
