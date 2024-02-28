#[doc = "Register `EN1` reader"]
pub type R = crate::R<EN1_SPEC>;
#[doc = "Register `EN1` writer"]
pub type W = crate::W<EN1_SPEC>;
#[doc = "Field `GPIO_EN1` reader - Mask of all of the pins on the port."]
pub type GPIO_EN1_R = crate::FieldReader<GPIO_EN1_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_EN1_A {
    #[doc = "0: Primary function selected."]
    PRIMARY = 0,
    #[doc = "1: Secondary function selected."]
    SECONDARY = 1,
}
impl From<GPIO_EN1_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_EN1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GPIO_EN1_A {
    type Ux = u32;
}
impl GPIO_EN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GPIO_EN1_A> {
        match self.bits {
            0 => Some(GPIO_EN1_A::PRIMARY),
            1 => Some(GPIO_EN1_A::SECONDARY),
            _ => None,
        }
    }
    #[doc = "Primary function selected."]
    #[inline(always)]
    pub fn is_primary(&self) -> bool {
        *self == GPIO_EN1_A::PRIMARY
    }
    #[doc = "Secondary function selected."]
    #[inline(always)]
    pub fn is_secondary(&self) -> bool {
        *self == GPIO_EN1_A::SECONDARY
    }
}
#[doc = "Field `GPIO_EN1` writer - Mask of all of the pins on the port."]
pub type GPIO_EN1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, GPIO_EN1_A>;
impl<'a, REG> GPIO_EN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Primary function selected."]
    #[inline(always)]
    pub fn primary(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_EN1_A::PRIMARY)
    }
    #[doc = "Secondary function selected."]
    #[inline(always)]
    pub fn secondary(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_EN1_A::SECONDARY)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_en1(&self) -> GPIO_EN1_R {
        GPIO_EN1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_en1(&mut self) -> GPIO_EN1_W<EN1_SPEC> {
        GPIO_EN1_W::new(self, 0)
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
#[doc = "GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EN1_SPEC;
impl crate::RegisterSpec for EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en1::R`](R) reader structure"]
impl crate::Readable for EN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`en1::W`](W) writer structure"]
impl crate::Writable for EN1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EN1 to value 0"]
impl crate::Resettable for EN1_SPEC {
    const RESET_VALUE: u32 = 0;
}
