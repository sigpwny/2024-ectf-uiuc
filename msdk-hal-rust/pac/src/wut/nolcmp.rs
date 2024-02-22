#[doc = "Register `NOLCMP` reader"]
pub type R = crate::R<NOLCMP_SPEC>;
#[doc = "Register `NOLCMP` writer"]
pub type W = crate::W<NOLCMP_SPEC>;
#[doc = "Field `NOLLCMP` reader - Non-overlapping Low Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A and next rising edge of PWM output 0A'."]
pub type NOLLCMP_R = crate::FieldReader;
#[doc = "Field `NOLLCMP` writer - Non-overlapping Low Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A and next rising edge of PWM output 0A'."]
pub type NOLLCMP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NOLHCMP` reader - Non-overlapping High Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A' and next rising edge of PWM output 0A."]
pub type NOLHCMP_R = crate::FieldReader;
#[doc = "Field `NOLHCMP` writer - Non-overlapping High Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A' and next rising edge of PWM output 0A."]
pub type NOLHCMP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Non-overlapping Low Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A and next rising edge of PWM output 0A'."]
    #[inline(always)]
    pub fn nollcmp(&self) -> NOLLCMP_R {
        NOLLCMP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Non-overlapping High Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A' and next rising edge of PWM output 0A."]
    #[inline(always)]
    pub fn nolhcmp(&self) -> NOLHCMP_R {
        NOLHCMP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Non-overlapping Low Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A and next rising edge of PWM output 0A'."]
    #[inline(always)]
    #[must_use]
    pub fn nollcmp(&mut self) -> NOLLCMP_W<NOLCMP_SPEC> {
        NOLLCMP_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Non-overlapping High Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A' and next rising edge of PWM output 0A."]
    #[inline(always)]
    #[must_use]
    pub fn nolhcmp(&mut self) -> NOLHCMP_W<NOLCMP_SPEC> {
        NOLHCMP_W::new(self, 8)
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
#[doc = "Timer Non-Overlapping Compare Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nolcmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nolcmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NOLCMP_SPEC;
impl crate::RegisterSpec for NOLCMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nolcmp::R`](R) reader structure"]
impl crate::Readable for NOLCMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nolcmp::W`](W) writer structure"]
impl crate::Writable for NOLCMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NOLCMP to value 0"]
impl crate::Resettable for NOLCMP_SPEC {
    const RESET_VALUE: u32 = 0;
}
