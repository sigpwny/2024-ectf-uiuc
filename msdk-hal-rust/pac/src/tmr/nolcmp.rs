#[doc = "Register `NOLCMP` reader"]
pub type R = crate::R<NOLCMP_SPEC>;
#[doc = "Register `NOLCMP` writer"]
pub type W = crate::W<NOLCMP_SPEC>;
#[doc = "Field `LO_A` reader - Non-Overlapping Low Compare value for Timer A controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
pub type LO_A_R = crate::FieldReader;
#[doc = "Field `LO_A` writer - Non-Overlapping Low Compare value for Timer A controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
pub type LO_A_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HI_A` reader - Non-Overlapping High Compare value for Timer A controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
pub type HI_A_R = crate::FieldReader;
#[doc = "Field `HI_A` writer - Non-Overlapping High Compare value for Timer A controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
pub type HI_A_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LO_B` reader - Non-Overlapping Low Compare value for Timer B controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
pub type LO_B_R = crate::FieldReader;
#[doc = "Field `LO_B` writer - Non-Overlapping Low Compare value for Timer B controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
pub type LO_B_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HI_B` reader - Non-Overlapping High Compare value for Timer B controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
pub type HI_B_R = crate::FieldReader;
#[doc = "Field `HI_B` writer - Non-Overlapping High Compare value for Timer B controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
pub type HI_B_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Non-Overlapping Low Compare value for Timer A controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
    #[inline(always)]
    pub fn lo_a(&self) -> LO_A_R {
        LO_A_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Non-Overlapping High Compare value for Timer A controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
    #[inline(always)]
    pub fn hi_a(&self) -> HI_A_R {
        HI_A_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Non-Overlapping Low Compare value for Timer B controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
    #[inline(always)]
    pub fn lo_b(&self) -> LO_B_R {
        LO_B_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Non-Overlapping High Compare value for Timer B controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
    #[inline(always)]
    pub fn hi_b(&self) -> HI_B_R {
        HI_B_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Non-Overlapping Low Compare value for Timer A controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
    #[inline(always)]
    #[must_use]
    pub fn lo_a(&mut self) -> LO_A_W<NOLCMP_SPEC> {
        LO_A_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Non-Overlapping High Compare value for Timer A controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
    #[inline(always)]
    #[must_use]
    pub fn hi_a(&mut self) -> HI_A_W<NOLCMP_SPEC> {
        HI_A_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Non-Overlapping Low Compare value for Timer B controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
    #[inline(always)]
    #[must_use]
    pub fn lo_b(&mut self) -> LO_B_W<NOLCMP_SPEC> {
        LO_B_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Non-Overlapping High Compare value for Timer B controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
    #[inline(always)]
    #[must_use]
    pub fn hi_b(&mut self) -> HI_B_W<NOLCMP_SPEC> {
        HI_B_W::new(self, 24)
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
