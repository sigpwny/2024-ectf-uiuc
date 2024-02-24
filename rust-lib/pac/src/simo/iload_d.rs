#[doc = "Register `ILOAD_D` reader"]
pub type R = crate::R<ILOAD_D_SPEC>;
#[doc = "Field `ILOADD` reader - Number of buck cycles that occur within the cycle clock"]
pub type ILOADD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of buck cycles that occur within the cycle clock"]
    #[inline(always)]
    pub fn iloadd(&self) -> ILOADD_R {
        ILOADD_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Buck Cycle Count VREGO_D Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iload_d::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ILOAD_D_SPEC;
impl crate::RegisterSpec for ILOAD_D_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iload_d::R`](R) reader structure"]
impl crate::Readable for ILOAD_D_SPEC {}
#[doc = "`reset()` method sets ILOAD_D to value 0"]
impl crate::Resettable for ILOAD_D_SPEC {
    const RESET_VALUE: u32 = 0;
}
