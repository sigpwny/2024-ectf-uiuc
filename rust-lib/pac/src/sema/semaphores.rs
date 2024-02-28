#[doc = "Register `SEMAPHORES[%s]` reader"]
pub type R = crate::R<SEMAPHORES_SPEC>;
#[doc = "Register `SEMAPHORES[%s]` writer"]
pub type W = crate::W<SEMAPHORES_SPEC>;
#[doc = "Field `sema` reader - "]
pub type SEMA_R = crate::BitReader;
#[doc = "Field `sema` writer - "]
pub type SEMA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sema(&self) -> SEMA_R {
        SEMA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sema(&mut self) -> SEMA_W<SEMAPHORES_SPEC> {
        SEMA_W::new(self, 0)
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
#[doc = "Read to test and set, returns prior value. Write 0 to clear semaphore.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`semaphores::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`semaphores::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEMAPHORES_SPEC;
impl crate::RegisterSpec for SEMAPHORES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`semaphores::R`](R) reader structure"]
impl crate::Readable for SEMAPHORES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`semaphores::W`](W) writer structure"]
impl crate::Writable for SEMAPHORES_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEMAPHORES[%s]
to value 0"]
impl crate::Resettable for SEMAPHORES_SPEC {
    const RESET_VALUE: u32 = 0;
}
