#[doc = "Register `REVISION` reader"]
pub type R = crate::R<REVISION_SPEC>;
#[doc = "Field `REVISION` reader - Manufacturer Chip Revision."]
pub type REVISION_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Manufacturer Chip Revision."]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Revision Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revision::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REVISION_SPEC;
impl crate::RegisterSpec for REVISION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`revision::R`](R) reader structure"]
impl crate::Readable for REVISION_SPEC {}
#[doc = "`reset()` method sets REVISION to value 0"]
impl crate::Resettable for REVISION_SPEC {
    const RESET_VALUE: u32 = 0;
}
