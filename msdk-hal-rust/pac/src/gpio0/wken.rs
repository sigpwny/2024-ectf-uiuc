#[doc = "Register `WKEN` reader"]
pub type R = crate::R<WKEN_SPEC>;
#[doc = "Register `WKEN` writer"]
pub type W = crate::W<WKEN_SPEC>;
#[doc = "Field `GPIO_WKEN` reader - Mask of all of the pins on the port."]
pub type GPIO_WKEN_R = crate::FieldReader<GPIO_WKEN_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_WKEN_A {
    #[doc = "0: PMU wakeup for this GPIO is disabled."]
    DIS = 0,
    #[doc = "1: PMU wakeup for this GPIO is enabled."]
    EN = 1,
}
impl From<GPIO_WKEN_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_WKEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GPIO_WKEN_A {
    type Ux = u32;
}
impl GPIO_WKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GPIO_WKEN_A> {
        match self.bits {
            0 => Some(GPIO_WKEN_A::DIS),
            1 => Some(GPIO_WKEN_A::EN),
            _ => None,
        }
    }
    #[doc = "PMU wakeup for this GPIO is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO_WKEN_A::DIS
    }
    #[doc = "PMU wakeup for this GPIO is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GPIO_WKEN_A::EN
    }
}
#[doc = "Field `GPIO_WKEN` writer - Mask of all of the pins on the port."]
pub type GPIO_WKEN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, GPIO_WKEN_A>;
impl<'a, REG> GPIO_WKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "PMU wakeup for this GPIO is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_WKEN_A::DIS)
    }
    #[doc = "PMU wakeup for this GPIO is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_WKEN_A::EN)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_wken(&self) -> GPIO_WKEN_R {
        GPIO_WKEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_wken(&mut self) -> GPIO_WKEN_W<WKEN_SPEC> {
        GPIO_WKEN_W::new(self, 0)
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
#[doc = "GPIO Wake Enable Register. Each bit in this register controls the PMU wakeup enable for the associated GPIO pin in this port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wken::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wken::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WKEN_SPEC;
impl crate::RegisterSpec for WKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wken::R`](R) reader structure"]
impl crate::Readable for WKEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wken::W`](W) writer structure"]
impl crate::Writable for WKEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WKEN to value 0"]
impl crate::Resettable for WKEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
