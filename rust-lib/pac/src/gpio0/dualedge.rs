#[doc = "Register `DUALEDGE` reader"]
pub type R = crate::R<DUALEDGE_SPEC>;
#[doc = "Register `DUALEDGE` writer"]
pub type W = crate::W<DUALEDGE_SPEC>;
#[doc = "Field `GPIO_DUALEDGE` reader - Mask of all of the pins on the port."]
pub type GPIO_DUALEDGE_R = crate::FieldReader<GPIO_DUALEDGE_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_DUALEDGE_A {
    #[doc = "0: No Effect."]
    NO = 0,
    #[doc = "1: Dual Edge mode is enabled. If edge-triggered interrupts are enabled on this GPIO pin, then both rising and falling edges will trigger interrupts regardless of the GPIO_INT_POL setting."]
    EN = 1,
}
impl From<GPIO_DUALEDGE_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_DUALEDGE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GPIO_DUALEDGE_A {
    type Ux = u32;
}
impl GPIO_DUALEDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GPIO_DUALEDGE_A> {
        match self.bits {
            0 => Some(GPIO_DUALEDGE_A::NO),
            1 => Some(GPIO_DUALEDGE_A::EN),
            _ => None,
        }
    }
    #[doc = "No Effect."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == GPIO_DUALEDGE_A::NO
    }
    #[doc = "Dual Edge mode is enabled. If edge-triggered interrupts are enabled on this GPIO pin, then both rising and falling edges will trigger interrupts regardless of the GPIO_INT_POL setting."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GPIO_DUALEDGE_A::EN
    }
}
#[doc = "Field `GPIO_DUALEDGE` writer - Mask of all of the pins on the port."]
pub type GPIO_DUALEDGE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, GPIO_DUALEDGE_A>;
impl<'a, REG> GPIO_DUALEDGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No Effect."]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_DUALEDGE_A::NO)
    }
    #[doc = "Dual Edge mode is enabled. If edge-triggered interrupts are enabled on this GPIO pin, then both rising and falling edges will trigger interrupts regardless of the GPIO_INT_POL setting."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_DUALEDGE_A::EN)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_dualedge(&self) -> GPIO_DUALEDGE_R {
        GPIO_DUALEDGE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_dualedge(&mut self) -> GPIO_DUALEDGE_W<DUALEDGE_SPEC> {
        GPIO_DUALEDGE_W::new(self, 0)
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
#[doc = "GPIO Interrupt Dual Edge Mode Register. Each bit in this register selects dual edge mode for the associated GPIO pin in this port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dualedge::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dualedge::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DUALEDGE_SPEC;
impl crate::RegisterSpec for DUALEDGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dualedge::R`](R) reader structure"]
impl crate::Readable for DUALEDGE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dualedge::W`](W) writer structure"]
impl crate::Writable for DUALEDGE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DUALEDGE to value 0"]
impl crate::Resettable for DUALEDGE_SPEC {
    const RESET_VALUE: u32 = 0;
}
