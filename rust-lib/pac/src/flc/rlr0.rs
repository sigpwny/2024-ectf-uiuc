#[doc = "Register `RLR0` reader"]
pub type R = crate::R<RLR0_SPEC>;
#[doc = "Register `RLR0` writer"]
pub type W = crate::W<RLR0_SPEC>;
#[doc = "Field `RLR0` reader - Access control."]
pub type RLR0_R = crate::FieldReader<u32>;
#[doc = "Field `RLR0` writer - Access control."]
pub type RLR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn rlr0(&self) -> RLR0_R {
        RLR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    #[must_use]
    pub fn rlr0(&mut self) -> RLR0_W<RLR0_SPEC> {
        RLR0_W::new(self, 0)
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
#[doc = "RLR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rlr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rlr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RLR0_SPEC;
impl crate::RegisterSpec for RLR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rlr0::R`](R) reader structure"]
impl crate::Readable for RLR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rlr0::W`](W) writer structure"]
impl crate::Writable for RLR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RLR0 to value 0"]
impl crate::Resettable for RLR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
