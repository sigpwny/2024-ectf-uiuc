#[doc = "Register `INTPOL` reader"]
pub type R = crate::R<INTPOL_SPEC>;
#[doc = "Register `INTPOL` writer"]
pub type W = crate::W<INTPOL_SPEC>;
#[doc = "Field `GPIO_INTPOL` reader - Mask of all of the pins on the port."]
pub type GPIO_INTPOL_R = crate::FieldReader<GPIO_INTPOL_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_INTPOL_A {
    #[doc = "0: Interrupts are latched on a falling edge or low level condition for this pin."]
    FALLING = 0,
    #[doc = "1: Interrupts are latched on a rising edge or high condition for this pin."]
    RISING = 1,
}
impl From<GPIO_INTPOL_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INTPOL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GPIO_INTPOL_A {
    type Ux = u32;
}
impl GPIO_INTPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GPIO_INTPOL_A> {
        match self.bits {
            0 => Some(GPIO_INTPOL_A::FALLING),
            1 => Some(GPIO_INTPOL_A::RISING),
            _ => None,
        }
    }
    #[doc = "Interrupts are latched on a falling edge or low level condition for this pin."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == GPIO_INTPOL_A::FALLING
    }
    #[doc = "Interrupts are latched on a rising edge or high condition for this pin."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == GPIO_INTPOL_A::RISING
    }
}
#[doc = "Field `GPIO_INTPOL` writer - Mask of all of the pins on the port."]
pub type GPIO_INTPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, GPIO_INTPOL_A>;
impl<'a, REG> GPIO_INTPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Interrupts are latched on a falling edge or low level condition for this pin."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_INTPOL_A::FALLING)
    }
    #[doc = "Interrupts are latched on a rising edge or high condition for this pin."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_INTPOL_A::RISING)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_intpol(&self) -> GPIO_INTPOL_R {
        GPIO_INTPOL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_intpol(&mut self) -> GPIO_INTPOL_W<INTPOL_SPEC> {
        GPIO_INTPOL_W::new(self, 0)
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
#[doc = "GPIO Interrupt Polarity Register. Each bit in this register controls the interrupt polarity setting for one GPIO pin in the associated port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intpol::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intpol::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTPOL_SPEC;
impl crate::RegisterSpec for INTPOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intpol::R`](R) reader structure"]
impl crate::Readable for INTPOL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intpol::W`](W) writer structure"]
impl crate::Writable for INTPOL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTPOL to value 0"]
impl crate::Resettable for INTPOL_SPEC {
    const RESET_VALUE: u32 = 0;
}
