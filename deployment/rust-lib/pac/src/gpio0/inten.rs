#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `GPIO_INTEN` reader - Mask of all of the pins on the port."]
pub type GPIO_INTEN_R = crate::FieldReader<GPIO_INTEN_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_INTEN_A {
    #[doc = "0: Interrupts are disabled for this GPIO pin."]
    DIS = 0,
    #[doc = "1: Interrupts are enabled for this GPIO pin."]
    EN = 1,
}
impl From<GPIO_INTEN_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INTEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GPIO_INTEN_A {
    type Ux = u32;
}
impl GPIO_INTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GPIO_INTEN_A> {
        match self.bits {
            0 => Some(GPIO_INTEN_A::DIS),
            1 => Some(GPIO_INTEN_A::EN),
            _ => None,
        }
    }
    #[doc = "Interrupts are disabled for this GPIO pin."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO_INTEN_A::DIS
    }
    #[doc = "Interrupts are enabled for this GPIO pin."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GPIO_INTEN_A::EN
    }
}
#[doc = "Field `GPIO_INTEN` writer - Mask of all of the pins on the port."]
pub type GPIO_INTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, GPIO_INTEN_A>;
impl<'a, REG> GPIO_INTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Interrupts are disabled for this GPIO pin."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_INTEN_A::DIS)
    }
    #[doc = "Interrupts are enabled for this GPIO pin."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_INTEN_A::EN)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_inten(&self) -> GPIO_INTEN_R {
        GPIO_INTEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_inten(&mut self) -> GPIO_INTEN_W<INTEN_SPEC> {
        GPIO_INTEN_W::new(self, 0)
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
#[doc = "GPIO Interrupt Enable Register. Each bit in this register controls the GPIO interrupt enable for the associated pin on the GPIO port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
