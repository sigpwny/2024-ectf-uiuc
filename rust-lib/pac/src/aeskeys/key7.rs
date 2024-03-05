#[doc = "Register `KEY7` reader"]
pub type R = crate::R<KEY7_SPEC>;
#[doc = "Register `KEY7` writer"]
pub type W = crate::W<KEY7_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<KEY7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
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
#[doc = "AES Key 7.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY7_SPEC;
impl crate::RegisterSpec for KEY7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key7::R`](R) reader structure"]
impl crate::Readable for KEY7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key7::W`](W) writer structure"]
impl crate::Writable for KEY7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY7 to value 0"]
impl crate::Resettable for KEY7_SPEC {
    const RESET_VALUE: u32 = 0;
}
