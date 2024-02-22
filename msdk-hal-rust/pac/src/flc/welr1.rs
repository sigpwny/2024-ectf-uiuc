#[doc = "Register `WELR1` reader"]
pub type R = crate::R<WELR1_SPEC>;
#[doc = "Register `WELR1` writer"]
pub type W = crate::W<WELR1_SPEC>;
#[doc = "Field `WELR1` reader - Access control."]
pub type WELR1_R = crate::FieldReader<u32>;
#[doc = "Field `WELR1` writer - Access control."]
pub type WELR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn welr1(&self) -> WELR1_R {
        WELR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    #[must_use]
    pub fn welr1(&mut self) -> WELR1_W<WELR1_SPEC> {
        WELR1_W::new(self, 0)
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
#[doc = "WELR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`welr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`welr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WELR1_SPEC;
impl crate::RegisterSpec for WELR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`welr1::R`](R) reader structure"]
impl crate::Readable for WELR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`welr1::W`](W) writer structure"]
impl crate::Writable for WELR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WELR1 to value 0"]
impl crate::Resettable for WELR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
