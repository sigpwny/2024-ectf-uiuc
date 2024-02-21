#[doc = "Register `ZERO_CROSS_CAL_A` reader"]
pub type R = crate::R<ZERO_CROSS_CAL_A_SPEC>;
#[doc = "Field `ZXCALA` reader - Zero Cross Calibrartion Value VREGO_A"]
pub type ZXCALA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Zero Cross Calibrartion Value VREGO_A"]
    #[inline(always)]
    pub fn zxcala(&self) -> ZXCALA_R {
        ZXCALA_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Zero Cross Calibration VERGO_A Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`zero_cross_cal_a::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ZERO_CROSS_CAL_A_SPEC;
impl crate::RegisterSpec for ZERO_CROSS_CAL_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`zero_cross_cal_a::R`](R) reader structure"]
impl crate::Readable for ZERO_CROSS_CAL_A_SPEC {}
#[doc = "`reset()` method sets ZERO_CROSS_CAL_A to value 0"]
impl crate::Resettable for ZERO_CROSS_CAL_A_SPEC {
    const RESET_VALUE: u32 = 0;
}
