#[doc = "Register `CFG` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `long_line_mode` reader - Long Line Mode."]
pub type LONG_LINE_MODE_R = crate::BitReader;
#[doc = "Field `long_line_mode` writer - Long Line Mode."]
pub type LONG_LINE_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `force_pres_det` reader - Force Line During Presence Detect."]
pub type FORCE_PRES_DET_R = crate::BitReader;
#[doc = "Field `force_pres_det` writer - Force Line During Presence Detect."]
pub type FORCE_PRES_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_bang_en` reader - Bit Bang Enable."]
pub type BIT_BANG_EN_R = crate::BitReader;
#[doc = "Field `bit_bang_en` writer - Bit Bang Enable."]
pub type BIT_BANG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ext_pullup_mode` reader - Provide an extra output control to control an external pullup."]
pub type EXT_PULLUP_MODE_R = crate::BitReader;
#[doc = "Field `ext_pullup_mode` writer - Provide an extra output control to control an external pullup."]
pub type EXT_PULLUP_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ext_pullup_enable` reader - Enable External Pullup."]
pub type EXT_PULLUP_ENABLE_R = crate::BitReader;
#[doc = "Field `ext_pullup_enable` writer - Enable External Pullup."]
pub type EXT_PULLUP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `single_bit_mode` reader - Enable Single Bit TX/RX Mode."]
pub type SINGLE_BIT_MODE_R = crate::BitReader;
#[doc = "Field `single_bit_mode` writer - Enable Single Bit TX/RX Mode."]
pub type SINGLE_BIT_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `overdrive` reader - Enables overdrive speed for 1-Wire operations."]
pub type OVERDRIVE_R = crate::BitReader;
#[doc = "Field `overdrive` writer - Enables overdrive speed for 1-Wire operations."]
pub type OVERDRIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `int_pullup_enable` reader - Enable intenral pullup."]
pub type INT_PULLUP_ENABLE_R = crate::BitReader;
#[doc = "Field `int_pullup_enable` writer - Enable intenral pullup."]
pub type INT_PULLUP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Long Line Mode."]
    #[inline(always)]
    pub fn long_line_mode(&self) -> LONG_LINE_MODE_R {
        LONG_LINE_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force Line During Presence Detect."]
    #[inline(always)]
    pub fn force_pres_det(&self) -> FORCE_PRES_DET_R {
        FORCE_PRES_DET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit Bang Enable."]
    #[inline(always)]
    pub fn bit_bang_en(&self) -> BIT_BANG_EN_R {
        BIT_BANG_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Provide an extra output control to control an external pullup."]
    #[inline(always)]
    pub fn ext_pullup_mode(&self) -> EXT_PULLUP_MODE_R {
        EXT_PULLUP_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable External Pullup."]
    #[inline(always)]
    pub fn ext_pullup_enable(&self) -> EXT_PULLUP_ENABLE_R {
        EXT_PULLUP_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Single Bit TX/RX Mode."]
    #[inline(always)]
    pub fn single_bit_mode(&self) -> SINGLE_BIT_MODE_R {
        SINGLE_BIT_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables overdrive speed for 1-Wire operations."]
    #[inline(always)]
    pub fn overdrive(&self) -> OVERDRIVE_R {
        OVERDRIVE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable intenral pullup."]
    #[inline(always)]
    pub fn int_pullup_enable(&self) -> INT_PULLUP_ENABLE_R {
        INT_PULLUP_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Long Line Mode."]
    #[inline(always)]
    #[must_use]
    pub fn long_line_mode(&mut self) -> LONG_LINE_MODE_W<CFG_SPEC> {
        LONG_LINE_MODE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Force Line During Presence Detect."]
    #[inline(always)]
    #[must_use]
    pub fn force_pres_det(&mut self) -> FORCE_PRES_DET_W<CFG_SPEC> {
        FORCE_PRES_DET_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit Bang Enable."]
    #[inline(always)]
    #[must_use]
    pub fn bit_bang_en(&mut self) -> BIT_BANG_EN_W<CFG_SPEC> {
        BIT_BANG_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Provide an extra output control to control an external pullup."]
    #[inline(always)]
    #[must_use]
    pub fn ext_pullup_mode(&mut self) -> EXT_PULLUP_MODE_W<CFG_SPEC> {
        EXT_PULLUP_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable External Pullup."]
    #[inline(always)]
    #[must_use]
    pub fn ext_pullup_enable(&mut self) -> EXT_PULLUP_ENABLE_W<CFG_SPEC> {
        EXT_PULLUP_ENABLE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Single Bit TX/RX Mode."]
    #[inline(always)]
    #[must_use]
    pub fn single_bit_mode(&mut self) -> SINGLE_BIT_MODE_W<CFG_SPEC> {
        SINGLE_BIT_MODE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enables overdrive speed for 1-Wire operations."]
    #[inline(always)]
    #[must_use]
    pub fn overdrive(&mut self) -> OVERDRIVE_W<CFG_SPEC> {
        OVERDRIVE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable intenral pullup."]
    #[inline(always)]
    #[must_use]
    pub fn int_pullup_enable(&mut self) -> INT_PULLUP_ENABLE_W<CFG_SPEC> {
        INT_PULLUP_ENABLE_W::new(self, 7)
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
#[doc = "1-Wire Master Configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
