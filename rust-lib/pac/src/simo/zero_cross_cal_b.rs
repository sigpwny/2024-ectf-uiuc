#[doc = "Register `ZERO_CROSS_CAL_B` reader"]
pub type R = crate::R<ZERO_CROSS_CAL_B_SPEC>;
#[doc = "Field `ZXCALB` reader - Zero Cross Calibrartion Value VREGO_B"]
pub type ZXCALB_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Zero Cross Calibrartion Value VREGO_B"]
    #[inline(always)]
    pub fn zxcalb(&self) -> ZXCALB_R {
        ZXCALB_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Zero Cross Calibration VERGO_B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`zero_cross_cal_b::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ZERO_CROSS_CAL_B_SPEC;
impl crate::RegisterSpec for ZERO_CROSS_CAL_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`zero_cross_cal_b::R`](R) reader structure"]
impl crate::Readable for ZERO_CROSS_CAL_B_SPEC {}
#[doc = "`reset()` method sets ZERO_CROSS_CAL_B to value 0"]
impl crate::Resettable for ZERO_CROSS_CAL_B_SPEC {
    const RESET_VALUE: u32 = 0;
}
