#[doc = "Register `CTRL_STAT` reader"]
pub type R = crate::R<CTRL_STAT_SPEC>;
#[doc = "Register `CTRL_STAT` writer"]
pub type W = crate::W<CTRL_STAT_SPEC>;
#[doc = "Field `start_ow_reset` reader - Start OW Reset."]
pub type START_OW_RESET_R = crate::BitReader;
#[doc = "Field `start_ow_reset` writer - Start OW Reset."]
pub type START_OW_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sra_mode` reader - SRA Mode."]
pub type SRA_MODE_R = crate::BitReader;
#[doc = "Field `sra_mode` writer - SRA Mode."]
pub type SRA_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_bang_oe` reader - Bit Bang Output Enable."]
pub type BIT_BANG_OE_R = crate::BitReader;
#[doc = "Field `bit_bang_oe` writer - Bit Bang Output Enable."]
pub type BIT_BANG_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ow_input` reader - OW Input State."]
pub type OW_INPUT_R = crate::BitReader;
#[doc = "Field `od_spec_mode` reader - Overdrive Spec Mode."]
pub type OD_SPEC_MODE_R = crate::BitReader;
#[doc = "Field `presence_detect` reader - Presence Pulse Detected."]
pub type PRESENCE_DETECT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Start OW Reset."]
    #[inline(always)]
    pub fn start_ow_reset(&self) -> START_OW_RESET_R {
        START_OW_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRA Mode."]
    #[inline(always)]
    pub fn sra_mode(&self) -> SRA_MODE_R {
        SRA_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit Bang Output Enable."]
    #[inline(always)]
    pub fn bit_bang_oe(&self) -> BIT_BANG_OE_R {
        BIT_BANG_OE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OW Input State."]
    #[inline(always)]
    pub fn ow_input(&self) -> OW_INPUT_R {
        OW_INPUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overdrive Spec Mode."]
    #[inline(always)]
    pub fn od_spec_mode(&self) -> OD_SPEC_MODE_R {
        OD_SPEC_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Presence Pulse Detected."]
    #[inline(always)]
    pub fn presence_detect(&self) -> PRESENCE_DETECT_R {
        PRESENCE_DETECT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start OW Reset."]
    #[inline(always)]
    #[must_use]
    pub fn start_ow_reset(&mut self) -> START_OW_RESET_W<CTRL_STAT_SPEC> {
        START_OW_RESET_W::new(self, 0)
    }
    #[doc = "Bit 1 - SRA Mode."]
    #[inline(always)]
    #[must_use]
    pub fn sra_mode(&mut self) -> SRA_MODE_W<CTRL_STAT_SPEC> {
        SRA_MODE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit Bang Output Enable."]
    #[inline(always)]
    #[must_use]
    pub fn bit_bang_oe(&mut self) -> BIT_BANG_OE_W<CTRL_STAT_SPEC> {
        BIT_BANG_OE_W::new(self, 2)
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
#[doc = "1-Wire Master Control/Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_STAT_SPEC;
impl crate::RegisterSpec for CTRL_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_stat::R`](R) reader structure"]
impl crate::Readable for CTRL_STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl_stat::W`](W) writer structure"]
impl crate::Writable for CTRL_STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_STAT to value 0"]
impl crate::Resettable for CTRL_STAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
