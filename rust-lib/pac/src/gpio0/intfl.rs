#[doc = "Register `INTFL` reader"]
pub type R = crate::R<INTFL_SPEC>;
#[doc = "Field `GPIO_INTFL` reader - Mask of all of the pins on the port."]
pub type GPIO_INTFL_R = crate::FieldReader<GPIO_INTFL_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_INTFL_A {
    #[doc = "0: No Interrupt is pending on this GPIO pin."]
    NO = 0,
    #[doc = "1: An Interrupt is pending on this GPIO pin."]
    PENDING = 1,
}
impl From<GPIO_INTFL_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INTFL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GPIO_INTFL_A {
    type Ux = u32;
}
impl GPIO_INTFL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GPIO_INTFL_A> {
        match self.bits {
            0 => Some(GPIO_INTFL_A::NO),
            1 => Some(GPIO_INTFL_A::PENDING),
            _ => None,
        }
    }
    #[doc = "No Interrupt is pending on this GPIO pin."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == GPIO_INTFL_A::NO
    }
    #[doc = "An Interrupt is pending on this GPIO pin."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == GPIO_INTFL_A::PENDING
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_intfl(&self) -> GPIO_INTFL_R {
        GPIO_INTFL_R::new(self.bits)
    }
}
#[doc = "GPIO Interrupt Status Register. Each bit in this register contains the pending interrupt status for the associated GPIO pin in this port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFL_SPEC;
impl crate::RegisterSpec for INTFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intfl::R`](R) reader structure"]
impl crate::Readable for INTFL_SPEC {}
#[doc = "`reset()` method sets INTFL to value 0"]
impl crate::Resettable for INTFL_SPEC {
    const RESET_VALUE: u32 = 0;
}
