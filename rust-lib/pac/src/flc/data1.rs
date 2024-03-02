#[doc = "Register `DATA1` reader"]
pub type R = crate::R<DATA1_SPEC>;
#[doc = "Register `DATA1` writer"]
pub type W = crate::W<DATA1_SPEC>;
#[doc = "Field `DATA` reader - Data next operation."]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data next operation."]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data next operation."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data next operation."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<DATA1_SPEC> {
        DATA_W::new(self, 0)
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
#[doc = "Flash Write Data for bits 63:32.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA1_SPEC;
impl crate::RegisterSpec for DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data1::R`](R) reader structure"]
impl crate::Readable for DATA1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data1::W`](W) writer structure"]
impl crate::Writable for DATA1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA1 to value 0"]
impl crate::Resettable for DATA1_SPEC {
    const RESET_VALUE: u32 = 0;
}
