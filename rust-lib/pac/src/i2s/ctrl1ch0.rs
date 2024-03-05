#[doc = "Register `CTRL1CH0` reader"]
pub type R = crate::R<CTRL1CH0_SPEC>;
#[doc = "Register `CTRL1CH0` writer"]
pub type W = crate::W<CTRL1CH0_SPEC>;
#[doc = "Field `BITS_WORD` reader - I2S word length."]
pub type BITS_WORD_R = crate::FieldReader;
#[doc = "Field `BITS_WORD` writer - I2S word length."]
pub type BITS_WORD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EN` reader - I2S clock enable."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - I2S clock enable."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMP_SIZE` reader - I2S sample size length."]
pub type SMP_SIZE_R = crate::FieldReader;
#[doc = "Field `SMP_SIZE` writer - I2S sample size length."]
pub type SMP_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADJUST` reader - LSB/MSB Justify."]
pub type ADJUST_R = crate::BitReader;
#[doc = "Field `ADJUST` writer - LSB/MSB Justify."]
pub type ADJUST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDIV` reader - I2S clock frequency divisor."]
pub type CLKDIV_R = crate::FieldReader<u16>;
#[doc = "Field `CLKDIV` writer - I2S clock frequency divisor."]
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - I2S word length."]
    #[inline(always)]
    pub fn bits_word(&self) -> BITS_WORD_R {
        BITS_WORD_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - I2S clock enable."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:13 - I2S sample size length."]
    #[inline(always)]
    pub fn smp_size(&self) -> SMP_SIZE_R {
        SMP_SIZE_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - LSB/MSB Justify."]
    #[inline(always)]
    pub fn adjust(&self) -> ADJUST_R {
        ADJUST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - I2S clock frequency divisor."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - I2S word length."]
    #[inline(always)]
    #[must_use]
    pub fn bits_word(&mut self) -> BITS_WORD_W<CTRL1CH0_SPEC> {
        BITS_WORD_W::new(self, 0)
    }
    #[doc = "Bit 8 - I2S clock enable."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CTRL1CH0_SPEC> {
        EN_W::new(self, 8)
    }
    #[doc = "Bits 9:13 - I2S sample size length."]
    #[inline(always)]
    #[must_use]
    pub fn smp_size(&mut self) -> SMP_SIZE_W<CTRL1CH0_SPEC> {
        SMP_SIZE_W::new(self, 9)
    }
    #[doc = "Bit 15 - LSB/MSB Justify."]
    #[inline(always)]
    #[must_use]
    pub fn adjust(&mut self) -> ADJUST_W<CTRL1CH0_SPEC> {
        ADJUST_W::new(self, 15)
    }
    #[doc = "Bits 16:31 - I2S clock frequency divisor."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<CTRL1CH0_SPEC> {
        CLKDIV_W::new(self, 16)
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
#[doc = "Local channel Setup.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1ch0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1ch0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1CH0_SPEC;
impl crate::RegisterSpec for CTRL1CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1ch0::R`](R) reader structure"]
impl crate::Readable for CTRL1CH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1ch0::W`](W) writer structure"]
impl crate::Writable for CTRL1CH0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL1CH0 to value 0"]
impl crate::Resettable for CTRL1CH0_SPEC {
    const RESET_VALUE: u32 = 0;
}
