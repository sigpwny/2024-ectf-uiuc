#[doc = "Register `ILOAD_C` reader"]
pub type R = crate::R<ILOAD_C_SPEC>;
#[doc = "Field `ILOADC` reader - Number of buck cycles that occur within the cycle clock"]
pub type ILOADC_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of buck cycles that occur within the cycle clock"]
    #[inline(always)]
    pub fn iloadc(&self) -> ILOADC_R {
        ILOADC_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Buck Cycle Count VREGO_C Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iload_c::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ILOAD_C_SPEC;
impl crate::RegisterSpec for ILOAD_C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iload_c::R`](R) reader structure"]
impl crate::Readable for ILOAD_C_SPEC {}
#[doc = "`reset()` method sets ILOAD_C to value 0"]
impl crate::Resettable for ILOAD_C_SPEC {
    const RESET_VALUE: u32 = 0;
}
