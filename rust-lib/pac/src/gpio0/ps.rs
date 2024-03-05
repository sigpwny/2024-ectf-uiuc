#[doc = "Register `PS` reader"]
pub type R = crate::R<PS_SPEC>;
#[doc = "Register `PS` writer"]
pub type W = crate::W<PS_SPEC>;
#[doc = "Field `ALL` reader - Mask of all of the pins on the port."]
pub type ALL_R = crate::FieldReader<u32>;
#[doc = "Field `ALL` writer - Mask of all of the pins on the port."]
pub type ALL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn all(&self) -> ALL_R {
        ALL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn all(&mut self) -> ALL_W<PS_SPEC> {
        ALL_W::new(self, 0)
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
#[doc = "GPIO Pull Select Mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ps::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ps::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PS_SPEC;
impl crate::RegisterSpec for PS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ps::R`](R) reader structure"]
impl crate::Readable for PS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ps::W`](W) writer structure"]
impl crate::Writable for PS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PS to value 0"]
impl crate::Resettable for PS_SPEC {
    const RESET_VALUE: u32 = 0;
}
