#[doc = "Register `TAP_SEL[%s]` reader"]
pub type R = crate::R<TAP_SEL_SPEC>;
#[doc = "Register `TAP_SEL[%s]` writer"]
pub type W = crate::W<TAP_SEL_SPEC>;
#[doc = "Field `LO` reader - Select delay line tap for lower bound of auto adjustment"]
pub type LO_R = crate::FieldReader;
#[doc = "Field `LO` writer - Select delay line tap for lower bound of auto adjustment"]
pub type LO_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LO_TAP_STAT` reader - Returns last delay line tap value"]
pub type LO_TAP_STAT_R = crate::BitReader;
#[doc = "Field `LO_TAP_STAT` writer - Returns last delay line tap value"]
pub type LO_TAP_STAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR_TAP_STAT` reader - Returns last delay line tap value"]
pub type CTR_TAP_STAT_R = crate::BitReader;
#[doc = "Field `CTR_TAP_STAT` writer - Returns last delay line tap value"]
pub type CTR_TAP_STAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HI_TAP_STAT` reader - Returns last delay line tap value"]
pub type HI_TAP_STAT_R = crate::BitReader;
#[doc = "Field `HI_TAP_STAT` writer - Returns last delay line tap value"]
pub type HI_TAP_STAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HI` reader - Selects delay line tap for high point of auto adjustment"]
pub type HI_R = crate::FieldReader;
#[doc = "Field `HI` writer - Selects delay line tap for high point of auto adjustment"]
pub type HI_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CTR` reader - Selects delay line tap for center point of auto adjustment"]
pub type CTR_R = crate::FieldReader;
#[doc = "Field `CTR` writer - Selects delay line tap for center point of auto adjustment"]
pub type CTR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `COARSE` reader - Selects delay line tap for coarse or fixed delay portion of the line"]
pub type COARSE_R = crate::FieldReader;
#[doc = "Field `COARSE` writer - Selects delay line tap for coarse or fixed delay portion of the line"]
pub type COARSE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DET_DLY` reader - Number of HCLK between delay line launch and sampling"]
pub type DET_DLY_R = crate::FieldReader;
#[doc = "Field `DET_DLY` writer - Number of HCLK between delay line launch and sampling"]
pub type DET_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DELAY_ACT` reader - Set if the delay is active"]
pub type DELAY_ACT_R = crate::BitReader;
#[doc = "Field `DELAY_ACT` writer - Set if the delay is active"]
pub type DELAY_ACT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Select delay line tap for lower bound of auto adjustment"]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Returns last delay line tap value"]
    #[inline(always)]
    pub fn lo_tap_stat(&self) -> LO_TAP_STAT_R {
        LO_TAP_STAT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Returns last delay line tap value"]
    #[inline(always)]
    pub fn ctr_tap_stat(&self) -> CTR_TAP_STAT_R {
        CTR_TAP_STAT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Returns last delay line tap value"]
    #[inline(always)]
    pub fn hi_tap_stat(&self) -> HI_TAP_STAT_R {
        HI_TAP_STAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Selects delay line tap for high point of auto adjustment"]
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Selects delay line tap for center point of auto adjustment"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - Selects delay line tap for coarse or fixed delay portion of the line"]
    #[inline(always)]
    pub fn coarse(&self) -> COARSE_R {
        COARSE_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 29:30 - Number of HCLK between delay line launch and sampling"]
    #[inline(always)]
    pub fn det_dly(&self) -> DET_DLY_R {
        DET_DLY_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Set if the delay is active"]
    #[inline(always)]
    pub fn delay_act(&self) -> DELAY_ACT_R {
        DELAY_ACT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select delay line tap for lower bound of auto adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn lo(&mut self) -> LO_W<TAP_SEL_SPEC> {
        LO_W::new(self, 0)
    }
    #[doc = "Bit 5 - Returns last delay line tap value"]
    #[inline(always)]
    #[must_use]
    pub fn lo_tap_stat(&mut self) -> LO_TAP_STAT_W<TAP_SEL_SPEC> {
        LO_TAP_STAT_W::new(self, 5)
    }
    #[doc = "Bit 6 - Returns last delay line tap value"]
    #[inline(always)]
    #[must_use]
    pub fn ctr_tap_stat(&mut self) -> CTR_TAP_STAT_W<TAP_SEL_SPEC> {
        CTR_TAP_STAT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Returns last delay line tap value"]
    #[inline(always)]
    #[must_use]
    pub fn hi_tap_stat(&mut self) -> HI_TAP_STAT_W<TAP_SEL_SPEC> {
        HI_TAP_STAT_W::new(self, 7)
    }
    #[doc = "Bits 8:12 - Selects delay line tap for high point of auto adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn hi(&mut self) -> HI_W<TAP_SEL_SPEC> {
        HI_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Selects delay line tap for center point of auto adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn ctr(&mut self) -> CTR_W<TAP_SEL_SPEC> {
        CTR_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - Selects delay line tap for coarse or fixed delay portion of the line"]
    #[inline(always)]
    #[must_use]
    pub fn coarse(&mut self) -> COARSE_W<TAP_SEL_SPEC> {
        COARSE_W::new(self, 24)
    }
    #[doc = "Bits 29:30 - Number of HCLK between delay line launch and sampling"]
    #[inline(always)]
    #[must_use]
    pub fn det_dly(&mut self) -> DET_DLY_W<TAP_SEL_SPEC> {
        DET_DLY_W::new(self, 29)
    }
    #[doc = "Bit 31 - Set if the delay is active"]
    #[inline(always)]
    #[must_use]
    pub fn delay_act(&mut self) -> DELAY_ACT_W<TAP_SEL_SPEC> {
        DELAY_ACT_W::new(self, 31)
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
#[doc = "DVS Tap Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tap_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tap_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAP_SEL_SPEC;
impl crate::RegisterSpec for TAP_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tap_sel::R`](R) reader structure"]
impl crate::Readable for TAP_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tap_sel::W`](W) writer structure"]
impl crate::Writable for TAP_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAP_SEL[%s]
to value 0"]
impl crate::Resettable for TAP_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
