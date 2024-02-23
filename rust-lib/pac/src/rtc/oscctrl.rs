#[doc = "Register `OSCCTRL` reader"]
pub type R = crate::R<OSCCTRL_SPEC>;
#[doc = "Register `OSCCTRL` writer"]
pub type W = crate::W<OSCCTRL_SPEC>;
#[doc = "Field `FILTER_EN` reader - Enables analog deglitch filter."]
pub type FILTER_EN_R = crate::BitReader;
#[doc = "Field `FILTER_EN` writer - Enables analog deglitch filter."]
pub type FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIAS_SEL` reader - If IBIAS_EN is 1, selects 4x,2x mode."]
pub type IBIAS_SEL_R = crate::BitReader;
#[doc = "Field `IBIAS_SEL` writer - If IBIAS_EN is 1, selects 4x,2x mode."]
pub type IBIAS_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST_EN` reader - Enables high current hysteresis buffer."]
pub type HYST_EN_R = crate::BitReader;
#[doc = "Field `HYST_EN` writer - Enables high current hysteresis buffer."]
pub type HYST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIAS_EN` reader - Enables higher 4x,2x current modes."]
pub type IBIAS_EN_R = crate::BitReader;
#[doc = "Field `IBIAS_EN` writer - Enables higher 4x,2x current modes."]
pub type IBIAS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASS` reader - RTC Crystal Bypass"]
pub type BYPASS_R = crate::BitReader;
#[doc = "Field `BYPASS` writer - RTC Crystal Bypass"]
pub type BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SQW_32K` reader - RTC 32kHz Square Wave Output"]
pub type SQW_32K_R = crate::BitReader;
#[doc = "Field `SQW_32K` writer - RTC 32kHz Square Wave Output"]
pub type SQW_32K_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables analog deglitch filter."]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If IBIAS_EN is 1, selects 4x,2x mode."]
    #[inline(always)]
    pub fn ibias_sel(&self) -> IBIAS_SEL_R {
        IBIAS_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables high current hysteresis buffer."]
    #[inline(always)]
    pub fn hyst_en(&self) -> HYST_EN_R {
        HYST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables higher 4x,2x current modes."]
    #[inline(always)]
    pub fn ibias_en(&self) -> IBIAS_EN_R {
        IBIAS_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC Crystal Bypass"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC 32kHz Square Wave Output"]
    #[inline(always)]
    pub fn sqw_32k(&self) -> SQW_32K_R {
        SQW_32K_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables analog deglitch filter."]
    #[inline(always)]
    #[must_use]
    pub fn filter_en(&mut self) -> FILTER_EN_W<OSCCTRL_SPEC> {
        FILTER_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - If IBIAS_EN is 1, selects 4x,2x mode."]
    #[inline(always)]
    #[must_use]
    pub fn ibias_sel(&mut self) -> IBIAS_SEL_W<OSCCTRL_SPEC> {
        IBIAS_SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enables high current hysteresis buffer."]
    #[inline(always)]
    #[must_use]
    pub fn hyst_en(&mut self) -> HYST_EN_W<OSCCTRL_SPEC> {
        HYST_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enables higher 4x,2x current modes."]
    #[inline(always)]
    #[must_use]
    pub fn ibias_en(&mut self) -> IBIAS_EN_W<OSCCTRL_SPEC> {
        IBIAS_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - RTC Crystal Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<OSCCTRL_SPEC> {
        BYPASS_W::new(self, 4)
    }
    #[doc = "Bit 5 - RTC 32kHz Square Wave Output"]
    #[inline(always)]
    #[must_use]
    pub fn sqw_32k(&mut self) -> SQW_32K_W<OSCCTRL_SPEC> {
        SQW_32K_W::new(self, 5)
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
#[doc = "RTC Oscillator Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oscctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCCTRL_SPEC;
impl crate::RegisterSpec for OSCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oscctrl::R`](R) reader structure"]
impl crate::Readable for OSCCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oscctrl::W`](W) writer structure"]
impl crate::Writable for OSCCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSCCTRL to value 0"]
impl crate::Resettable for OSCCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
