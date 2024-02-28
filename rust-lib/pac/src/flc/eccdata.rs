#[doc = "Register `ECCDATA` reader"]
pub type R = crate::R<ECCDATA_SPEC>;
#[doc = "Register `ECCDATA` writer"]
pub type W = crate::W<ECCDATA_SPEC>;
#[doc = "Field `EVEN` reader - Error Correction Code Odd Data."]
pub type EVEN_R = crate::FieldReader<u16>;
#[doc = "Field `EVEN` writer - Error Correction Code Odd Data."]
pub type EVEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `ODD` reader - Error Correction Code Even Data."]
pub type ODD_R = crate::FieldReader<u16>;
#[doc = "Field `ODD` writer - Error Correction Code Even Data."]
pub type ODD_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Error Correction Code Odd Data."]
    #[inline(always)]
    pub fn even(&self) -> EVEN_R {
        EVEN_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Error Correction Code Even Data."]
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Error Correction Code Odd Data."]
    #[inline(always)]
    #[must_use]
    pub fn even(&mut self) -> EVEN_W<ECCDATA_SPEC> {
        EVEN_W::new(self, 0)
    }
    #[doc = "Bits 16:24 - Error Correction Code Even Data."]
    #[inline(always)]
    #[must_use]
    pub fn odd(&mut self) -> ODD_W<ECCDATA_SPEC> {
        ODD_W::new(self, 16)
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
#[doc = "ECC Data Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCDATA_SPEC;
impl crate::RegisterSpec for ECCDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccdata::R`](R) reader structure"]
impl crate::Readable for ECCDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eccdata::W`](W) writer structure"]
impl crate::Writable for ECCDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCDATA to value 0"]
impl crate::Resettable for ECCDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
