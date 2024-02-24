#[doc = "Register `SSEC` reader"]
pub type R = crate::R<SSEC_SPEC>;
#[doc = "Register `SSEC` writer"]
pub type W = crate::W<SSEC_SPEC>;
#[doc = "Field `SSEC` reader - Sub-Seconds Counter (12-bit)."]
pub type SSEC_R = crate::FieldReader<u16>;
#[doc = "Field `SSEC` writer - Sub-Seconds Counter (12-bit)."]
pub type SSEC_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Sub-Seconds Counter (12-bit)."]
    #[inline(always)]
    pub fn ssec(&self) -> SSEC_R {
        SSEC_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Sub-Seconds Counter (12-bit)."]
    #[inline(always)]
    #[must_use]
    pub fn ssec(&mut self) -> SSEC_W<SSEC_SPEC> {
        SSEC_W::new(self, 0)
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
#[doc = "RTC Sub-second Counter. This counter increments at 256Hz. RTC_SEC is incremented when this register rolls over from 0xFF to 0x00.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSEC_SPEC;
impl crate::RegisterSpec for SSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssec::R`](R) reader structure"]
impl crate::Readable for SSEC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssec::W`](W) writer structure"]
impl crate::Writable for SSEC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSEC to value 0"]
impl crate::Resettable for SSEC_SPEC {
    const RESET_VALUE: u32 = 0;
}
