#[doc = "Register `OUT` reader"]
pub type R = crate::R<OUT_SPEC>;
#[doc = "Register `OUT` writer"]
pub type W = crate::W<OUT_SPEC>;
#[doc = "Field `GPIO_OUT` reader - Mask of all of the pins on the port."]
pub type GPIO_OUT_R = crate::FieldReader<GPIO_OUT_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_OUT_A {
    #[doc = "0: Drive Logic 0 (low) on GPIO output."]
    LOW = 0,
    #[doc = "1: Drive logic 1 (high) on GPIO output."]
    HIGH = 1,
}
impl From<GPIO_OUT_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_OUT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GPIO_OUT_A {
    type Ux = u32;
}
impl GPIO_OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GPIO_OUT_A> {
        match self.bits {
            0 => Some(GPIO_OUT_A::LOW),
            1 => Some(GPIO_OUT_A::HIGH),
            _ => None,
        }
    }
    #[doc = "Drive Logic 0 (low) on GPIO output."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == GPIO_OUT_A::LOW
    }
    #[doc = "Drive logic 1 (high) on GPIO output."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == GPIO_OUT_A::HIGH
    }
}
#[doc = "Field `GPIO_OUT` writer - Mask of all of the pins on the port."]
pub type GPIO_OUT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, GPIO_OUT_A>;
impl<'a, REG> GPIO_OUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Drive Logic 0 (low) on GPIO output."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_OUT_A::LOW)
    }
    #[doc = "Drive logic 1 (high) on GPIO output."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_OUT_A::HIGH)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_out(&self) -> GPIO_OUT_R {
        GPIO_OUT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_out(&mut self) -> GPIO_OUT_W<OUT_SPEC> {
        GPIO_OUT_W::new(self, 0)
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
#[doc = "GPIO Output Register. Each bit controls the GPIO_OUT setting for one pin in the associated port. This register can be written either directly, or by using the GPIO_OUT_SET and GPIO_OUT_CLR registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_SPEC;
impl crate::RegisterSpec for OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out::R`](R) reader structure"]
impl crate::Readable for OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out::W`](W) writer structure"]
impl crate::Writable for OUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT to value 0"]
impl crate::Resettable for OUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
