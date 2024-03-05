#[doc = "Register `CNTRLD` reader"]
pub type R = crate::R<CNTRLD_SPEC>;
#[doc = "Register `CNTRLD` writer"]
pub type W = crate::W<CNTRLD_SPEC>;
#[doc = "Field `CNT` reader - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
pub type CNT_R = crate::FieldReader<u32>;
#[doc = "Field `CNT` writer - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `EN` reader - Count Reload Enable."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Count Reload Enable."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 31 - Count Reload Enable."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<CNTRLD_SPEC> {
        CNT_W::new(self, 0)
    }
    #[doc = "Bit 31 - Count Reload Enable."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CNTRLD_SPEC> {
        EN_W::new(self, 31)
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
#[doc = "DMA Channel Count Reload Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntrld::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntrld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTRLD_SPEC;
impl crate::RegisterSpec for CNTRLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntrld::R`](R) reader structure"]
impl crate::Readable for CNTRLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cntrld::W`](W) writer structure"]
impl crate::Writable for CNTRLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTRLD to value 0"]
impl crate::Resettable for CNTRLD_SPEC {
    const RESET_VALUE: u32 = 0;
}
