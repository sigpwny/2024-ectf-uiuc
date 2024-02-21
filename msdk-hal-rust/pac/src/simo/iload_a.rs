#[doc = "Register `ILOAD_A` reader"]
pub type R = crate::R<ILOAD_A_SPEC>;
#[doc = "Field `ILOADA` reader - Number of buck cycles that occur within the cycle clock"]
pub type ILOADA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of buck cycles that occur within the cycle clock"]
    #[inline(always)]
    pub fn iloada(&self) -> ILOADA_R {
        ILOADA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Buck Cycle Count VREGO_A Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iload_a::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ILOAD_A_SPEC;
impl crate::RegisterSpec for ILOAD_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iload_a::R`](R) reader structure"]
impl crate::Readable for ILOAD_A_SPEC {}
#[doc = "`reset()` method sets ILOAD_A to value 0"]
impl crate::Resettable for ILOAD_A_SPEC {
    const RESET_VALUE: u32 = 0;
}
