#[doc = "Register `CNT` reader"]
pub type R = crate::R<CNT_SPEC>;
#[doc = "Field `COUNT` reader - Current Value of the Windowed Watchdog Timer Counter."]
pub type COUNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current Value of the Windowed Watchdog Timer Counter."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
#[doc = "Windowed Watchdog Timer Count Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNT_SPEC;
impl crate::RegisterSpec for CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CNT_SPEC {}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
