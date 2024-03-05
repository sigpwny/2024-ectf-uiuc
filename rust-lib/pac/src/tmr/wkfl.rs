#[doc = "Register `WKFL` reader"]
pub type R = crate::R<WKFL_SPEC>;
#[doc = "Register `WKFL` writer"]
pub type W = crate::W<WKFL_SPEC>;
#[doc = "Field `A` reader - Wake-Up Flag for Timer A"]
pub type A_R = crate::BitReader;
#[doc = "Field `A` writer - Wake-Up Flag for Timer A"]
pub type A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B` reader - Wake-Up Flag for Timer B"]
pub type B_R = crate::BitReader;
#[doc = "Field `B` writer - Wake-Up Flag for Timer B"]
pub type B_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wake-Up Flag for Timer A"]
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Wake-Up Flag for Timer B"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-Up Flag for Timer A"]
    #[inline(always)]
    #[must_use]
    pub fn a(&mut self) -> A_W<WKFL_SPEC> {
        A_W::new(self, 0)
    }
    #[doc = "Bit 16 - Wake-Up Flag for Timer B"]
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> B_W<WKFL_SPEC> {
        B_W::new(self, 16)
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
#[doc = "Timer Wakeup Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wkfl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wkfl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WKFL_SPEC;
impl crate::RegisterSpec for WKFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wkfl::R`](R) reader structure"]
impl crate::Readable for WKFL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wkfl::W`](W) writer structure"]
impl crate::Writable for WKFL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WKFL to value 0"]
impl crate::Resettable for WKFL_SPEC {
    const RESET_VALUE: u32 = 0;
}
