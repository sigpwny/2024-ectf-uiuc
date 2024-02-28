#[doc = "Register `ECCADDR` reader"]
pub type R = crate::R<ECCADDR_SPEC>;
#[doc = "Register `ECCADDR` writer"]
pub type W = crate::W<ECCADDR_SPEC>;
#[doc = "Field `ECCERRAD` reader - ECC Error Address."]
pub type ECCERRAD_R = crate::FieldReader<u32>;
#[doc = "Field `ECCERRAD` writer - ECC Error Address."]
pub type ECCERRAD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC Error Address."]
    #[inline(always)]
    pub fn eccerrad(&self) -> ECCERRAD_R {
        ECCERRAD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ECC Error Address."]
    #[inline(always)]
    #[must_use]
    pub fn eccerrad(&mut self) -> ECCERRAD_W<ECCADDR_SPEC> {
        ECCERRAD_W::new(self, 0)
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
#[doc = "ECC Error Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCADDR_SPEC;
impl crate::RegisterSpec for ECCADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccaddr::R`](R) reader structure"]
impl crate::Readable for ECCADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eccaddr::W`](W) writer structure"]
impl crate::Writable for ECCADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCADDR to value 0"]
impl crate::Resettable for ECCADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
