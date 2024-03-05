#[doc = "Register `SAFE_DIS` writer"]
pub type W = crate::W<SAFE_DIS_SPEC>;
#[doc = "Field `PT0` writer - "]
pub type PT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PT1` writer - "]
pub type PT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PT2` writer - "]
pub type PT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PT3` writer - "]
pub type PT3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pt0(&mut self) -> PT0_W<SAFE_DIS_SPEC> {
        PT0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pt1(&mut self) -> PT1_W<SAFE_DIS_SPEC> {
        PT1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pt2(&mut self) -> PT2_W<SAFE_DIS_SPEC> {
        PT2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pt3(&mut self) -> PT3_W<SAFE_DIS_SPEC> {
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
#[doc = "Pulse Train Global Safe Disable.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`safe_dis::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAFE_DIS_SPEC;
impl crate::RegisterSpec for SAFE_DIS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`safe_dis::W`](W) writer structure"]
impl crate::Writable for SAFE_DIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAFE_DIS to value 0"]
impl crate::Resettable for SAFE_DIS_SPEC {
    const RESET_VALUE: u32 = 0;
}
