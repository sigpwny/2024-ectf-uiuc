#[doc = "Register `PADCTRL1` reader"]
pub type R = crate::R<PADCTRL1_SPEC>;
#[doc = "Register `PADCTRL1` writer"]
pub type W = crate::W<PADCTRL1_SPEC>;
#[doc = "Field `GPIO_PADCTRL1` reader - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
pub type GPIO_PADCTRL1_R = crate::FieldReader<GPIO_PADCTRL1_A>;
#[doc = "The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_PADCTRL1_A {
    #[doc = "0: High Impedance."]
    IMPEDANCE = 0,
    #[doc = "1: Weak pull-up mode."]
    PU = 1,
    #[doc = "2: weak pull-down mode."]
    PD = 2,
}
impl From<GPIO_PADCTRL1_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_PADCTRL1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GPIO_PADCTRL1_A {
    type Ux = u32;
}
impl GPIO_PADCTRL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GPIO_PADCTRL1_A> {
        match self.bits {
            0 => Some(GPIO_PADCTRL1_A::IMPEDANCE),
            1 => Some(GPIO_PADCTRL1_A::PU),
            2 => Some(GPIO_PADCTRL1_A::PD),
            _ => None,
        }
    }
    #[doc = "High Impedance."]
    #[inline(always)]
    pub fn is_impedance(&self) -> bool {
        *self == GPIO_PADCTRL1_A::IMPEDANCE
    }
    #[doc = "Weak pull-up mode."]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == GPIO_PADCTRL1_A::PU
    }
    #[doc = "weak pull-down mode."]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == GPIO_PADCTRL1_A::PD
    }
}
#[doc = "Field `GPIO_PADCTRL1` writer - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
pub type GPIO_PADCTRL1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, GPIO_PADCTRL1_A>;
impl<'a, REG> GPIO_PADCTRL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "High Impedance."]
    #[inline(always)]
    pub fn impedance(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_PADCTRL1_A::IMPEDANCE)
    }
    #[doc = "Weak pull-up mode."]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_PADCTRL1_A::PU)
    }
    #[doc = "weak pull-down mode."]
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_PADCTRL1_A::PD)
    }
}
impl R {
    #[doc = "Bits 0:31 - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
    #[inline(always)]
    pub fn gpio_padctrl1(&self) -> GPIO_PADCTRL1_R {
        GPIO_PADCTRL1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_padctrl1(&mut self) -> GPIO_PADCTRL1_W<PADCTRL1_SPEC> {
        GPIO_PADCTRL1_W::new(self, 0)
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
#[doc = "GPIO Input Mode Config 2. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PADCTRL1_SPEC;
impl crate::RegisterSpec for PADCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padctrl1::R`](R) reader structure"]
impl crate::Readable for PADCTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`padctrl1::W`](W) writer structure"]
impl crate::Writable for PADCTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PADCTRL1 to value 0"]
impl crate::Resettable for PADCTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
