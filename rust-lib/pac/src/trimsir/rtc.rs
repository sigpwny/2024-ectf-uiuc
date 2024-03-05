#[doc = "Register `RTC` reader"]
pub type R = crate::R<RTC_SPEC>;
#[doc = "Register `RTC` writer"]
pub type W = crate::W<RTC_SPEC>;
#[doc = "Field `X1TRIM` reader - RTC X1 Trim."]
pub type X1TRIM_R = crate::FieldReader;
#[doc = "Field `X1TRIM` writer - RTC X1 Trim."]
pub type X1TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `X2TRIM` reader - RTC X2 Trim."]
pub type X2TRIM_R = crate::FieldReader;
#[doc = "Field `X2TRIM` writer - RTC X2 Trim."]
pub type X2TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LOCK` reader - Lock."]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - Lock."]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:20 - RTC X1 Trim."]
    #[inline(always)]
    pub fn x1trim(&self) -> X1TRIM_R {
        X1TRIM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - RTC X2 Trim."]
    #[inline(always)]
    pub fn x2trim(&self) -> X2TRIM_R {
        X2TRIM_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Lock."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:20 - RTC X1 Trim."]
    #[inline(always)]
    #[must_use]
    pub fn x1trim(&mut self) -> X1TRIM_W<RTC_SPEC> {
        X1TRIM_W::new(self, 16)
    }
    #[doc = "Bits 21:25 - RTC X2 Trim."]
    #[inline(always)]
    #[must_use]
    pub fn x2trim(&mut self) -> X2TRIM_W<RTC_SPEC> {
        X2TRIM_W::new(self, 21)
    }
    #[doc = "Bit 31 - Lock."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<RTC_SPEC> {
        LOCK_W::new(self, 31)
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
#[doc = "RTC Trim System Initialization Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_SPEC;
impl crate::RegisterSpec for RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc::R`](R) reader structure"]
impl crate::Readable for RTC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc::W`](W) writer structure"]
impl crate::Writable for RTC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC to value 0"]
impl crate::Resettable for RTC_SPEC {
    const RESET_VALUE: u32 = 0;
}
