#[doc = "Register `OSR` reader"]
pub type R = crate::R<OSR_SPEC>;
#[doc = "Register `OSR` writer"]
pub type W = crate::W<OSR_SPEC>;
#[doc = "Field `OSR` reader - OSR"]
pub type OSR_R = crate::FieldReader;
#[doc = "Field `OSR` writer - OSR"]
pub type OSR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - OSR"]
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - OSR"]
    #[inline(always)]
    #[must_use]
    pub fn osr(&mut self) -> OSR_W<OSR_SPEC> {
        OSR_W::new(self, 0)
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
#[doc = "Over Sampling Rate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSR_SPEC;
impl crate::RegisterSpec for OSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osr::R`](R) reader structure"]
impl crate::Readable for OSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osr::W`](W) writer structure"]
impl crate::Writable for OSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSR to value 0"]
impl crate::Resettable for OSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
