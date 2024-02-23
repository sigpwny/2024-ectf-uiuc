#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `WR` reader - Write. This bit is automatically cleared after the operation."]
pub type WR_R = crate::BitReader<WR_A>;
#[doc = "Write. This bit is automatically cleared after the operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WR_A {
    #[doc = "0: No operation/complete."]
    COMPLETE = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<WR_A> for bool {
    #[inline(always)]
    fn from(variant: WR_A) -> Self {
        variant as u8 != 0
    }
}
impl WR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WR_A {
        match self.bits {
            false => WR_A::COMPLETE,
            true => WR_A::START,
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == WR_A::COMPLETE
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == WR_A::START
    }
}
#[doc = "Field `WR` writer - Write. This bit is automatically cleared after the operation."]
pub type WR_W<'a, REG> = crate::BitWriter<'a, REG, WR_A>;
impl<'a, REG> WR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(WR_A::COMPLETE)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(WR_A::START)
    }
}
#[doc = "Field `ME` reader - Mass Erase. This bit is automatically cleared after the operation."]
pub use WR_R as ME_R;
#[doc = "Field `PGE` reader - Page Erase. This bit is automatically cleared after the operation."]
pub use WR_R as PGE_R;
#[doc = "Field `ME` writer - Mass Erase. This bit is automatically cleared after the operation."]
pub use WR_W as ME_W;
#[doc = "Field `PGE` writer - Page Erase. This bit is automatically cleared after the operation."]
pub use WR_W as PGE_W;
#[doc = "Field `ERASE_CODE` reader - Erase Code. The ERASE_CODE must be set up property before erase operation can be initiated. These bits are automatically cleared after the operation is complete."]
pub type ERASE_CODE_R = crate::FieldReader<ERASE_CODE_A>;
#[doc = "Erase Code. The ERASE_CODE must be set up property before erase operation can be initiated. These bits are automatically cleared after the operation is complete.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ERASE_CODE_A {
    #[doc = "0: No operation."]
    NOP = 0,
    #[doc = "85: Enable Page Erase."]
    ERASE_PAGE = 85,
    #[doc = "170: Enable Mass Erase. The debug port must be enabled."]
    ERASE_ALL = 170,
}
impl From<ERASE_CODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ERASE_CODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ERASE_CODE_A {
    type Ux = u8;
}
impl ERASE_CODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ERASE_CODE_A> {
        match self.bits {
            0 => Some(ERASE_CODE_A::NOP),
            85 => Some(ERASE_CODE_A::ERASE_PAGE),
            170 => Some(ERASE_CODE_A::ERASE_ALL),
            _ => None,
        }
    }
    #[doc = "No operation."]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == ERASE_CODE_A::NOP
    }
    #[doc = "Enable Page Erase."]
    #[inline(always)]
    pub fn is_erase_page(&self) -> bool {
        *self == ERASE_CODE_A::ERASE_PAGE
    }
    #[doc = "Enable Mass Erase. The debug port must be enabled."]
    #[inline(always)]
    pub fn is_erase_all(&self) -> bool {
        *self == ERASE_CODE_A::ERASE_ALL
    }
}
#[doc = "Field `ERASE_CODE` writer - Erase Code. The ERASE_CODE must be set up property before erase operation can be initiated. These bits are automatically cleared after the operation is complete."]
pub type ERASE_CODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8, ERASE_CODE_A>;
impl<'a, REG> ERASE_CODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No operation."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(ERASE_CODE_A::NOP)
    }
    #[doc = "Enable Page Erase."]
    #[inline(always)]
    pub fn erase_page(self) -> &'a mut crate::W<REG> {
        self.variant(ERASE_CODE_A::ERASE_PAGE)
    }
    #[doc = "Enable Mass Erase. The debug port must be enabled."]
    #[inline(always)]
    pub fn erase_all(self) -> &'a mut crate::W<REG> {
        self.variant(ERASE_CODE_A::ERASE_ALL)
    }
}
#[doc = "Field `PEND` reader - Flash Pending. When Flash operation is in progress (busy), Flash reads and writes will fail. When PEND is set, write to all Flash registers, with exception of the Flash interrupt register, are ignored."]
pub type PEND_R = crate::BitReader<PEND_A>;
#[doc = "Flash Pending. When Flash operation is in progress (busy), Flash reads and writes will fail. When PEND is set, write to all Flash registers, with exception of the Flash interrupt register, are ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEND_A {
    #[doc = "0: Idle."]
    IDLE = 0,
    #[doc = "1: Busy."]
    BUSY = 1,
}
impl From<PEND_A> for bool {
    #[inline(always)]
    fn from(variant: PEND_A) -> Self {
        variant as u8 != 0
    }
}
impl PEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEND_A {
        match self.bits {
            false => PEND_A::IDLE,
            true => PEND_A::BUSY,
        }
    }
    #[doc = "Idle."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == PEND_A::IDLE
    }
    #[doc = "Busy."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == PEND_A::BUSY
    }
}
#[doc = "Field `LVE` reader - Low Voltage enable."]
pub type LVE_R = crate::BitReader;
#[doc = "Field `LVE` writer - Low Voltage enable."]
pub type LVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNLOCK` reader - Flash Unlock. The correct unlock code must be written to these four bits before any Flash write or erase operation is allowed."]
pub type UNLOCK_R = crate::FieldReader<UNLOCK_A>;
#[doc = "Flash Unlock. The correct unlock code must be written to these four bits before any Flash write or erase operation is allowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UNLOCK_A {
    #[doc = "2: Flash Unlocked."]
    UNLOCKED = 2,
    #[doc = "3: Flash Locked."]
    LOCKED = 3,
}
impl From<UNLOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: UNLOCK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UNLOCK_A {
    type Ux = u8;
}
impl UNLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UNLOCK_A> {
        match self.bits {
            2 => Some(UNLOCK_A::UNLOCKED),
            3 => Some(UNLOCK_A::LOCKED),
            _ => None,
        }
    }
    #[doc = "Flash Unlocked."]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == UNLOCK_A::UNLOCKED
    }
    #[doc = "Flash Locked."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == UNLOCK_A::LOCKED
    }
}
#[doc = "Field `UNLOCK` writer - Flash Unlock. The correct unlock code must be written to these four bits before any Flash write or erase operation is allowed."]
pub type UNLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 4, UNLOCK_A>;
impl<'a, REG> UNLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Flash Unlocked."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(UNLOCK_A::UNLOCKED)
    }
    #[doc = "Flash Locked."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(UNLOCK_A::LOCKED)
    }
}
impl R {
    #[doc = "Bit 0 - Write. This bit is automatically cleared after the operation."]
    #[inline(always)]
    pub fn wr(&self) -> WR_R {
        WR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mass Erase. This bit is automatically cleared after the operation."]
    #[inline(always)]
    pub fn me(&self) -> ME_R {
        ME_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Page Erase. This bit is automatically cleared after the operation."]
    #[inline(always)]
    pub fn pge(&self) -> PGE_R {
        PGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Erase Code. The ERASE_CODE must be set up property before erase operation can be initiated. These bits are automatically cleared after the operation is complete."]
    #[inline(always)]
    pub fn erase_code(&self) -> ERASE_CODE_R {
        ERASE_CODE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Flash Pending. When Flash operation is in progress (busy), Flash reads and writes will fail. When PEND is set, write to all Flash registers, with exception of the Flash interrupt register, are ignored."]
    #[inline(always)]
    pub fn pend(&self) -> PEND_R {
        PEND_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Low Voltage enable."]
    #[inline(always)]
    pub fn lve(&self) -> LVE_R {
        LVE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Flash Unlock. The correct unlock code must be written to these four bits before any Flash write or erase operation is allowed."]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Write. This bit is automatically cleared after the operation."]
    #[inline(always)]
    #[must_use]
    pub fn wr(&mut self) -> WR_W<CTRL_SPEC> {
        WR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Mass Erase. This bit is automatically cleared after the operation."]
    #[inline(always)]
    #[must_use]
    pub fn me(&mut self) -> ME_W<CTRL_SPEC> {
        ME_W::new(self, 1)
    }
    #[doc = "Bit 2 - Page Erase. This bit is automatically cleared after the operation."]
    #[inline(always)]
    #[must_use]
    pub fn pge(&mut self) -> PGE_W<CTRL_SPEC> {
        PGE_W::new(self, 2)
    }
    #[doc = "Bits 8:15 - Erase Code. The ERASE_CODE must be set up property before erase operation can be initiated. These bits are automatically cleared after the operation is complete."]
    #[inline(always)]
    #[must_use]
    pub fn erase_code(&mut self) -> ERASE_CODE_W<CTRL_SPEC> {
        ERASE_CODE_W::new(self, 8)
    }
    #[doc = "Bit 25 - Low Voltage enable."]
    #[inline(always)]
    #[must_use]
    pub fn lve(&mut self) -> LVE_W<CTRL_SPEC> {
        LVE_W::new(self, 25)
    }
    #[doc = "Bits 28:31 - Flash Unlock. The correct unlock code must be written to these four bits before any Flash write or erase operation is allowed."]
    #[inline(always)]
    #[must_use]
    pub fn unlock(&mut self) -> UNLOCK_W<CTRL_SPEC> {
        UNLOCK_W::new(self, 28)
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
#[doc = "Flash Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
