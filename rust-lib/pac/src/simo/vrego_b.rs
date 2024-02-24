#[doc = "Register `VREGO_B` reader"]
pub type R = crate::R<VREGO_B_SPEC>;
#[doc = "Register `VREGO_B` writer"]
pub type W = crate::W<VREGO_B_SPEC>;
#[doc = "Field `VSETB` reader - Regulator Output Voltage Setting"]
pub type VSETB_R = crate::FieldReader;
#[doc = "Field `VSETB` writer - Regulator Output Voltage Setting"]
pub type VSETB_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RANGEB` reader - Regulator Output Range Set"]
pub type RANGEB_R = crate::BitReader<RANGEB_A>;
#[doc = "Regulator Output Range Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RANGEB_A {
    #[doc = "0: Low output voltage range"]
    LOW = 0,
    #[doc = "1: High output voltage range"]
    HIGH = 1,
}
impl From<RANGEB_A> for bool {
    #[inline(always)]
    fn from(variant: RANGEB_A) -> Self {
        variant as u8 != 0
    }
}
impl RANGEB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RANGEB_A {
        match self.bits {
            false => RANGEB_A::LOW,
            true => RANGEB_A::HIGH,
        }
    }
    #[doc = "Low output voltage range"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == RANGEB_A::LOW
    }
    #[doc = "High output voltage range"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == RANGEB_A::HIGH
    }
}
#[doc = "Field `RANGEB` writer - Regulator Output Range Set"]
pub type RANGEB_W<'a, REG> = crate::BitWriter<'a, REG, RANGEB_A>;
impl<'a, REG> RANGEB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low output voltage range"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(RANGEB_A::LOW)
    }
    #[doc = "High output voltage range"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(RANGEB_A::HIGH)
    }
}
impl R {
    #[doc = "Bits 0:6 - Regulator Output Voltage Setting"]
    #[inline(always)]
    pub fn vsetb(&self) -> VSETB_R {
        VSETB_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Regulator Output Range Set"]
    #[inline(always)]
    pub fn rangeb(&self) -> RANGEB_R {
        RANGEB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Regulator Output Voltage Setting"]
    #[inline(always)]
    #[must_use]
    pub fn vsetb(&mut self) -> VSETB_W<VREGO_B_SPEC> {
        VSETB_W::new(self, 0)
    }
    #[doc = "Bit 7 - Regulator Output Range Set"]
    #[inline(always)]
    #[must_use]
    pub fn rangeb(&mut self) -> RANGEB_W<VREGO_B_SPEC> {
        RANGEB_W::new(self, 7)
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
#[doc = "Buck Voltage Regulator B Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vrego_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vrego_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VREGO_B_SPEC;
impl crate::RegisterSpec for VREGO_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vrego_b::R`](R) reader structure"]
impl crate::Readable for VREGO_B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vrego_b::W`](W) writer structure"]
impl crate::Writable for VREGO_B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VREGO_B to value 0"]
impl crate::Resettable for VREGO_B_SPEC {
    const RESET_VALUE: u32 = 0;
}
