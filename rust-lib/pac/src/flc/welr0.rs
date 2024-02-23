#[doc = "Register `WELR0` reader"]
pub type R = crate::R<WELR0_SPEC>;
#[doc = "Register `WELR0` writer"]
pub type W = crate::W<WELR0_SPEC>;
#[doc = "Field `WELR0` reader - Access control."]
pub type WELR0_R = crate::FieldReader<u32>;
#[doc = "Field `WELR0` writer - Access control."]
pub type WELR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn welr0(&self) -> WELR0_R {
        WELR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    #[must_use]
    pub fn welr0(&mut self) -> WELR0_W<WELR0_SPEC> {
        WELR0_W::new(self, 0)
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
#[doc = "WELR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`welr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`welr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WELR0_SPEC;
impl crate::RegisterSpec for WELR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`welr0::R`](R) reader structure"]
impl crate::Readable for WELR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`welr0::W`](W) writer structure"]
impl crate::Writable for WELR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WELR0 to value 0"]
impl crate::Resettable for WELR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
