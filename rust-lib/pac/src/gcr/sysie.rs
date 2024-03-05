#[doc = "Register `SYSIE` reader"]
pub type R = crate::R<SYSIE_SPEC>;
#[doc = "Register `SYSIE` writer"]
pub type W = crate::W<SYSIE_SPEC>;
#[doc = "Field `ICEUNLOCK` reader - ARM ICE Unlock Interrupt Enable."]
pub type ICEUNLOCK_R = crate::BitReader<ICEUNLOCK_A>;
#[doc = "ARM ICE Unlock Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICEUNLOCK_A {
    #[doc = "0: disabled."]
    DIS = 0,
    #[doc = "1: enabled."]
    EN = 1,
}
impl From<ICEUNLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: ICEUNLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl ICEUNLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICEUNLOCK_A {
        match self.bits {
            false => ICEUNLOCK_A::DIS,
            true => ICEUNLOCK_A::EN,
        }
    }
    #[doc = "disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ICEUNLOCK_A::DIS
    }
    #[doc = "enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ICEUNLOCK_A::EN
    }
}
#[doc = "Field `ICEUNLOCK` writer - ARM ICE Unlock Interrupt Enable."]
pub type ICEUNLOCK_W<'a, REG> = crate::BitWriter<'a, REG, ICEUNLOCK_A>;
impl<'a, REG> ICEUNLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(ICEUNLOCK_A::DIS)
    }
    #[doc = "enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(ICEUNLOCK_A::EN)
    }
}
impl R {
    #[doc = "Bit 0 - ARM ICE Unlock Interrupt Enable."]
    #[inline(always)]
    pub fn iceunlock(&self) -> ICEUNLOCK_R {
        ICEUNLOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ARM ICE Unlock Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn iceunlock(&mut self) -> ICEUNLOCK_W<SYSIE_SPEC> {
        ICEUNLOCK_W::new(self, 0)
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
#[doc = "System Status Interrupt Enable Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSIE_SPEC;
impl crate::RegisterSpec for SYSIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysie::R`](R) reader structure"]
impl crate::Readable for SYSIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sysie::W`](W) writer structure"]
impl crate::Writable for SYSIE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSIE to value 0"]
impl crate::Resettable for SYSIE_SPEC {
    const RESET_VALUE: u32 = 0;
}
