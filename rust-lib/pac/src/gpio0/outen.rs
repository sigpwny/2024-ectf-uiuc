#[doc = "Register `OUTEN` reader"]
pub type R = crate::R<OUTEN_SPEC>;
#[doc = "Register `OUTEN` writer"]
pub type W = crate::W<OUTEN_SPEC>;
#[doc = "Field `EN` reader - Mask of all of the pins on the port."]
pub type EN_R = crate::FieldReader<EN_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum EN_A {
    #[doc = "0: GPIO Output Disable"]
    DIS = 0,
    #[doc = "1: GPIO Output Enable"]
    EN = 1,
}
impl From<EN_A> for u32 {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EN_A {
    type Ux = u32;
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EN_A> {
        match self.bits {
            0 => Some(EN_A::DIS),
            1 => Some(EN_A::EN),
            _ => None,
        }
    }
    #[doc = "GPIO Output Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN_A::DIS
    }
    #[doc = "GPIO Output Enable"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN_A::EN
    }
}
#[doc = "Field `EN` writer - Mask of all of the pins on the port."]
pub type EN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, EN_A>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "GPIO Output Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::DIS)
    }
    #[doc = "GPIO Output Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::EN)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<OUTEN_SPEC> {
        EN_W::new(self, 0)
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
#[doc = "GPIO Output Enable Register. Each bit controls the GPIO_OUT_EN setting for one GPIO pin in the associated port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTEN_SPEC;
impl crate::RegisterSpec for OUTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outen::R`](R) reader structure"]
impl crate::Readable for OUTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`outen::W`](W) writer structure"]
impl crate::Writable for OUTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTEN to value 0"]
impl crate::Resettable for OUTEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
