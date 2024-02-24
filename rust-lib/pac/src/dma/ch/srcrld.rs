#[doc = "Register `SRCRLD` reader"]
pub type R = crate::R<SRCRLD_SPEC>;
#[doc = "Register `SRCRLD` writer"]
pub type W = crate::W<SRCRLD_SPEC>;
#[doc = "Field `ADDR` reader - Source Address Reload Value."]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Source Address Reload Value."]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - Source Address Reload Value."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - Source Address Reload Value."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<SRCRLD_SPEC> {
        ADDR_W::new(self, 0)
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
#[doc = "Source Address Reload Value. The value of this register is loaded into DMA0_SRC upon a count-to-zero condition.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcrld::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcrld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRCRLD_SPEC;
impl crate::RegisterSpec for SRCRLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcrld::R`](R) reader structure"]
impl crate::Readable for SRCRLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srcrld::W`](W) writer structure"]
impl crate::Writable for SRCRLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCRLD to value 0"]
impl crate::Resettable for SRCRLD_SPEC {
    const RESET_VALUE: u32 = 0;
}
