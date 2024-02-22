#[doc = "Register `INTEN_CLR` reader"]
pub type R = crate::R<INTEN_CLR_SPEC>;
#[doc = "Register `INTEN_CLR` writer"]
pub type W = crate::W<INTEN_CLR_SPEC>;
#[doc = "Field `GPIO_INTEN_CLR` reader - Mask of all of the pins on the port."]
pub type GPIO_INTEN_CLR_R = crate::FieldReader<GPIO_INTEN_CLR_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_INTEN_CLR_A {
    #[doc = "0: No Effect."]
    NO = 0,
    #[doc = "1: Clear GPIO_INT_EN bit in this position to '0'"]
    CLEAR = 1,
}
impl From<GPIO_INTEN_CLR_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INTEN_CLR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GPIO_INTEN_CLR_A {
    type Ux = u32;
}
impl GPIO_INTEN_CLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GPIO_INTEN_CLR_A> {
        match self.bits {
            0 => Some(GPIO_INTEN_CLR_A::NO),
            1 => Some(GPIO_INTEN_CLR_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "No Effect."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == GPIO_INTEN_CLR_A::NO
    }
    #[doc = "Clear GPIO_INT_EN bit in this position to '0'"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO_INTEN_CLR_A::CLEAR
    }
}
#[doc = "Field `GPIO_INTEN_CLR` writer - Mask of all of the pins on the port."]
pub type GPIO_INTEN_CLR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, GPIO_INTEN_CLR_A>;
impl<'a, REG> GPIO_INTEN_CLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No Effect."]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_INTEN_CLR_A::NO)
    }
    #[doc = "Clear GPIO_INT_EN bit in this position to '0'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_INTEN_CLR_A::CLEAR)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_inten_clr(&self) -> GPIO_INTEN_CLR_R {
        GPIO_INTEN_CLR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_inten_clr(&mut self) -> GPIO_INTEN_CLR_W<INTEN_CLR_SPEC> {
        GPIO_INTEN_CLR_W::new(self, 0)
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
#[doc = "GPIO Interrupt Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_EN to 0, without affecting other bits in that register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_CLR_SPEC;
impl crate::RegisterSpec for INTEN_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten_clr::R`](R) reader structure"]
impl crate::Readable for INTEN_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten_clr::W`](W) writer structure"]
impl crate::Writable for INTEN_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN_CLR to value 0"]
impl crate::Resettable for INTEN_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
