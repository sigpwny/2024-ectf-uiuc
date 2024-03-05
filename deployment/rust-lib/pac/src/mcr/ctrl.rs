#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `INRO_EN` reader - INRO Enable."]
pub type INRO_EN_R = crate::BitReader;
#[doc = "Field `INRO_EN` writer - INRO Enable."]
pub type INRO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERTCO_EN` reader - ERTCO Enable."]
pub type ERTCO_EN_R = crate::BitReader;
#[doc = "Field `ERTCO_EN` writer - ERTCO Enable."]
pub type ERTCO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIMO_CLKSCL_EN` reader - SIMO Clock Scaling Enable."]
pub type SIMO_CLKSCL_EN_R = crate::BitReader;
#[doc = "Field `SIMO_CLKSCL_EN` writer - SIMO Clock Scaling Enable."]
pub type SIMO_CLKSCL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIMO_RSTD` reader - SIMO System Reset Disable."]
pub type SIMO_RSTD_R = crate::BitReader;
#[doc = "Field `SIMO_RSTD` writer - SIMO System Reset Disable."]
pub type SIMO_RSTD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - INRO Enable."]
    #[inline(always)]
    pub fn inro_en(&self) -> INRO_EN_R {
        INRO_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ERTCO Enable."]
    #[inline(always)]
    pub fn ertco_en(&self) -> ERTCO_EN_R {
        ERTCO_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SIMO Clock Scaling Enable."]
    #[inline(always)]
    pub fn simo_clkscl_en(&self) -> SIMO_CLKSCL_EN_R {
        SIMO_CLKSCL_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SIMO System Reset Disable."]
    #[inline(always)]
    pub fn simo_rstd(&self) -> SIMO_RSTD_R {
        SIMO_RSTD_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - INRO Enable."]
    #[inline(always)]
    #[must_use]
    pub fn inro_en(&mut self) -> INRO_EN_W<CTRL_SPEC> {
        INRO_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - ERTCO Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ertco_en(&mut self) -> ERTCO_EN_W<CTRL_SPEC> {
        ERTCO_EN_W::new(self, 3)
    }
    #[doc = "Bit 8 - SIMO Clock Scaling Enable."]
    #[inline(always)]
    #[must_use]
    pub fn simo_clkscl_en(&mut self) -> SIMO_CLKSCL_EN_W<CTRL_SPEC> {
        SIMO_CLKSCL_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - SIMO System Reset Disable."]
    #[inline(always)]
    #[must_use]
    pub fn simo_rstd(&mut self) -> SIMO_RSTD_W<CTRL_SPEC> {
        SIMO_RSTD_W::new(self, 9)
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
#[doc = "Miscellaneous Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
