#[doc = "Register `PNR` reader"]
pub type R = crate::R<PNR_SPEC>;
#[doc = "Register `PNR` writer"]
pub type W = crate::W<PNR_SPEC>;
#[doc = "Field `CTS` reader - Current sampled value of CTS IO"]
pub type CTS_R = crate::BitReader;
#[doc = "Field `RTS` reader - This bit controls the value to apply on the RTS IO. If set to 1, the RTS IO is set to high level. If set to 0, the RTS IO is set to low level."]
pub type RTS_R = crate::BitReader;
#[doc = "Field `RTS` writer - This bit controls the value to apply on the RTS IO. If set to 1, the RTS IO is set to high level. If set to 0, the RTS IO is set to low level."]
pub type RTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Current sampled value of CTS IO"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit controls the value to apply on the RTS IO. If set to 1, the RTS IO is set to high level. If set to 0, the RTS IO is set to low level."]
    #[inline(always)]
    pub fn rts(&self) -> RTS_R {
        RTS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - This bit controls the value to apply on the RTS IO. If set to 1, the RTS IO is set to high level. If set to 0, the RTS IO is set to low level."]
    #[inline(always)]
    #[must_use]
    pub fn rts(&mut self) -> RTS_W<PNR_SPEC> {
        RTS_W::new(self, 1)
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
#[doc = "Pin register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pnr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pnr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PNR_SPEC;
impl crate::RegisterSpec for PNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pnr::R`](R) reader structure"]
impl crate::Readable for PNR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pnr::W`](W) writer structure"]
impl crate::Writable for PNR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PNR to value 0"]
impl crate::Resettable for PNR_SPEC {
    const RESET_VALUE: u32 = 0;
}
