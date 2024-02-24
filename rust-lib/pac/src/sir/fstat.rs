#[doc = "Register `FSTAT` reader"]
pub type R = crate::R<FSTAT_SPEC>;
#[doc = "Field `FPU` reader - FPU Function."]
pub type FPU_R = crate::BitReader<FPU_A>;
#[doc = "FPU Function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<FPU_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_A) -> Self {
        variant as u8 != 0
    }
}
impl FPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPU_A {
        match self.bits {
            false => FPU_A::NO,
            true => FPU_A::YES,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == FPU_A::NO
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == FPU_A::YES
    }
}
#[doc = "Field `ADC` reader - 10-bit Sigma Delta ADC."]
pub type ADC_R = crate::BitReader<ADC_A>;
#[doc = "10-bit Sigma Delta ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC_A {
        match self.bits {
            false => ADC_A::NO,
            true => ADC_A::YES,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ADC_A::NO
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == ADC_A::YES
    }
}
#[doc = "Field `SMPHR` reader - SMPHR function."]
pub type SMPHR_R = crate::BitReader<SMPHR_A>;
#[doc = "SMPHR function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPHR_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<SMPHR_A> for bool {
    #[inline(always)]
    fn from(variant: SMPHR_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPHR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPHR_A {
        match self.bits {
            false => SMPHR_A::NO,
            true => SMPHR_A::YES,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SMPHR_A::NO
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SMPHR_A::YES
    }
}
impl R {
    #[doc = "Bit 0 - FPU Function."]
    #[inline(always)]
    pub fn fpu(&self) -> FPU_R {
        FPU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 10-bit Sigma Delta ADC."]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - SMPHR function."]
    #[inline(always)]
    pub fn smphr(&self) -> SMPHR_R {
        SMPHR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "funcstat register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSTAT_SPEC;
impl crate::RegisterSpec for FSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fstat::R`](R) reader structure"]
impl crate::Readable for FSTAT_SPEC {}
#[doc = "`reset()` method sets FSTAT to value 0"]
impl crate::Resettable for FSTAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
