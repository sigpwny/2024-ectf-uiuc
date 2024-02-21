#[doc = "Register `CLKSEL` reader"]
pub type R = crate::R<CLKSEL_SPEC>;
#[doc = "Register `CLKSEL` writer"]
pub type W = crate::W<CLKSEL_SPEC>;
#[doc = "Field `SOURCE` reader - WWDT Clock Selection Register."]
pub type SOURCE_R = crate::FieldReader;
#[doc = "Field `SOURCE` writer - WWDT Clock Selection Register."]
pub type SOURCE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - WWDT Clock Selection Register."]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - WWDT Clock Selection Register."]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SOURCE_W<CLKSEL_SPEC> {
        SOURCE_W::new(self, 0)
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
#[doc = "Windowed Watchdog Timer Clock Select Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKSEL_SPEC;
impl crate::RegisterSpec for CLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel::R`](R) reader structure"]
impl crate::Readable for CLKSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clksel::W`](W) writer structure"]
impl crate::Writable for CLKSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL to value 0"]
impl crate::Resettable for CLKSEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
