#[doc = "Register `TODA` reader"]
pub type R = crate::R<TODA_SPEC>;
#[doc = "Register `TODA` writer"]
pub type W = crate::W<TODA_SPEC>;
#[doc = "Field `TOD_ALARM` reader - Time-of-day Alarm."]
pub type TOD_ALARM_R = crate::FieldReader<u32>;
#[doc = "Field `TOD_ALARM` writer - Time-of-day Alarm."]
pub type TOD_ALARM_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Time-of-day Alarm."]
    #[inline(always)]
    pub fn tod_alarm(&self) -> TOD_ALARM_R {
        TOD_ALARM_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Time-of-day Alarm."]
    #[inline(always)]
    #[must_use]
    pub fn tod_alarm(&mut self) -> TOD_ALARM_W<TODA_SPEC> {
        TOD_ALARM_W::new(self, 0)
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
#[doc = "Time-of-day Alarm.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`toda::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`toda::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TODA_SPEC;
impl crate::RegisterSpec for TODA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`toda::R`](R) reader structure"]
impl crate::Readable for TODA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`toda::W`](W) writer structure"]
impl crate::Writable for TODA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TODA to value 0"]
impl crate::Resettable for TODA_SPEC {
    const RESET_VALUE: u32 = 0;
}
