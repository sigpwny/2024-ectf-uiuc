#[doc = "Register `HYSEN` reader"]
pub type R = crate::R<HYSEN_SPEC>;
#[doc = "Register `HYSEN` writer"]
pub type W = crate::W<HYSEN_SPEC>;
#[doc = "Field `GPIO_HYSEN` reader - Mask of all of the pins on the port."]
pub type GPIO_HYSEN_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_HYSEN` writer - Mask of all of the pins on the port."]
pub type GPIO_HYSEN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_hysen(&self) -> GPIO_HYSEN_R {
        GPIO_HYSEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_hysen(&mut self) -> GPIO_HYSEN_W<HYSEN_SPEC> {
        GPIO_HYSEN_W::new(self, 0)
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
#[doc = "GPIO Input Hysteresis Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hysen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hysen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HYSEN_SPEC;
impl crate::RegisterSpec for HYSEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hysen::R`](R) reader structure"]
impl crate::Readable for HYSEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hysen::W`](W) writer structure"]
impl crate::Writable for HYSEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HYSEN to value 0"]
impl crate::Resettable for HYSEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
