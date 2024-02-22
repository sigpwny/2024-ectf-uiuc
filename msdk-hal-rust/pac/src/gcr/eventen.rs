#[doc = "Register `EVENTEN` reader"]
pub type R = crate::R<EVENTEN_SPEC>;
#[doc = "Register `EVENTEN` writer"]
pub type W = crate::W<EVENTEN_SPEC>;
#[doc = "Field `DMA` reader - Enable DMA event. When this bit is set, a DMA event will cause an RXEV event to wake the CPU from WFE sleep mode."]
pub type DMA_R = crate::BitReader;
#[doc = "Field `DMA` writer - Enable DMA event. When this bit is set, a DMA event will cause an RXEV event to wake the CPU from WFE sleep mode."]
pub type DMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX` reader - Enable RXEV pin event. When this bit is set, a logic high of GPIO1.8 will cause an RXEV event to wake the CPU from WFE sleep mode."]
pub type RX_R = crate::BitReader;
#[doc = "Field `RX` writer - Enable RXEV pin event. When this bit is set, a logic high of GPIO1.8 will cause an RXEV event to wake the CPU from WFE sleep mode."]
pub type RX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX` reader - Enable TXEV pin event. When this bit is set, TXEV event from the CPU is output to GPIO1.9."]
pub type TX_R = crate::BitReader;
#[doc = "Field `TX` writer - Enable TXEV pin event. When this bit is set, TXEV event from the CPU is output to GPIO1.9."]
pub type TX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable DMA event. When this bit is set, a DMA event will cause an RXEV event to wake the CPU from WFE sleep mode."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable RXEV pin event. When this bit is set, a logic high of GPIO1.8 will cause an RXEV event to wake the CPU from WFE sleep mode."]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable TXEV pin event. When this bit is set, TXEV event from the CPU is output to GPIO1.9."]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DMA event. When this bit is set, a DMA event will cause an RXEV event to wake the CPU from WFE sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<EVENTEN_SPEC> {
        DMA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable RXEV pin event. When this bit is set, a logic high of GPIO1.8 will cause an RXEV event to wake the CPU from WFE sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RX_W<EVENTEN_SPEC> {
        RX_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable TXEV pin event. When this bit is set, TXEV event from the CPU is output to GPIO1.9."]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<EVENTEN_SPEC> {
        TX_W::new(self, 2)
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
#[doc = "Event Enable Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eventen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eventen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVENTEN_SPEC;
impl crate::RegisterSpec for EVENTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eventen::R`](R) reader structure"]
impl crate::Readable for EVENTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eventen::W`](W) writer structure"]
impl crate::Writable for EVENTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTEN to value 0"]
impl crate::Resettable for EVENTEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
