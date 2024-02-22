#[doc = "Register `TRIM` reader"]
pub type R = crate::R<TRIM_SPEC>;
#[doc = "Register `TRIM` writer"]
pub type W = crate::W<TRIM_SPEC>;
#[doc = "Field `TRIM` reader - RTC Trim. This register contains the 2's complement value that specifies the trim resolution. Each increment or decrement of the bit adds or subtracts 1ppm at each 4KHz clock value, with a maximum correction of +/- 127ppm."]
pub type TRIM_R = crate::FieldReader;
#[doc = "Field `TRIM` writer - RTC Trim. This register contains the 2's complement value that specifies the trim resolution. Each increment or decrement of the bit adds or subtracts 1ppm at each 4KHz clock value, with a maximum correction of +/- 127ppm."]
pub type TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VRTC_TMR` reader - VBAT Timer Value. When RTC is running off of VBAT, this field is incremented every 32 seconds."]
pub type VRTC_TMR_R = crate::FieldReader<u32>;
#[doc = "Field `VRTC_TMR` writer - VBAT Timer Value. When RTC is running off of VBAT, this field is incremented every 32 seconds."]
pub type VRTC_TMR_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - RTC Trim. This register contains the 2's complement value that specifies the trim resolution. Each increment or decrement of the bit adds or subtracts 1ppm at each 4KHz clock value, with a maximum correction of +/- 127ppm."]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - VBAT Timer Value. When RTC is running off of VBAT, this field is incremented every 32 seconds."]
    #[inline(always)]
    pub fn vrtc_tmr(&self) -> VRTC_TMR_R {
        VRTC_TMR_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - RTC Trim. This register contains the 2's complement value that specifies the trim resolution. Each increment or decrement of the bit adds or subtracts 1ppm at each 4KHz clock value, with a maximum correction of +/- 127ppm."]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<TRIM_SPEC> {
        TRIM_W::new(self, 0)
    }
    #[doc = "Bits 8:31 - VBAT Timer Value. When RTC is running off of VBAT, this field is incremented every 32 seconds."]
    #[inline(always)]
    #[must_use]
    pub fn vrtc_tmr(&mut self) -> VRTC_TMR_W<TRIM_SPEC> {
        VRTC_TMR_W::new(self, 8)
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
#[doc = "RTC Trim Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRIM_SPEC;
impl crate::RegisterSpec for TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trim::R`](R) reader structure"]
impl crate::Readable for TRIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trim::W`](W) writer structure"]
impl crate::Writable for TRIM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRIM to value 0"]
impl crate::Resettable for TRIM_SPEC {
    const RESET_VALUE: u32 = 0;
}
