#[doc = "Register `MAXTON` reader"]
pub type R = crate::R<MAXTON_SPEC>;
#[doc = "Register `MAXTON` writer"]
pub type W = crate::W<MAXTON_SPEC>;
#[doc = "Field `TONSET` reader - Sets the maximum on time for the high side FET, each increment represents 500ns"]
pub type TONSET_R = crate::FieldReader;
#[doc = "Field `TONSET` writer - Sets the maximum on time for the high side FET, each increment represents 500ns"]
pub type TONSET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Sets the maximum on time for the high side FET, each increment represents 500ns"]
    #[inline(always)]
    pub fn tonset(&self) -> TONSET_R {
        TONSET_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sets the maximum on time for the high side FET, each increment represents 500ns"]
    #[inline(always)]
    #[must_use]
    pub fn tonset(&mut self) -> TONSET_W<MAXTON_SPEC> {
        TONSET_W::new(self, 0)
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
#[doc = "Maximum High Side FET Time On Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxton::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maxton::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAXTON_SPEC;
impl crate::RegisterSpec for MAXTON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxton::R`](R) reader structure"]
impl crate::Readable for MAXTON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maxton::W`](W) writer structure"]
impl crate::Writable for MAXTON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAXTON to value 0"]
impl crate::Resettable for MAXTON_SPEC {
    const RESET_VALUE: u32 = 0;
}
