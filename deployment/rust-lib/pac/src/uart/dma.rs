#[doc = "Register `DMA` reader"]
pub type R = crate::R<DMA_SPEC>;
#[doc = "Register `DMA` writer"]
pub type W = crate::W<DMA_SPEC>;
#[doc = "Field `TX_THD_VAL` reader - TX FIFO Level DMA Trigger If the TX FIFO level is less than this value, then the TX FIFO DMA interface will send a signal to system DMA to notify that TX FIFO is ready to receive data from memory."]
pub type TX_THD_VAL_R = crate::FieldReader;
#[doc = "Field `TX_THD_VAL` writer - TX FIFO Level DMA Trigger If the TX FIFO level is less than this value, then the TX FIFO DMA interface will send a signal to system DMA to notify that TX FIFO is ready to receive data from memory."]
pub type TX_THD_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TX_EN` reader - TX DMA channel enable"]
pub type TX_EN_R = crate::BitReader;
#[doc = "Field `TX_EN` writer - TX DMA channel enable"]
pub type TX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_THD_VAL` reader - Rx FIFO Level DMA Trigger If the RX FIFO level is greater than this value, then the RX FIFO DMA interface will send a signal to the system DMA to notify that RX FIFO has characters to transfer to memory."]
pub type RX_THD_VAL_R = crate::FieldReader;
#[doc = "Field `RX_THD_VAL` writer - Rx FIFO Level DMA Trigger If the RX FIFO level is greater than this value, then the RX FIFO DMA interface will send a signal to the system DMA to notify that RX FIFO has characters to transfer to memory."]
pub type RX_THD_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RX_EN` reader - RX DMA channel enable"]
pub type RX_EN_R = crate::BitReader;
#[doc = "Field `RX_EN` writer - RX DMA channel enable"]
pub type RX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - TX FIFO Level DMA Trigger If the TX FIFO level is less than this value, then the TX FIFO DMA interface will send a signal to system DMA to notify that TX FIFO is ready to receive data from memory."]
    #[inline(always)]
    pub fn tx_thd_val(&self) -> TX_THD_VAL_R {
        TX_THD_VAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - TX DMA channel enable"]
    #[inline(always)]
    pub fn tx_en(&self) -> TX_EN_R {
        TX_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - Rx FIFO Level DMA Trigger If the RX FIFO level is greater than this value, then the RX FIFO DMA interface will send a signal to the system DMA to notify that RX FIFO has characters to transfer to memory."]
    #[inline(always)]
    pub fn rx_thd_val(&self) -> RX_THD_VAL_R {
        RX_THD_VAL_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - RX DMA channel enable"]
    #[inline(always)]
    pub fn rx_en(&self) -> RX_EN_R {
        RX_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - TX FIFO Level DMA Trigger If the TX FIFO level is less than this value, then the TX FIFO DMA interface will send a signal to system DMA to notify that TX FIFO is ready to receive data from memory."]
    #[inline(always)]
    #[must_use]
    pub fn tx_thd_val(&mut self) -> TX_THD_VAL_W<DMA_SPEC> {
        TX_THD_VAL_W::new(self, 0)
    }
    #[doc = "Bit 4 - TX DMA channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_en(&mut self) -> TX_EN_W<DMA_SPEC> {
        TX_EN_W::new(self, 4)
    }
    #[doc = "Bits 5:8 - Rx FIFO Level DMA Trigger If the RX FIFO level is greater than this value, then the RX FIFO DMA interface will send a signal to the system DMA to notify that RX FIFO has characters to transfer to memory."]
    #[inline(always)]
    #[must_use]
    pub fn rx_thd_val(&mut self) -> RX_THD_VAL_W<DMA_SPEC> {
        RX_THD_VAL_W::new(self, 5)
    }
    #[doc = "Bit 9 - RX DMA channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx_en(&mut self) -> RX_EN_W<DMA_SPEC> {
        RX_EN_W::new(self, 9)
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
#[doc = "DMA Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
