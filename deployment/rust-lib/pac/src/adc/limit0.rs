#[doc = "Register `LIMIT0` reader"]
pub type R = crate::R<LIMIT0_SPEC>;
#[doc = "Register `LIMIT0` writer"]
pub type W = crate::W<LIMIT0_SPEC>;
#[doc = "Field `ch_lo_limit` reader - Low Limit Threshold"]
pub type CH_LO_LIMIT_R = crate::FieldReader<u16>;
#[doc = "Field `ch_lo_limit` writer - Low Limit Threshold"]
pub type CH_LO_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `ch_hi_limit` reader - High Limit Threshold"]
pub type CH_HI_LIMIT_R = crate::FieldReader<u16>;
#[doc = "Field `ch_hi_limit` writer - High Limit Threshold"]
pub type CH_HI_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `ch_sel` reader - ADC Channel Select"]
pub type CH_SEL_R = crate::FieldReader;
#[doc = "Field `ch_sel` writer - ADC Channel Select"]
pub type CH_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ch_lo_limit_en` reader - Low Limit Monitoring Enable"]
pub type CH_LO_LIMIT_EN_R = crate::BitReader;
#[doc = "Field `ch_lo_limit_en` writer - Low Limit Monitoring Enable"]
pub type CH_LO_LIMIT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_hi_limit_en` reader - High Limit Monitoring Enable"]
pub type CH_HI_LIMIT_EN_R = crate::BitReader;
#[doc = "Field `ch_hi_limit_en` writer - High Limit Monitoring Enable"]
pub type CH_HI_LIMIT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Low Limit Threshold"]
    #[inline(always)]
    pub fn ch_lo_limit(&self) -> CH_LO_LIMIT_R {
        CH_LO_LIMIT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:21 - High Limit Threshold"]
    #[inline(always)]
    pub fn ch_hi_limit(&self) -> CH_HI_LIMIT_R {
        CH_HI_LIMIT_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:28 - ADC Channel Select"]
    #[inline(always)]
    pub fn ch_sel(&self) -> CH_SEL_R {
        CH_SEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - Low Limit Monitoring Enable"]
    #[inline(always)]
    pub fn ch_lo_limit_en(&self) -> CH_LO_LIMIT_EN_R {
        CH_LO_LIMIT_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - High Limit Monitoring Enable"]
    #[inline(always)]
    pub fn ch_hi_limit_en(&self) -> CH_HI_LIMIT_EN_R {
        CH_HI_LIMIT_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Low Limit Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn ch_lo_limit(&mut self) -> CH_LO_LIMIT_W<LIMIT0_SPEC> {
        CH_LO_LIMIT_W::new(self, 0)
    }
    #[doc = "Bits 12:21 - High Limit Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn ch_hi_limit(&mut self) -> CH_HI_LIMIT_W<LIMIT0_SPEC> {
        CH_HI_LIMIT_W::new(self, 12)
    }
    #[doc = "Bits 24:28 - ADC Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch_sel(&mut self) -> CH_SEL_W<LIMIT0_SPEC> {
        CH_SEL_W::new(self, 24)
    }
    #[doc = "Bit 29 - Low Limit Monitoring Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_lo_limit_en(&mut self) -> CH_LO_LIMIT_EN_W<LIMIT0_SPEC> {
        CH_LO_LIMIT_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - High Limit Monitoring Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_hi_limit_en(&mut self) -> CH_HI_LIMIT_EN_W<LIMIT0_SPEC> {
        CH_HI_LIMIT_EN_W::new(self, 30)
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
#[doc = "ADC Limit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`limit0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`limit0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LIMIT0_SPEC;
impl crate::RegisterSpec for LIMIT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`limit0::R`](R) reader structure"]
impl crate::Readable for LIMIT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`limit0::W`](W) writer structure"]
impl crate::Writable for LIMIT0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LIMIT0 to value 0"]
impl crate::Resettable for LIMIT0_SPEC {
    const RESET_VALUE: u32 = 0;
}
