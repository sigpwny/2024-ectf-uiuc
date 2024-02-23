#[doc = "Register `OUT_CLR` writer"]
pub type W = crate::W<OUT_CLR_SPEC>;
#[doc = "Field `GPIO_OUT_CLR` writer - Mask of all of the pins on the port."]
pub type GPIO_OUT_CLR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_out_clr(&mut self) -> GPIO_OUT_CLR_W<OUT_CLR_SPEC> {
        GPIO_OUT_CLR_W::new(self, 0)
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
#[doc = "GPIO Output Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT to 0, without affecting other bits in that register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_CLR_SPEC;
impl crate::RegisterSpec for OUT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out_clr::W`](W) writer structure"]
impl crate::Writable for OUT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_CLR to value 0"]
impl crate::Resettable for OUT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
