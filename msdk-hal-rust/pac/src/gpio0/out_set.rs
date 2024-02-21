#[doc = "Register `OUT_SET` writer"]
pub type W = crate::W<OUT_SET_SPEC>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_OUT_SET_AW {
    #[doc = "0: No Effect."]
    NO = 0,
    #[doc = "1: Set GPIO_OUT bit in this position to '1'"]
    SET = 1,
}
impl From<GPIO_OUT_SET_AW> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_OUT_SET_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GPIO_OUT_SET_AW {
    type Ux = u32;
}
#[doc = "Field `GPIO_OUT_SET` writer - Mask of all of the pins on the port."]
pub type GPIO_OUT_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 32, GPIO_OUT_SET_AW>;
impl<'a, REG> GPIO_OUT_SET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No Effect."]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_OUT_SET_AW::NO)
    }
    #[doc = "Set GPIO_OUT bit in this position to '1'"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_OUT_SET_AW::SET)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_out_set(&mut self) -> GPIO_OUT_SET_W<OUT_SET_SPEC> {
        GPIO_OUT_SET_W::new(self, 0)
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
#[doc = "GPIO Output Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT to 1, without affecting other bits in that register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_SET_SPEC;
impl crate::RegisterSpec for OUT_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out_set::W`](W) writer structure"]
impl crate::Writable for OUT_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_SET to value 0"]
impl crate::Resettable for OUT_SET_SPEC {
    const RESET_VALUE: u32 = 0;
}
