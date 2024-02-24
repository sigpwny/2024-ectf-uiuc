#[doc = "Register `mail0` reader"]
pub type R = crate::R<MAIL0_SPEC>;
#[doc = "Register `mail0` writer"]
pub type W = crate::W<MAIL0_SPEC>;
#[doc = "Field `data` reader - "]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `data` writer - "]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<MAIL0_SPEC> {
        DATA_W::new(self, 0)
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
#[doc = "Semaphore Mailbox 0 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mail0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mail0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAIL0_SPEC;
impl crate::RegisterSpec for MAIL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mail0::R`](R) reader structure"]
impl crate::Readable for MAIL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mail0::W`](W) writer structure"]
impl crate::Writable for MAIL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mail0 to value 0"]
impl crate::Resettable for MAIL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
