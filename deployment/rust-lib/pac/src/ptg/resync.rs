#[doc = "Register `RESYNC` reader"]
pub type R = crate::R<RESYNC_SPEC>;
#[doc = "Register `RESYNC` writer"]
pub type W = crate::W<RESYNC_SPEC>;
#[doc = "Field `pt0` reader - Resync control for PT0"]
pub type PT0_R = crate::BitReader;
#[doc = "Field `pt0` writer - Resync control for PT0"]
pub type PT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pt1` reader - Resync control for PT1"]
pub type PT1_R = crate::BitReader;
#[doc = "Field `pt1` writer - Resync control for PT1"]
pub type PT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pt2` reader - Resync control for PT2"]
pub type PT2_R = crate::BitReader;
#[doc = "Field `pt2` writer - Resync control for PT2"]
pub type PT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pt3` reader - Resync control for PT3"]
pub type PT3_R = crate::BitReader;
#[doc = "Field `pt3` writer - Resync control for PT3"]
pub type PT3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Resync control for PT0"]
    #[inline(always)]
    pub fn pt0(&self) -> PT0_R {
        PT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Resync control for PT1"]
    #[inline(always)]
    pub fn pt1(&self) -> PT1_R {
        PT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Resync control for PT2"]
    #[inline(always)]
    pub fn pt2(&self) -> PT2_R {
        PT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Resync control for PT3"]
    #[inline(always)]
    pub fn pt3(&self) -> PT3_R {
        PT3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Resync control for PT0"]
    #[inline(always)]
    #[must_use]
    pub fn pt0(&mut self) -> PT0_W<RESYNC_SPEC> {
        PT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Resync control for PT1"]
    #[inline(always)]
    #[must_use]
    pub fn pt1(&mut self) -> PT1_W<RESYNC_SPEC> {
        PT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Resync control for PT2"]
    #[inline(always)]
    #[must_use]
    pub fn pt2(&mut self) -> PT2_W<RESYNC_SPEC> {
        PT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Resync control for PT3"]
    #[inline(always)]
    #[must_use]
    pub fn pt3(&mut self) -> PT3_W<RESYNC_SPEC> {
        PT3_W::new(self, 3)
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
#[doc = "Global Resync (All Pulse Trains) Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESYNC_SPEC;
impl crate::RegisterSpec for RESYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resync::R`](R) reader structure"]
impl crate::Readable for RESYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`resync::W`](W) writer structure"]
impl crate::Writable for RESYNC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESYNC to value 0"]
impl crate::Resettable for RESYNC_SPEC {
    const RESET_VALUE: u32 = 0;
}
