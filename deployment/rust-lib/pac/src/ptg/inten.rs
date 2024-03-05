#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `pt0` reader - Pulse Train 0 Stopped Interrupt Enable/Disable"]
pub type PT0_R = crate::BitReader;
#[doc = "Field `pt0` writer - Pulse Train 0 Stopped Interrupt Enable/Disable"]
pub type PT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pt1` reader - Pulse Train 1 Stopped Interrupt Enable/Disable"]
pub type PT1_R = crate::BitReader;
#[doc = "Field `pt1` writer - Pulse Train 1 Stopped Interrupt Enable/Disable"]
pub type PT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pt2` reader - Pulse Train 2 Stopped Interrupt Enable/Disable"]
pub type PT2_R = crate::BitReader;
#[doc = "Field `pt2` writer - Pulse Train 2 Stopped Interrupt Enable/Disable"]
pub type PT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pt3` reader - Pulse Train 3 Stopped Interrupt Enable/Disable"]
pub type PT3_R = crate::BitReader;
#[doc = "Field `pt3` writer - Pulse Train 3 Stopped Interrupt Enable/Disable"]
pub type PT3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pulse Train 0 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt0(&self) -> PT0_R {
        PT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pulse Train 1 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt1(&self) -> PT1_R {
        PT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pulse Train 2 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt2(&self) -> PT2_R {
        PT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pulse Train 3 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt3(&self) -> PT3_R {
        PT3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse Train 0 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pt0(&mut self) -> PT0_W<INTEN_SPEC> {
        PT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pulse Train 1 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pt1(&mut self) -> PT1_W<INTEN_SPEC> {
        PT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pulse Train 2 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pt2(&mut self) -> PT2_W<INTEN_SPEC> {
        PT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pulse Train 3 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pt3(&mut self) -> PT3_W<INTEN_SPEC> {
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
#[doc = "Pulse Train Interrupt Enable/Disable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
