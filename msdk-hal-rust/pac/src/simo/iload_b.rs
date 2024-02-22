#[doc = "Register `ILOAD_B` reader"]
pub type R = crate::R<ILOAD_B_SPEC>;
#[doc = "Field `ILOADB` reader - Number of buck cycles that occur within the cycle clock"]
pub type ILOADB_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of buck cycles that occur within the cycle clock"]
    #[inline(always)]
    pub fn iloadb(&self) -> ILOADB_R {
        ILOADB_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Buck Cycle Count VREGO_B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iload_b::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ILOAD_B_SPEC;
impl crate::RegisterSpec for ILOAD_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iload_b::R`](R) reader structure"]
impl crate::Readable for ILOAD_B_SPEC {}
#[doc = "`reset()` method sets ILOAD_B to value 0"]
impl crate::Resettable for ILOAD_B_SPEC {
    const RESET_VALUE: u32 = 0;
}
