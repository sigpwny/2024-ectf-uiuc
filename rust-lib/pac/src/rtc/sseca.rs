#[doc = "Register `SSECA` reader"]
pub type R = crate::R<SSECA_SPEC>;
#[doc = "Register `SSECA` writer"]
pub type W = crate::W<SSECA_SPEC>;
#[doc = "Field `SSEC_ALARM` reader - This register contains the reload value for the sub-second alarm."]
pub type SSEC_ALARM_R = crate::FieldReader<u32>;
#[doc = "Field `SSEC_ALARM` writer - This register contains the reload value for the sub-second alarm."]
pub type SSEC_ALARM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register contains the reload value for the sub-second alarm."]
    #[inline(always)]
    pub fn ssec_alarm(&self) -> SSEC_ALARM_R {
        SSEC_ALARM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register contains the reload value for the sub-second alarm."]
    #[inline(always)]
    #[must_use]
    pub fn ssec_alarm(&mut self) -> SSEC_ALARM_W<SSECA_SPEC> {
        SSEC_ALARM_W::new(self, 0)
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
#[doc = "RTC sub-second alarm. This register contains the reload value for the sub-second alarm.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sseca::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sseca::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSECA_SPEC;
impl crate::RegisterSpec for SSECA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sseca::R`](R) reader structure"]
impl crate::Readable for SSECA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sseca::W`](W) writer structure"]
impl crate::Writable for SSECA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSECA to value 0"]
impl crate::Resettable for SSECA_SPEC {
    const RESET_VALUE: u32 = 0;
}
