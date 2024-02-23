#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `BUSY` reader - Bus Status."]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "Bus Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: I2C Bus Idle."]
    IDLE = 0,
    #[doc = "1: I2C Bus Busy."]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::IDLE,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "I2C Bus Idle."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY_A::IDLE
    }
    #[doc = "I2C Bus Busy."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
}
#[doc = "Field `RX_EM` reader - RX empty."]
pub type RX_EM_R = crate::BitReader<RX_EM_A>;
#[doc = "RX empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_EM_A {
    #[doc = "0: Not Empty."]
    NOT_EMPTY = 0,
    #[doc = "1: Empty."]
    EMPTY = 1,
}
impl From<RX_EM_A> for bool {
    #[inline(always)]
    fn from(variant: RX_EM_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_EM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_EM_A {
        match self.bits {
            false => RX_EM_A::NOT_EMPTY,
            true => RX_EM_A::EMPTY,
        }
    }
    #[doc = "Not Empty."]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RX_EM_A::NOT_EMPTY
    }
    #[doc = "Empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RX_EM_A::EMPTY
    }
}
#[doc = "Field `RX_FULL` reader - RX Full."]
pub type RX_FULL_R = crate::BitReader<RX_FULL_A>;
#[doc = "RX Full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FULL_A {
    #[doc = "0: Not Full."]
    NOT_FULL = 0,
    #[doc = "1: Full."]
    FULL = 1,
}
impl From<RX_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FULL_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_FULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_FULL_A {
        match self.bits {
            false => RX_FULL_A::NOT_FULL,
            true => RX_FULL_A::FULL,
        }
    }
    #[doc = "Not Full."]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RX_FULL_A::NOT_FULL
    }
    #[doc = "Full."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RX_FULL_A::FULL
    }
}
#[doc = "Field `TX_EM` reader - TX Empty."]
pub type TX_EM_R = crate::BitReader<TX_EM_A>;
#[doc = "TX Empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_EM_A {
    #[doc = "0: Not Empty."]
    NOT_EMPTY = 0,
    #[doc = "1: Empty."]
    EMPTY = 1,
}
impl From<TX_EM_A> for bool {
    #[inline(always)]
    fn from(variant: TX_EM_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_EM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_EM_A {
        match self.bits {
            false => TX_EM_A::NOT_EMPTY,
            true => TX_EM_A::EMPTY,
        }
    }
    #[doc = "Not Empty."]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TX_EM_A::NOT_EMPTY
    }
    #[doc = "Empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TX_EM_A::EMPTY
    }
}
#[doc = "Field `TX_EM` writer - TX Empty."]
pub type TX_EM_W<'a, REG> = crate::BitWriter<'a, REG, TX_EM_A>;
impl<'a, REG> TX_EM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Empty."]
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut crate::W<REG> {
        self.variant(TX_EM_A::NOT_EMPTY)
    }
    #[doc = "Empty."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(TX_EM_A::EMPTY)
    }
}
#[doc = "Field `TX_FULL` reader - TX Full."]
pub type TX_FULL_R = crate::BitReader<TX_FULL_A>;
#[doc = "TX Full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_FULL_A {
    #[doc = "0: Not Empty."]
    NOT_EMPTY = 0,
    #[doc = "1: Empty."]
    EMPTY = 1,
}
impl From<TX_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: TX_FULL_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_FULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_FULL_A {
        match self.bits {
            false => TX_FULL_A::NOT_EMPTY,
            true => TX_FULL_A::EMPTY,
        }
    }
    #[doc = "Not Empty."]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TX_FULL_A::NOT_EMPTY
    }
    #[doc = "Empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TX_FULL_A::EMPTY
    }
}
#[doc = "Field `TX_FULL` writer - TX Full."]
pub type TX_FULL_W<'a, REG> = crate::BitWriter<'a, REG, TX_FULL_A>;
impl<'a, REG> TX_FULL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Empty."]
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FULL_A::NOT_EMPTY)
    }
    #[doc = "Empty."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FULL_A::EMPTY)
    }
}
#[doc = "Field `MST_BUSY` reader - Clock Mode."]
pub type MST_BUSY_R = crate::BitReader<MST_BUSY_A>;
#[doc = "Clock Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MST_BUSY_A {
    #[doc = "0: Device not actively driving SCL clock cycles."]
    NOT_ACTIVELY_DRIVING_SCL_CLOCK = 0,
    #[doc = "1: Device operating as master and actively driving SCL clock cycles."]
    ACTIVELY_DRIVING_SCL_CLOCK = 1,
}
impl From<MST_BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: MST_BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl MST_BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MST_BUSY_A {
        match self.bits {
            false => MST_BUSY_A::NOT_ACTIVELY_DRIVING_SCL_CLOCK,
            true => MST_BUSY_A::ACTIVELY_DRIVING_SCL_CLOCK,
        }
    }
    #[doc = "Device not actively driving SCL clock cycles."]
    #[inline(always)]
    pub fn is_not_actively_driving_scl_clock(&self) -> bool {
        *self == MST_BUSY_A::NOT_ACTIVELY_DRIVING_SCL_CLOCK
    }
    #[doc = "Device operating as master and actively driving SCL clock cycles."]
    #[inline(always)]
    pub fn is_actively_driving_scl_clock(&self) -> bool {
        *self == MST_BUSY_A::ACTIVELY_DRIVING_SCL_CLOCK
    }
}
impl R {
    #[doc = "Bit 0 - Bus Status."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX empty."]
    #[inline(always)]
    pub fn rx_em(&self) -> RX_EM_R {
        RX_EM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Full."]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX Empty."]
    #[inline(always)]
    pub fn tx_em(&self) -> TX_EM_R {
        TX_EM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX Full."]
    #[inline(always)]
    pub fn tx_full(&self) -> TX_FULL_R {
        TX_FULL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Mode."]
    #[inline(always)]
    pub fn mst_busy(&self) -> MST_BUSY_R {
        MST_BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - TX Empty."]
    #[inline(always)]
    #[must_use]
    pub fn tx_em(&mut self) -> TX_EM_W<STATUS_SPEC> {
        TX_EM_W::new(self, 3)
    }
    #[doc = "Bit 4 - TX Full."]
    #[inline(always)]
    #[must_use]
    pub fn tx_full(&mut self) -> TX_FULL_W<STATUS_SPEC> {
        TX_FULL_W::new(self, 4)
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
#[doc = "Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
