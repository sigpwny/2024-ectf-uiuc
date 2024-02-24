#[doc = "Register `EN0` reader"]
pub type R = crate::R<EN0_SPEC>;
#[doc = "Register `EN0` writer"]
pub type W = crate::W<EN0_SPEC>;
#[doc = "Field `GPIO_EN` reader - Mask of all of the pins on the port."]
pub type GPIO_EN_R = crate::FieldReader<GPIO_EN_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_EN_A {
    #[doc = "0: Alternate function enabled."]
    ALTERNATE = 0,
    #[doc = "1: GPIO function is enabled."]
    GPIO = 1,
}
impl From<GPIO_EN_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_EN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GPIO_EN_A {
    type Ux = u32;
}
impl GPIO_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GPIO_EN_A> {
        match self.bits {
            0 => Some(GPIO_EN_A::ALTERNATE),
            1 => Some(GPIO_EN_A::GPIO),
            _ => None,
        }
    }
    #[doc = "Alternate function enabled."]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == GPIO_EN_A::ALTERNATE
    }
    #[doc = "GPIO function is enabled."]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == GPIO_EN_A::GPIO
    }
}
#[doc = "Field `GPIO_EN` writer - Mask of all of the pins on the port."]
pub type GPIO_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, GPIO_EN_A>;
impl<'a, REG> GPIO_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Alternate function enabled."]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_EN_A::ALTERNATE)
    }
    #[doc = "GPIO function is enabled."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_EN_A::GPIO)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_en(&mut self) -> GPIO_EN_W<EN0_SPEC> {
        GPIO_EN_W::new(self, 0)
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
#[doc = "GPIO Function Enable Register. Each bit controls the GPIO_EN setting for one GPIO pin on the associated port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EN0_SPEC;
impl crate::RegisterSpec for EN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en0::R`](R) reader structure"]
impl crate::Readable for EN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`en0::W`](W) writer structure"]
impl crate::Writable for EN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EN0 to value 0"]
impl crate::Resettable for EN0_SPEC {
    const RESET_VALUE: u32 = 0;
}
