#[doc = "Register `PWM` reader"]
pub type R = crate::R<PWM_SPEC>;
#[doc = "Register `PWM` writer"]
pub type W = crate::W<PWM_SPEC>;
#[doc = "Field `PWM` reader - Timer PWM Match: In PWM Mode, this field sets the count value for the first transition period of the PWM cycle. At the end of the cycle where CNT equals PWM, the PWM output transitions to the second period of the PWM cycle. The second PWM period count is stored in the CMP register. The value set for PWM must me less than the value set in CMP for PWM mode operation. Timer Capture Value: In Capture, Compare, and Capture/Compare modes, this field is used to store the CNT value when a Capture, Compare, or Capture/Compare event occurs."]
pub type PWM_R = crate::FieldReader<u32>;
#[doc = "Field `PWM` writer - Timer PWM Match: In PWM Mode, this field sets the count value for the first transition period of the PWM cycle. At the end of the cycle where CNT equals PWM, the PWM output transitions to the second period of the PWM cycle. The second PWM period count is stored in the CMP register. The value set for PWM must me less than the value set in CMP for PWM mode operation. Timer Capture Value: In Capture, Compare, and Capture/Compare modes, this field is used to store the CNT value when a Capture, Compare, or Capture/Compare event occurs."]
pub type PWM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer PWM Match: In PWM Mode, this field sets the count value for the first transition period of the PWM cycle. At the end of the cycle where CNT equals PWM, the PWM output transitions to the second period of the PWM cycle. The second PWM period count is stored in the CMP register. The value set for PWM must me less than the value set in CMP for PWM mode operation. Timer Capture Value: In Capture, Compare, and Capture/Compare modes, this field is used to store the CNT value when a Capture, Compare, or Capture/Compare event occurs."]
    #[inline(always)]
    pub fn pwm(&self) -> PWM_R {
        PWM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer PWM Match: In PWM Mode, this field sets the count value for the first transition period of the PWM cycle. At the end of the cycle where CNT equals PWM, the PWM output transitions to the second period of the PWM cycle. The second PWM period count is stored in the CMP register. The value set for PWM must me less than the value set in CMP for PWM mode operation. Timer Capture Value: In Capture, Compare, and Capture/Compare modes, this field is used to store the CNT value when a Capture, Compare, or Capture/Compare event occurs."]
    #[inline(always)]
    #[must_use]
    pub fn pwm(&mut self) -> PWM_W<PWM_SPEC> {
        PWM_W::new(self, 0)
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
#[doc = "Timer PWM Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWM_SPEC;
impl crate::RegisterSpec for PWM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm::R`](R) reader structure"]
impl crate::Readable for PWM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwm::W`](W) writer structure"]
impl crate::Writable for PWM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWM to value 0"]
impl crate::Resettable for PWM_SPEC {
    const RESET_VALUE: u32 = 0;
}
