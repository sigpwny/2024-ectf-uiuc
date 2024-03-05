#[doc = "Register `LPCN` reader"]
pub type R = crate::R<LPCN_SPEC>;
#[doc = "Register `LPCN` writer"]
pub type W = crate::W<LPCN_SPEC>;
#[doc = "Field `RAMRET0` reader - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type RAMRET0_R = crate::BitReader<RAMRET0_A>;
#[doc = "System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMRET0_A {
    #[doc = "0: Disable Ram Retention."]
    DIS = 0,
    #[doc = "1: Enable System RAM 0 retention."]
    EN = 1,
}
impl From<RAMRET0_A> for bool {
    #[inline(always)]
    fn from(variant: RAMRET0_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMRET0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RAMRET0_A {
        match self.bits {
            false => RAMRET0_A::DIS,
            true => RAMRET0_A::EN,
        }
    }
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RAMRET0_A::DIS
    }
    #[doc = "Enable System RAM 0 retention."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RAMRET0_A::EN
    }
}
#[doc = "Field `RAMRET0` writer - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type RAMRET0_W<'a, REG> = crate::BitWriter<'a, REG, RAMRET0_A>;
impl<'a, REG> RAMRET0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RAMRET0_A::DIS)
    }
    #[doc = "Enable System RAM 0 retention."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RAMRET0_A::EN)
    }
}
#[doc = "Field `RAMRET1` reader - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type RAMRET1_R = crate::BitReader<RAMRET1_A>;
#[doc = "System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMRET1_A {
    #[doc = "0: Disable Ram Retention."]
    DIS = 0,
    #[doc = "1: Enable System RAM 1 retention."]
    EN = 1,
}
impl From<RAMRET1_A> for bool {
    #[inline(always)]
    fn from(variant: RAMRET1_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMRET1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RAMRET1_A {
        match self.bits {
            false => RAMRET1_A::DIS,
            true => RAMRET1_A::EN,
        }
    }
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RAMRET1_A::DIS
    }
    #[doc = "Enable System RAM 1 retention."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RAMRET1_A::EN
    }
}
#[doc = "Field `RAMRET1` writer - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type RAMRET1_W<'a, REG> = crate::BitWriter<'a, REG, RAMRET1_A>;
impl<'a, REG> RAMRET1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RAMRET1_A::DIS)
    }
    #[doc = "Enable System RAM 1 retention."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RAMRET1_A::EN)
    }
}
#[doc = "Field `RAMRET2` reader - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type RAMRET2_R = crate::BitReader<RAMRET2_A>;
#[doc = "System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMRET2_A {
    #[doc = "0: Disable Ram Retention."]
    DIS = 0,
    #[doc = "1: Enable System RAM 2 retention."]
    EN = 1,
}
impl From<RAMRET2_A> for bool {
    #[inline(always)]
    fn from(variant: RAMRET2_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMRET2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RAMRET2_A {
        match self.bits {
            false => RAMRET2_A::DIS,
            true => RAMRET2_A::EN,
        }
    }
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RAMRET2_A::DIS
    }
    #[doc = "Enable System RAM 2 retention."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RAMRET2_A::EN
    }
}
#[doc = "Field `RAMRET2` writer - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type RAMRET2_W<'a, REG> = crate::BitWriter<'a, REG, RAMRET2_A>;
impl<'a, REG> RAMRET2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RAMRET2_A::DIS)
    }
    #[doc = "Enable System RAM 2 retention."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RAMRET2_A::EN)
    }
}
#[doc = "Field `RAMRET3` reader - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type RAMRET3_R = crate::BitReader<RAMRET3_A>;
#[doc = "System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMRET3_A {
    #[doc = "0: Disable Ram Retention."]
    DIS = 0,
    #[doc = "1: Enable System RAM 3 retention."]
    EN = 1,
}
impl From<RAMRET3_A> for bool {
    #[inline(always)]
    fn from(variant: RAMRET3_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMRET3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RAMRET3_A {
        match self.bits {
            false => RAMRET3_A::DIS,
            true => RAMRET3_A::EN,
        }
    }
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RAMRET3_A::DIS
    }
    #[doc = "Enable System RAM 3 retention."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RAMRET3_A::EN
    }
}
#[doc = "Field `RAMRET3` writer - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type RAMRET3_W<'a, REG> = crate::BitWriter<'a, REG, RAMRET3_A>;
impl<'a, REG> RAMRET3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RAMRET3_A::DIS)
    }
    #[doc = "Enable System RAM 3 retention."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RAMRET3_A::EN)
    }
}
#[doc = "Field `LPMCLKSEL` reader - Low Power Mode APB Clock Select."]
pub type LPMCLKSEL_R = crate::BitReader;
#[doc = "Field `LPMCLKSEL` writer - Low Power Mode APB Clock Select."]
pub type LPMCLKSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMFAST` reader - Low Power Mode Clock Select."]
pub type LPMFAST_R = crate::BitReader;
#[doc = "Field `LPMFAST` writer - Low Power Mode Clock Select."]
pub type LPMFAST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BG_DIS` reader - Bandgap OFF. This controls the System Bandgap in DeepSleep mode."]
pub type BG_DIS_R = crate::BitReader<BG_DIS_A>;
#[doc = "Bandgap OFF. This controls the System Bandgap in DeepSleep mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BG_DIS_A {
    #[doc = "0: Bandgap is always ON."]
    ON = 0,
    #[doc = "1: Bandgap is OFF in DeepSleep mode (default)."]
    OFF = 1,
}
impl From<BG_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: BG_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl BG_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BG_DIS_A {
        match self.bits {
            false => BG_DIS_A::ON,
            true => BG_DIS_A::OFF,
        }
    }
    #[doc = "Bandgap is always ON."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == BG_DIS_A::ON
    }
    #[doc = "Bandgap is OFF in DeepSleep mode (default)."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == BG_DIS_A::OFF
    }
}
#[doc = "Field `BG_DIS` writer - Bandgap OFF. This controls the System Bandgap in DeepSleep mode."]
pub type BG_DIS_W<'a, REG> = crate::BitWriter<'a, REG, BG_DIS_A>;
impl<'a, REG> BG_DIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bandgap is always ON."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(BG_DIS_A::ON)
    }
    #[doc = "Bandgap is OFF in DeepSleep mode (default)."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(BG_DIS_A::OFF)
    }
}
#[doc = "Field `LPWKST_CLR` reader - Low Power Wakeup Status Register Clear"]
pub type LPWKST_CLR_R = crate::BitReader;
#[doc = "Field `LPWKST_CLR` writer - Low Power Wakeup Status Register Clear"]
pub type LPWKST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret0(&self) -> RAMRET0_R {
        RAMRET0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret1(&self) -> RAMRET1_R {
        RAMRET1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret2(&self) -> RAMRET2_R {
        RAMRET2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret3(&self) -> RAMRET3_R {
        RAMRET3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Low Power Mode APB Clock Select."]
    #[inline(always)]
    pub fn lpmclksel(&self) -> LPMCLKSEL_R {
        LPMCLKSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low Power Mode Clock Select."]
    #[inline(always)]
    pub fn lpmfast(&self) -> LPMFAST_R {
        LPMFAST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Bandgap OFF. This controls the System Bandgap in DeepSleep mode."]
    #[inline(always)]
    pub fn bg_dis(&self) -> BG_DIS_R {
        BG_DIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 31 - Low Power Wakeup Status Register Clear"]
    #[inline(always)]
    pub fn lpwkst_clr(&self) -> LPWKST_CLR_R {
        LPWKST_CLR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    #[must_use]
    pub fn ramret0(&mut self) -> RAMRET0_W<LPCN_SPEC> {
        RAMRET0_W::new(self, 0)
    }
    #[doc = "Bit 1 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    #[must_use]
    pub fn ramret1(&mut self) -> RAMRET1_W<LPCN_SPEC> {
        RAMRET1_W::new(self, 1)
    }
    #[doc = "Bit 2 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    #[must_use]
    pub fn ramret2(&mut self) -> RAMRET2_W<LPCN_SPEC> {
        RAMRET2_W::new(self, 2)
    }
    #[doc = "Bit 3 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    #[must_use]
    pub fn ramret3(&mut self) -> RAMRET3_W<LPCN_SPEC> {
        RAMRET3_W::new(self, 3)
    }
    #[doc = "Bit 8 - Low Power Mode APB Clock Select."]
    #[inline(always)]
    #[must_use]
    pub fn lpmclksel(&mut self) -> LPMCLKSEL_W<LPCN_SPEC> {
        LPMCLKSEL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Low Power Mode Clock Select."]
    #[inline(always)]
    #[must_use]
    pub fn lpmfast(&mut self) -> LPMFAST_W<LPCN_SPEC> {
        LPMFAST_W::new(self, 9)
    }
    #[doc = "Bit 11 - Bandgap OFF. This controls the System Bandgap in DeepSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn bg_dis(&mut self) -> BG_DIS_W<LPCN_SPEC> {
        BG_DIS_W::new(self, 11)
    }
    #[doc = "Bit 31 - Low Power Wakeup Status Register Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lpwkst_clr(&mut self) -> LPWKST_CLR_W<LPCN_SPEC> {
        LPWKST_CLR_W::new(self, 31)
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
#[doc = "Low Power Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpcn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpcn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPCN_SPEC;
impl crate::RegisterSpec for LPCN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpcn::R`](R) reader structure"]
impl crate::Readable for LPCN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpcn::W`](W) writer structure"]
impl crate::Writable for LPCN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPCN to value 0"]
impl crate::Resettable for LPCN_SPEC {
    const RESET_VALUE: u32 = 0;
}
