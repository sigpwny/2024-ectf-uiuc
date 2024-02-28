#[doc = "Register `RELOAD` reader"]
pub type R = crate::R<RELOAD_SPEC>;
#[doc = "Register `RELOAD` writer"]
pub type W = crate::W<RELOAD_SPEC>;
#[doc = "Field `RELOAD` reader - Rerload Value."]
pub type RELOAD_R = crate::FieldReader<u32>;
#[doc = "Field `RELOAD` writer - Rerload Value."]
pub type RELOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Rerload Value."]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Rerload Value."]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<RELOAD_SPEC> {
        RELOAD_W::new(self, 0)
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
#[doc = "Reload register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reload::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reload::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RELOAD_SPEC;
impl crate::RegisterSpec for RELOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reload::R`](R) reader structure"]
impl crate::Readable for RELOAD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reload::W`](W) writer structure"]
impl crate::Writable for RELOAD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RELOAD to value 0"]
impl crate::Resettable for RELOAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
