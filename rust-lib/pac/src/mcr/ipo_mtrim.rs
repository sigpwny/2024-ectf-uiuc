#[doc = "Register `IPO_MTRIM` reader"]
pub type R = crate::R<IPO_MTRIM_SPEC>;
#[doc = "Register `IPO_MTRIM` writer"]
pub type W = crate::W<IPO_MTRIM_SPEC>;
#[doc = "Field `MTRIM` reader - Manual Trim Value."]
pub type MTRIM_R = crate::FieldReader;
#[doc = "Field `MTRIM` writer - Manual Trim Value."]
pub type MTRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRIM_RANGE` reader - Trim Range Select."]
pub type TRIM_RANGE_R = crate::BitReader;
#[doc = "Field `TRIM_RANGE` writer - Trim Range Select."]
pub type TRIM_RANGE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Manual Trim Value."]
    #[inline(always)]
    pub fn mtrim(&self) -> MTRIM_R {
        MTRIM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Trim Range Select."]
    #[inline(always)]
    pub fn trim_range(&self) -> TRIM_RANGE_R {
        TRIM_RANGE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Manual Trim Value."]
    #[inline(always)]
    #[must_use]
    pub fn mtrim(&mut self) -> MTRIM_W<IPO_MTRIM_SPEC> {
        MTRIM_W::new(self, 0)
    }
    #[doc = "Bit 8 - Trim Range Select."]
    #[inline(always)]
    #[must_use]
    pub fn trim_range(&mut self) -> TRIM_RANGE_W<IPO_MTRIM_SPEC> {
        TRIM_RANGE_W::new(self, 8)
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
#[doc = "IPO Manual Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipo_mtrim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipo_mtrim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPO_MTRIM_SPEC;
impl crate::RegisterSpec for IPO_MTRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipo_mtrim::R`](R) reader structure"]
impl crate::Readable for IPO_MTRIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ipo_mtrim::W`](W) writer structure"]
impl crate::Writable for IPO_MTRIM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPO_MTRIM to value 0"]
impl crate::Resettable for IPO_MTRIM_SPEC {
    const RESET_VALUE: u32 = 0;
}
