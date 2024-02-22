#[doc = "Register `PADCTRL0` reader"]
pub type R = crate::R<PADCTRL0_SPEC>;
#[doc = "Register `PADCTRL0` writer"]
pub type W = crate::W<PADCTRL0_SPEC>;
#[doc = "Field `GPIO_PADCTRL0` reader - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
pub type GPIO_PADCTRL0_R = crate::FieldReader<GPIO_PADCTRL0_A>;
#[doc = "The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_PADCTRL0_A {
    #[doc = "0: High Impedance."]
    IMPEDANCE = 0,
    #[doc = "1: Weak pull-up mode."]
    PU = 1,
    #[doc = "2: weak pull-down mode."]
    PD = 2,
}
impl From<GPIO_PADCTRL0_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_PADCTRL0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GPIO_PADCTRL0_A {
    type Ux = u32;
}
impl GPIO_PADCTRL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GPIO_PADCTRL0_A> {
        match self.bits {
            0 => Some(GPIO_PADCTRL0_A::IMPEDANCE),
            1 => Some(GPIO_PADCTRL0_A::PU),
            2 => Some(GPIO_PADCTRL0_A::PD),
            _ => None,
        }
    }
    #[doc = "High Impedance."]
    #[inline(always)]
    pub fn is_impedance(&self) -> bool {
        *self == GPIO_PADCTRL0_A::IMPEDANCE
    }
    #[doc = "Weak pull-up mode."]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == GPIO_PADCTRL0_A::PU
    }
    #[doc = "weak pull-down mode."]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == GPIO_PADCTRL0_A::PD
    }
}
#[doc = "Field `GPIO_PADCTRL0` writer - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
pub type GPIO_PADCTRL0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, GPIO_PADCTRL0_A>;
impl<'a, REG> GPIO_PADCTRL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "High Impedance."]
    #[inline(always)]
    pub fn impedance(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_PADCTRL0_A::IMPEDANCE)
    }
    #[doc = "Weak pull-up mode."]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_PADCTRL0_A::PU)
    }
    #[doc = "weak pull-down mode."]
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_PADCTRL0_A::PD)
    }
}
impl R {
    #[doc = "Bits 0:31 - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
    #[inline(always)]
    pub fn gpio_padctrl0(&self) -> GPIO_PADCTRL0_R {
        GPIO_PADCTRL0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_padctrl0(&mut self) -> GPIO_PADCTRL0_W<PADCTRL0_SPEC> {
        GPIO_PADCTRL0_W::new(self, 0)
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
#[doc = "GPIO Input Mode Config 1. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PADCTRL0_SPEC;
impl crate::RegisterSpec for PADCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padctrl0::R`](R) reader structure"]
impl crate::Readable for PADCTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`padctrl0::W`](W) writer structure"]
impl crate::Writable for PADCTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PADCTRL0 to value 0"]
impl crate::Resettable for PADCTRL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
