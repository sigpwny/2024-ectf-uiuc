#[doc = "Register `IPOLO` reader"]
pub type R = crate::R<IPOLO_SPEC>;
#[doc = "Field `IPO_LIMITLO` reader - IPO Low Limit Trim."]
pub type IPO_LIMITLO_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - IPO Low Limit Trim."]
    #[inline(always)]
    pub fn ipo_limitlo(&self) -> IPO_LIMITLO_R {
        IPO_LIMITLO_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "IPO Low Trim System Initialization Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipolo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPOLO_SPEC;
impl crate::RegisterSpec for IPOLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipolo::R`](R) reader structure"]
impl crate::Readable for IPOLO_SPEC {}
#[doc = "`reset()` method sets IPOLO to value 0"]
impl crate::Resettable for IPOLO_SPEC {
    const RESET_VALUE: u32 = 0;
}
