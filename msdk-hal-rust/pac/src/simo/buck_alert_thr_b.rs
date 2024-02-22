#[doc = "Register `BUCK_ALERT_THR_B` reader"]
pub type R = crate::R<BUCK_ALERT_THR_B_SPEC>;
#[doc = "Register `BUCK_ALERT_THR_B` writer"]
pub type W = crate::W<BUCK_ALERT_THR_B_SPEC>;
#[doc = "Field `BUCKTHRB` reader - Threshold for ILOADB to generate the BUCK_ALERT"]
pub type BUCKTHRB_R = crate::FieldReader;
#[doc = "Field `BUCKTHRB` writer - Threshold for ILOADB to generate the BUCK_ALERT"]
pub type BUCKTHRB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Threshold for ILOADB to generate the BUCK_ALERT"]
    #[inline(always)]
    pub fn buckthrb(&self) -> BUCKTHRB_R {
        BUCKTHRB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Threshold for ILOADB to generate the BUCK_ALERT"]
    #[inline(always)]
    #[must_use]
    pub fn buckthrb(&mut self) -> BUCKTHRB_W<BUCK_ALERT_THR_B_SPEC> {
        BUCKTHRB_W::new(self, 0)
    }
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
#[doc = "Buck Cycle Count Alert VERGO_B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buck_alert_thr_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buck_alert_thr_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUCK_ALERT_THR_B_SPEC;
impl crate::RegisterSpec for BUCK_ALERT_THR_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buck_alert_thr_b::R`](R) reader structure"]
impl crate::Readable for BUCK_ALERT_THR_B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buck_alert_thr_b::W`](W) writer structure"]
impl crate::Writable for BUCK_ALERT_THR_B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUCK_ALERT_THR_B to value 0"]
impl crate::Resettable for BUCK_ALERT_THR_B_SPEC {
    const RESET_VALUE: u32 = 0;
}
