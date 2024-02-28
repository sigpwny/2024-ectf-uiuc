#[doc = "Register `MEMZ` reader"]
pub type R = crate::R<MEMZ_SPEC>;
#[doc = "Register `MEMZ` writer"]
pub type W = crate::W<MEMZ_SPEC>;
#[doc = "Field `RAM0` reader - System RAM Block 0 Zeroization."]
pub type RAM0_R = crate::BitReader<RAM0_A>;
#[doc = "System RAM Block 0 Zeroization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM0_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<RAM0_A> for bool {
    #[inline(always)]
    fn from(variant: RAM0_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RAM0_A {
        match self.bits {
            false => RAM0_A::NOP,
            true => RAM0_A::START,
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == RAM0_A::NOP
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == RAM0_A::START
    }
}
#[doc = "Field `RAM0` writer - System RAM Block 0 Zeroization."]
pub type RAM0_W<'a, REG> = crate::BitWriter<'a, REG, RAM0_A>;
impl<'a, REG> RAM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(RAM0_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(RAM0_A::START)
    }
}
#[doc = "Field `RAM1` reader - System RAM Block 1 Zeroization."]
pub use RAM0_R as RAM1_R;
#[doc = "Field `RAM2` reader - System RAM Block 2 Zeroization."]
pub use RAM0_R as RAM2_R;
#[doc = "Field `RAM3` reader - System RAM Block 3 Zeroization."]
pub use RAM0_R as RAM3_R;
#[doc = "Field `SYSRAM0ECC` reader - System RAM 0 ECC Zeroization."]
pub use RAM0_R as SYSRAM0ECC_R;
#[doc = "Field `ICC0` reader - Instruction Cachei 0 Zeroization."]
pub use RAM0_R as ICC0_R;
#[doc = "Field `ICC1` reader - Instruction Cachei 1 Zeroization."]
pub use RAM0_R as ICC1_R;
#[doc = "Field `RAM1` writer - System RAM Block 1 Zeroization."]
pub use RAM0_W as RAM1_W;
#[doc = "Field `RAM2` writer - System RAM Block 2 Zeroization."]
pub use RAM0_W as RAM2_W;
#[doc = "Field `RAM3` writer - System RAM Block 3 Zeroization."]
pub use RAM0_W as RAM3_W;
#[doc = "Field `SYSRAM0ECC` writer - System RAM 0 ECC Zeroization."]
pub use RAM0_W as SYSRAM0ECC_W;
#[doc = "Field `ICC0` writer - Instruction Cachei 0 Zeroization."]
pub use RAM0_W as ICC0_W;
#[doc = "Field `ICC1` writer - Instruction Cachei 1 Zeroization."]
pub use RAM0_W as ICC1_W;
impl R {
    #[doc = "Bit 0 - System RAM Block 0 Zeroization."]
    #[inline(always)]
    pub fn ram0(&self) -> RAM0_R {
        RAM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - System RAM Block 1 Zeroization."]
    #[inline(always)]
    pub fn ram1(&self) -> RAM1_R {
        RAM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System RAM Block 2 Zeroization."]
    #[inline(always)]
    pub fn ram2(&self) -> RAM2_R {
        RAM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - System RAM Block 3 Zeroization."]
    #[inline(always)]
    pub fn ram3(&self) -> RAM3_R {
        RAM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - System RAM 0 ECC Zeroization."]
    #[inline(always)]
    pub fn sysram0ecc(&self) -> SYSRAM0ECC_R {
        SYSRAM0ECC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Instruction Cachei 0 Zeroization."]
    #[inline(always)]
    pub fn icc0(&self) -> ICC0_R {
        ICC0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Instruction Cachei 1 Zeroization."]
    #[inline(always)]
    pub fn icc1(&self) -> ICC1_R {
        ICC1_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System RAM Block 0 Zeroization."]
    #[inline(always)]
    #[must_use]
    pub fn ram0(&mut self) -> RAM0_W<MEMZ_SPEC> {
        RAM0_W::new(self, 0)
    }
    #[doc = "Bit 1 - System RAM Block 1 Zeroization."]
    #[inline(always)]
    #[must_use]
    pub fn ram1(&mut self) -> RAM1_W<MEMZ_SPEC> {
        RAM1_W::new(self, 1)
    }
    #[doc = "Bit 2 - System RAM Block 2 Zeroization."]
    #[inline(always)]
    #[must_use]
    pub fn ram2(&mut self) -> RAM2_W<MEMZ_SPEC> {
        RAM2_W::new(self, 2)
    }
    #[doc = "Bit 3 - System RAM Block 3 Zeroization."]
    #[inline(always)]
    #[must_use]
    pub fn ram3(&mut self) -> RAM3_W<MEMZ_SPEC> {
        RAM3_W::new(self, 3)
    }
    #[doc = "Bit 4 - System RAM 0 ECC Zeroization."]
    #[inline(always)]
    #[must_use]
    pub fn sysram0ecc(&mut self) -> SYSRAM0ECC_W<MEMZ_SPEC> {
        SYSRAM0ECC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Instruction Cachei 0 Zeroization."]
    #[inline(always)]
    #[must_use]
    pub fn icc0(&mut self) -> ICC0_W<MEMZ_SPEC> {
        ICC0_W::new(self, 5)
    }
    #[doc = "Bit 6 - Instruction Cachei 1 Zeroization."]
    #[inline(always)]
    #[must_use]
    pub fn icc1(&mut self) -> ICC1_W<MEMZ_SPEC> {
        ICC1_W::new(self, 6)
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
#[doc = "Memory Zeroize Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEMZ_SPEC;
impl crate::RegisterSpec for MEMZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memz::R`](R) reader structure"]
impl crate::Readable for MEMZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`memz::W`](W) writer structure"]
impl crate::Writable for MEMZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMZ to value 0"]
impl crate::Resettable for MEMZ_SPEC {
    const RESET_VALUE: u32 = 0;
}
