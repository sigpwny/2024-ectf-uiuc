#[doc = "Register `VREGO_C` reader"]
pub type R = crate::R<VREGO_C_SPEC>;
#[doc = "Register `VREGO_C` writer"]
pub type W = crate::W<VREGO_C_SPEC>;
#[doc = "Field `VSETC` reader - Regulator Output Voltage Setting"]
pub type VSETC_R = crate::FieldReader;
#[doc = "Field `VSETC` writer - Regulator Output Voltage Setting"]
pub type VSETC_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RANGEC` reader - Regulator Output Range Set"]
pub type RANGEC_R = crate::BitReader<RANGEC_A>;
#[doc = "Regulator Output Range Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RANGEC_A {
    #[doc = "0: Low output voltage range"]
    LOW = 0,
    #[doc = "1: High output voltage range"]
    HIGH = 1,
}
impl From<RANGEC_A> for bool {
    #[inline(always)]
    fn from(variant: RANGEC_A) -> Self {
        variant as u8 != 0
    }
}
impl RANGEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RANGEC_A {
        match self.bits {
            false => RANGEC_A::LOW,
            true => RANGEC_A::HIGH,
        }
    }
    #[doc = "Low output voltage range"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == RANGEC_A::LOW
    }
    #[doc = "High output voltage range"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == RANGEC_A::HIGH
    }
}
#[doc = "Field `RANGEC` writer - Regulator Output Range Set"]
pub type RANGEC_W<'a, REG> = crate::BitWriter<'a, REG, RANGEC_A>;
impl<'a, REG> RANGEC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low output voltage range"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(RANGEC_A::LOW)
    }
    #[doc = "High output voltage range"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(RANGEC_A::HIGH)
    }
}
impl R {
    #[doc = "Bits 0:6 - Regulator Output Voltage Setting"]
    #[inline(always)]
    pub fn vsetc(&self) -> VSETC_R {
        VSETC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Regulator Output Range Set"]
    #[inline(always)]
    pub fn rangec(&self) -> RANGEC_R {
        RANGEC_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Regulator Output Voltage Setting"]
    #[inline(always)]
    #[must_use]
    pub fn vsetc(&mut self) -> VSETC_W<VREGO_C_SPEC> {
        VSETC_W::new(self, 0)
    }
    #[doc = "Bit 7 - Regulator Output Range Set"]
    #[inline(always)]
    #[must_use]
    pub fn rangec(&mut self) -> RANGEC_W<VREGO_C_SPEC> {
        RANGEC_W::new(self, 7)
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
#[doc = "Buck Voltage Regulator C Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vrego_c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vrego_c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VREGO_C_SPEC;
impl crate::RegisterSpec for VREGO_C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vrego_c::R`](R) reader structure"]
impl crate::Readable for VREGO_C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vrego_c::W`](W) writer structure"]
impl crate::Writable for VREGO_C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VREGO_C to value 0"]
impl crate::Resettable for VREGO_C_SPEC {
    const RESET_VALUE: u32 = 0;
}
