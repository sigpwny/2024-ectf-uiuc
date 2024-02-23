#[doc = "Register `INTR` reader"]
pub type R = crate::R<INTR_SPEC>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<INTR_SPEC>;
#[doc = "Field `DONE` reader - Flash Done Interrupt. This bit is set to 1 upon Flash write or erase completion."]
pub type DONE_R = crate::BitReader<DONE_A>;
#[doc = "Flash Done Interrupt. This bit is set to 1 upon Flash write or erase completion.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DONE_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<DONE_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DONE_A {
        match self.bits {
            false => DONE_A::INACTIVE,
            true => DONE_A::PENDING,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DONE_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DONE_A::PENDING
    }
}
#[doc = "Field `DONE` writer - Flash Done Interrupt. This bit is set to 1 upon Flash write or erase completion."]
pub type DONE_W<'a, REG> = crate::BitWriter<'a, REG, DONE_A>;
impl<'a, REG> DONE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(DONE_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(DONE_A::PENDING)
    }
}
#[doc = "Field `AF` reader - Flash Access Fail. This bit is set when an attempt is made to write the flash while the flash is busy or the flash is locked. This bit can only be set to 1 by hardware."]
pub type AF_R = crate::BitReader<AF_A>;
#[doc = "Flash Access Fail. This bit is set when an attempt is made to write the flash while the flash is busy or the flash is locked. This bit can only be set to 1 by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AF_A {
    #[doc = "0: No Failure."]
    NO_ERROR = 0,
    #[doc = "1: Failure occurs."]
    ERROR = 1,
}
impl From<AF_A> for bool {
    #[inline(always)]
    fn from(variant: AF_A) -> Self {
        variant as u8 != 0
    }
}
impl AF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AF_A {
        match self.bits {
            false => AF_A::NO_ERROR,
            true => AF_A::ERROR,
        }
    }
    #[doc = "No Failure."]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == AF_A::NO_ERROR
    }
    #[doc = "Failure occurs."]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == AF_A::ERROR
    }
}
#[doc = "Field `AF` writer - Flash Access Fail. This bit is set when an attempt is made to write the flash while the flash is busy or the flash is locked. This bit can only be set to 1 by hardware."]
pub type AF_W<'a, REG> = crate::BitWriter<'a, REG, AF_A>;
impl<'a, REG> AF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Failure."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(AF_A::NO_ERROR)
    }
    #[doc = "Failure occurs."]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(AF_A::ERROR)
    }
}
#[doc = "Field `DONEIE` reader - Flash Done Interrupt Enable."]
pub type DONEIE_R = crate::BitReader<DONEIE_A>;
#[doc = "Flash Done Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DONEIE_A {
    #[doc = "0: Disable."]
    DISABLE = 0,
    #[doc = "1: Enable."]
    ENABLE = 1,
}
impl From<DONEIE_A> for bool {
    #[inline(always)]
    fn from(variant: DONEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DONEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DONEIE_A {
        match self.bits {
            false => DONEIE_A::DISABLE,
            true => DONEIE_A::ENABLE,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DONEIE_A::DISABLE
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DONEIE_A::ENABLE
    }
}
#[doc = "Field `DONEIE` writer - Flash Done Interrupt Enable."]
pub type DONEIE_W<'a, REG> = crate::BitWriter<'a, REG, DONEIE_A>;
impl<'a, REG> DONEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DONEIE_A::DISABLE)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DONEIE_A::ENABLE)
    }
}
#[doc = "Field `AFIE` reader - Flash Done Interrupt Enable."]
pub use DONEIE_R as AFIE_R;
#[doc = "Field `AFIE` writer - Flash Done Interrupt Enable."]
pub use DONEIE_W as AFIE_W;
impl R {
    #[doc = "Bit 0 - Flash Done Interrupt. This bit is set to 1 upon Flash write or erase completion."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Access Fail. This bit is set when an attempt is made to write the flash while the flash is busy or the flash is locked. This bit can only be set to 1 by hardware."]
    #[inline(always)]
    pub fn af(&self) -> AF_R {
        AF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash Done Interrupt Enable."]
    #[inline(always)]
    pub fn doneie(&self) -> DONEIE_R {
        DONEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Flash Done Interrupt Enable."]
    #[inline(always)]
    pub fn afie(&self) -> AFIE_R {
        AFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Done Interrupt. This bit is set to 1 upon Flash write or erase completion."]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<INTR_SPEC> {
        DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Flash Access Fail. This bit is set when an attempt is made to write the flash while the flash is busy or the flash is locked. This bit can only be set to 1 by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn af(&mut self) -> AF_W<INTR_SPEC> {
        AF_W::new(self, 1)
    }
    #[doc = "Bit 8 - Flash Done Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn doneie(&mut self) -> DONEIE_W<INTR_SPEC> {
        DONEIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Flash Done Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn afie(&mut self) -> AFIE_W<INTR_SPEC> {
        AFIE_W::new(self, 9)
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
#[doc = "Flash Interrupt Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for INTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
