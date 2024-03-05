#[doc = "Register `CMP` reader"]
pub type R = crate::R<CMP_SPEC>;
#[doc = "Register `CMP` writer"]
pub type W = crate::W<CMP_SPEC>;
#[doc = "Field `COMPARE` reader - Timer Compare Value."]
pub type COMPARE_R = crate::FieldReader<u32>;
#[doc = "Field `COMPARE` writer - Timer Compare Value."]
pub type COMPARE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer Compare Value."]
    #[inline(always)]
    pub fn compare(&self) -> COMPARE_R {
        COMPARE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer Compare Value."]
    #[inline(always)]
    #[must_use]
    pub fn compare(&mut self) -> COMPARE_W<CMP_SPEC> {
        COMPARE_W::new(self, 0)
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
#[doc = "Compare. This register stores the compare value, which is used to set the maximum count value to initiate a reload of the timer to 0x0001.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP_SPEC;
impl crate::RegisterSpec for CMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp::R`](R) reader structure"]
impl crate::Readable for CMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmp::W`](W) writer structure"]
impl crate::Writable for CMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP to value 0xffff"]
impl crate::Resettable for CMP_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
