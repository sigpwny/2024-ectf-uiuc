#[doc = "Register `VREGO_A` reader"]
pub type R = crate::R<VREGO_A_SPEC>;
#[doc = "Register `VREGO_A` writer"]
pub type W = crate::W<VREGO_A_SPEC>;
#[doc = "Field `VSETA` reader - Regulator Output Voltage Setting"]
pub type VSETA_R = crate::FieldReader;
#[doc = "Field `VSETA` writer - Regulator Output Voltage Setting"]
pub type VSETA_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RANGEA` reader - Regulator Output Range Set"]
pub type RANGEA_R = crate::BitReader<RANGEA_A>;
#[doc = "Regulator Output Range Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RANGEA_A {
    #[doc = "0: Low output voltage range"]
    LOW = 0,
    #[doc = "1: High output voltage range"]
    HIGH = 1,
}
impl From<RANGEA_A> for bool {
    #[inline(always)]
    fn from(variant: RANGEA_A) -> Self {
        variant as u8 != 0
    }
}
impl RANGEA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RANGEA_A {
        match self.bits {
            false => RANGEA_A::LOW,
            true => RANGEA_A::HIGH,
        }
    }
    #[doc = "Low output voltage range"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == RANGEA_A::LOW
    }
    #[doc = "High output voltage range"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == RANGEA_A::HIGH
    }
}
#[doc = "Field `RANGEA` writer - Regulator Output Range Set"]
pub type RANGEA_W<'a, REG> = crate::BitWriter<'a, REG, RANGEA_A>;
impl<'a, REG> RANGEA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low output voltage range"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(RANGEA_A::LOW)
    }
    #[doc = "High output voltage range"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(RANGEA_A::HIGH)
    }
}
impl R {
    #[doc = "Bits 0:6 - Regulator Output Voltage Setting"]
    #[inline(always)]
    pub fn vseta(&self) -> VSETA_R {
        VSETA_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Regulator Output Range Set"]
    #[inline(always)]
    pub fn rangea(&self) -> RANGEA_R {
        RANGEA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Regulator Output Voltage Setting"]
    #[inline(always)]
    #[must_use]
    pub fn vseta(&mut self) -> VSETA_W<VREGO_A_SPEC> {
        VSETA_W::new(self, 0)
    }
    #[doc = "Bit 7 - Regulator Output Range Set"]
    #[inline(always)]
    #[must_use]
    pub fn rangea(&mut self) -> RANGEA_W<VREGO_A_SPEC> {
        RANGEA_W::new(self, 7)
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
#[doc = "Buck Voltage Regulator A Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vrego_a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vrego_a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VREGO_A_SPEC;
impl crate::RegisterSpec for VREGO_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vrego_a::R`](R) reader structure"]
impl crate::Readable for VREGO_A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vrego_a::W`](W) writer structure"]
impl crate::Writable for VREGO_A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VREGO_A to value 0"]
impl crate::Resettable for VREGO_A_SPEC {
    const RESET_VALUE: u32 = 0;
}
