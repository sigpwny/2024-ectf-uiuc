#[doc = "Register `SYSST` reader"]
pub type R = crate::R<SYSST_SPEC>;
#[doc = "Register `SYSST` writer"]
pub type W = crate::W<SYSST_SPEC>;
#[doc = "Field `ICELOCK` reader - ARM ICE Lock Status."]
pub type ICELOCK_R = crate::BitReader<ICELOCK_A>;
#[doc = "ARM ICE Lock Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICELOCK_A {
    #[doc = "0: ICE is unlocked."]
    UNLOCKED = 0,
    #[doc = "1: ICE is locked."]
    LOCKED = 1,
}
impl From<ICELOCK_A> for bool {
    #[inline(always)]
    fn from(variant: ICELOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl ICELOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICELOCK_A {
        match self.bits {
            false => ICELOCK_A::UNLOCKED,
            true => ICELOCK_A::LOCKED,
        }
    }
    #[doc = "ICE is unlocked."]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == ICELOCK_A::UNLOCKED
    }
    #[doc = "ICE is locked."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == ICELOCK_A::LOCKED
    }
}
#[doc = "Field `ICELOCK` writer - ARM ICE Lock Status."]
pub type ICELOCK_W<'a, REG> = crate::BitWriter<'a, REG, ICELOCK_A>;
impl<'a, REG> ICELOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ICE is unlocked."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(ICELOCK_A::UNLOCKED)
    }
    #[doc = "ICE is locked."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(ICELOCK_A::LOCKED)
    }
}
impl R {
    #[doc = "Bit 0 - ARM ICE Lock Status."]
    #[inline(always)]
    pub fn icelock(&self) -> ICELOCK_R {
        ICELOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ARM ICE Lock Status."]
    #[inline(always)]
    #[must_use]
    pub fn icelock(&mut self) -> ICELOCK_W<SYSST_SPEC> {
        ICELOCK_W::new(self, 0)
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
#[doc = "System Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSST_SPEC;
impl crate::RegisterSpec for SYSST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysst::R`](R) reader structure"]
impl crate::Readable for SYSST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sysst::W`](W) writer structure"]
impl crate::Writable for SYSST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSST to value 0"]
impl crate::Resettable for SYSST_SPEC {
    const RESET_VALUE: u32 = 0;
}
