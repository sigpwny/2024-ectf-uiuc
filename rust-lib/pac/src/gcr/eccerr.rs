#[doc = "Register `ECCERR` reader"]
pub type R = crate::R<ECCERR_SPEC>;
#[doc = "Register `ECCERR` writer"]
pub type W = crate::W<ECCERR_SPEC>;
#[doc = "Field `RAM` reader - ECC System RAM0 Error Flag. Write 1 to clear."]
pub type RAM_R = crate::BitReader;
#[doc = "Field `RAM` writer - ECC System RAM0 Error Flag. Write 1 to clear."]
pub type RAM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ECC System RAM0 Error Flag. Write 1 to clear."]
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC System RAM0 Error Flag. Write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn ram(&mut self) -> RAM_W<ECCERR_SPEC> {
        RAM_W::new(self, 0)
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
#[doc = "ECC Error Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccerr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccerr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCERR_SPEC;
impl crate::RegisterSpec for ECCERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccerr::R`](R) reader structure"]
impl crate::Readable for ECCERR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eccerr::W`](W) writer structure"]
impl crate::Writable for ECCERR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCERR to value 0"]
impl crate::Resettable for ECCERR_SPEC {
    const RESET_VALUE: u32 = 0;
}
