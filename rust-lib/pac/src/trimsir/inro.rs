#[doc = "Register `INRO` reader"]
pub type R = crate::R<INRO_SPEC>;
#[doc = "Register `INRO` writer"]
pub type W = crate::W<INRO_SPEC>;
#[doc = "Field `TRIM16K` reader - INRO 16KHz Trim."]
pub type TRIM16K_R = crate::FieldReader;
#[doc = "Field `TRIM16K` writer - INRO 16KHz Trim."]
pub type TRIM16K_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRIM30K` reader - INRO 30KHz Trim."]
pub type TRIM30K_R = crate::FieldReader;
#[doc = "Field `TRIM30K` writer - INRO 30KHz Trim."]
pub type TRIM30K_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LPCLKSEL` reader - INRO Low Power Mode Clock Select."]
pub type LPCLKSEL_R = crate::FieldReader<LPCLKSEL_A>;
#[doc = "INRO Low Power Mode Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPCLKSEL_A {
    #[doc = "0: `0`"]
    _8KHZ = 0,
    #[doc = "1: `1`"]
    _16KHZ = 1,
    #[doc = "2: `10`"]
    _30KHZ = 2,
}
impl From<LPCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPCLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPCLKSEL_A {
    type Ux = u8;
}
impl LPCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPCLKSEL_A> {
        match self.bits {
            0 => Some(LPCLKSEL_A::_8KHZ),
            1 => Some(LPCLKSEL_A::_16KHZ),
            2 => Some(LPCLKSEL_A::_30KHZ),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_8khz(&self) -> bool {
        *self == LPCLKSEL_A::_8KHZ
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_16khz(&self) -> bool {
        *self == LPCLKSEL_A::_16KHZ
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_30khz(&self) -> bool {
        *self == LPCLKSEL_A::_30KHZ
    }
}
#[doc = "Field `LPCLKSEL` writer - INRO Low Power Mode Clock Select."]
pub type LPCLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LPCLKSEL_A>;
impl<'a, REG> LPCLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _8khz(self) -> &'a mut crate::W<REG> {
        self.variant(LPCLKSEL_A::_8KHZ)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _16khz(self) -> &'a mut crate::W<REG> {
        self.variant(LPCLKSEL_A::_16KHZ)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _30khz(self) -> &'a mut crate::W<REG> {
        self.variant(LPCLKSEL_A::_30KHZ)
    }
}
impl R {
    #[doc = "Bits 0:2 - INRO 16KHz Trim."]
    #[inline(always)]
    pub fn trim16k(&self) -> TRIM16K_R {
        TRIM16K_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - INRO 30KHz Trim."]
    #[inline(always)]
    pub fn trim30k(&self) -> TRIM30K_R {
        TRIM30K_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - INRO Low Power Mode Clock Select."]
    #[inline(always)]
    pub fn lpclksel(&self) -> LPCLKSEL_R {
        LPCLKSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - INRO 16KHz Trim."]
    #[inline(always)]
    #[must_use]
    pub fn trim16k(&mut self) -> TRIM16K_W<INRO_SPEC> {
        TRIM16K_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - INRO 30KHz Trim."]
    #[inline(always)]
    #[must_use]
    pub fn trim30k(&mut self) -> TRIM30K_W<INRO_SPEC> {
        TRIM30K_W::new(self, 3)
    }
    #[doc = "Bits 6:7 - INRO Low Power Mode Clock Select."]
    #[inline(always)]
    #[must_use]
    pub fn lpclksel(&mut self) -> LPCLKSEL_W<INRO_SPEC> {
        LPCLKSEL_W::new(self, 6)
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
#[doc = "RTC Trim System Initialization Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inro::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inro::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INRO_SPEC;
impl crate::RegisterSpec for INRO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inro::R`](R) reader structure"]
impl crate::Readable for INRO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inro::W`](W) writer structure"]
impl crate::Writable for INRO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INRO to value 0"]
impl crate::Resettable for INRO_SPEC {
    const RESET_VALUE: u32 = 0;
}
