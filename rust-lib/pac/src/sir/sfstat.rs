#[doc = "Register `SFSTAT` reader"]
pub type R = crate::R<SFSTAT_SPEC>;
#[doc = "Field `TRNG` reader - TRNG Function."]
pub type TRNG_R = crate::BitReader<TRNG_A>;
#[doc = "TRNG Function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRNG_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<TRNG_A> for bool {
    #[inline(always)]
    fn from(variant: TRNG_A) -> Self {
        variant as u8 != 0
    }
}
impl TRNG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRNG_A {
        match self.bits {
            false => TRNG_A::NO,
            true => TRNG_A::YES,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == TRNG_A::NO
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == TRNG_A::YES
    }
}
#[doc = "Field `AES` reader - AES Block."]
pub type AES_R = crate::BitReader<AES_A>;
#[doc = "AES Block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AES_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<AES_A> for bool {
    #[inline(always)]
    fn from(variant: AES_A) -> Self {
        variant as u8 != 0
    }
}
impl AES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AES_A {
        match self.bits {
            false => AES_A::NO,
            true => AES_A::YES,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == AES_A::NO
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == AES_A::YES
    }
}
impl R {
    #[doc = "Bit 0 - TRNG Function."]
    #[inline(always)]
    pub fn trng(&self) -> TRNG_R {
        TRNG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - AES Block."]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Security function status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SFSTAT_SPEC;
impl crate::RegisterSpec for SFSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfstat::R`](R) reader structure"]
impl crate::Readable for SFSTAT_SPEC {}
#[doc = "`reset()` method sets SFSTAT to value 0"]
impl crate::Resettable for SFSTAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
