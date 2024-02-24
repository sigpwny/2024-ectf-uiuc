#[doc = "Register `URVCTRL` reader"]
pub type R = crate::R<URVCTRL_SPEC>;
#[doc = "Register `URVCTRL` writer"]
pub type W = crate::W<URVCTRL_SPEC>;
#[doc = "Field `MEMSEL` reader - RAM2, RAM3 exclusive ownership."]
pub type MEMSEL_R = crate::BitReader;
#[doc = "Field `MEMSEL` writer - RAM2, RAM3 exclusive ownership."]
pub type MEMSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFLUSHEN` reader - URV instruction flush enable."]
pub type IFLUSHEN_R = crate::BitReader;
#[doc = "Field `IFLUSHEN` writer - URV instruction flush enable."]
pub type IFLUSHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RAM2, RAM3 exclusive ownership."]
    #[inline(always)]
    pub fn memsel(&self) -> MEMSEL_R {
        MEMSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - URV instruction flush enable."]
    #[inline(always)]
    pub fn iflushen(&self) -> IFLUSHEN_R {
        IFLUSHEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM2, RAM3 exclusive ownership."]
    #[inline(always)]
    #[must_use]
    pub fn memsel(&mut self) -> MEMSEL_W<URVCTRL_SPEC> {
        MEMSEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - URV instruction flush enable."]
    #[inline(always)]
    #[must_use]
    pub fn iflushen(&mut self) -> IFLUSHEN_W<URVCTRL_SPEC> {
        IFLUSHEN_W::new(self, 1)
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
#[doc = "RISC-V Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`urvctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`urvctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct URVCTRL_SPEC;
impl crate::RegisterSpec for URVCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`urvctrl::R`](R) reader structure"]
impl crate::Readable for URVCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`urvctrl::W`](W) writer structure"]
impl crate::Writable for URVCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets URVCTRL to value 0"]
impl crate::Resettable for URVCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
