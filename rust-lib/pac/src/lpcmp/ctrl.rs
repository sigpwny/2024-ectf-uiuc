#[doc = "Register `CTRL[%s]` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL[%s]` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `EN` reader - Comparator Enable."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Comparator Enable."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL` reader - Polarity Select"]
pub type POL_R = crate::BitReader;
#[doc = "Field `POL` writer - Polarity Select"]
pub type POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_EN` reader - IRQ Enable."]
pub type INT_EN_R = crate::BitReader;
#[doc = "Field `INT_EN` writer - IRQ Enable."]
pub type INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT` reader - Raw Compartor Input."]
pub type OUT_R = crate::BitReader;
#[doc = "Field `OUT` writer - Raw Compartor Input."]
pub type OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_FL` reader - IRQ Flag"]
pub type INT_FL_R = crate::BitReader;
#[doc = "Field `INT_FL` writer - IRQ Flag"]
pub type INT_FL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator Enable."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - Polarity Select"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IRQ Enable."]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - Raw Compartor Input."]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - IRQ Flag"]
    #[inline(always)]
    pub fn int_fl(&self) -> INT_FL_R {
        INT_FL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator Enable."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CTRL_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 5 - Polarity Select"]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<CTRL_SPEC> {
        POL_W::new(self, 5)
    }
    #[doc = "Bit 6 - IRQ Enable."]
    #[inline(always)]
    #[must_use]
    pub fn int_en(&mut self) -> INT_EN_W<CTRL_SPEC> {
        INT_EN_W::new(self, 6)
    }
    #[doc = "Bit 14 - Raw Compartor Input."]
    #[inline(always)]
    #[must_use]
    pub fn out(&mut self) -> OUT_W<CTRL_SPEC> {
        OUT_W::new(self, 14)
    }
    #[doc = "Bit 15 - IRQ Flag"]
    #[inline(always)]
    #[must_use]
    pub fn int_fl(&mut self) -> INT_FL_W<CTRL_SPEC> {
        INT_FL_W::new(self, 15)
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
#[doc = "Comparator Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL[%s]
to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
