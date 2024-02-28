#[doc = "Register `CLKHI` reader"]
pub type R = crate::R<CLKHI_SPEC>;
#[doc = "Register `CLKHI` writer"]
pub type W = crate::W<CLKHI_SPEC>;
#[doc = "Field `HI` reader - Clock High. In master mode, these bits define the SCL high period."]
pub type HI_R = crate::FieldReader<u16>;
#[doc = "Field `HI` writer - Clock High. In master mode, these bits define the SCL high period."]
pub type HI_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Clock High. In master mode, these bits define the SCL high period."]
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Clock High. In master mode, these bits define the SCL high period."]
    #[inline(always)]
    #[must_use]
    pub fn hi(&mut self) -> HI_W<CLKHI_SPEC> {
        HI_W::new(self, 0)
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
#[doc = "Clock high Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkhi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkhi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKHI_SPEC;
impl crate::RegisterSpec for CLKHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkhi::R`](R) reader structure"]
impl crate::Readable for CLKHI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkhi::W`](W) writer structure"]
impl crate::Writable for CLKHI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKHI to value 0"]
impl crate::Resettable for CLKHI_SPEC {
    const RESET_VALUE: u32 = 0;
}
