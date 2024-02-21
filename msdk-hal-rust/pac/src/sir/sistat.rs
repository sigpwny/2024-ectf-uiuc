#[doc = "Register `SISTAT` reader"]
pub type R = crate::R<SISTAT_SPEC>;
#[doc = "Field `MAGIC` reader - Magic Word Validation. This bit is set by the system initialization block following power-up."]
pub type MAGIC_R = crate::BitReader<MAGIC_A>;
#[doc = "Magic Word Validation. This bit is set by the system initialization block following power-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAGIC_A {
    #[doc = "0: Magic word was not set (OTP has not been initialized properly)."]
    MAGIC_NOT_SET = 0,
    #[doc = "1: Magic word was set (OTP contains valid settings)."]
    MAGIC_SET = 1,
}
impl From<MAGIC_A> for bool {
    #[inline(always)]
    fn from(variant: MAGIC_A) -> Self {
        variant as u8 != 0
    }
}
impl MAGIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MAGIC_A {
        match self.bits {
            false => MAGIC_A::MAGIC_NOT_SET,
            true => MAGIC_A::MAGIC_SET,
        }
    }
    #[doc = "Magic word was not set (OTP has not been initialized properly)."]
    #[inline(always)]
    pub fn is_magic_not_set(&self) -> bool {
        *self == MAGIC_A::MAGIC_NOT_SET
    }
    #[doc = "Magic word was set (OTP contains valid settings)."]
    #[inline(always)]
    pub fn is_magic_set(&self) -> bool {
        *self == MAGIC_A::MAGIC_SET
    }
}
#[doc = "Field `CRCERR` reader - CRC Error Status. This bit is set by the system initialization block following power-up."]
pub type CRCERR_R = crate::BitReader<CRCERR_A>;
#[doc = "CRC Error Status. This bit is set by the system initialization block following power-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCERR_A {
    #[doc = "0: No CRC errors occurred during the read of the OTP memory block."]
    NO_ERROR = 0,
    #[doc = "1: A CRC error occurred while reading the OTP. The address of the failure location in the OTP memory is stored in the ERRADDR register."]
    ERROR = 1,
}
impl From<CRCERR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERR_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCERR_A {
        match self.bits {
            false => CRCERR_A::NO_ERROR,
            true => CRCERR_A::ERROR,
        }
    }
    #[doc = "No CRC errors occurred during the read of the OTP memory block."]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == CRCERR_A::NO_ERROR
    }
    #[doc = "A CRC error occurred while reading the OTP. The address of the failure location in the OTP memory is stored in the ERRADDR register."]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == CRCERR_A::ERROR
    }
}
impl R {
    #[doc = "Bit 0 - Magic Word Validation. This bit is set by the system initialization block following power-up."]
    #[inline(always)]
    pub fn magic(&self) -> MAGIC_R {
        MAGIC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRC Error Status. This bit is set by the system initialization block following power-up."]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "System Initialization Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sistat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SISTAT_SPEC;
impl crate::RegisterSpec for SISTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sistat::R`](R) reader structure"]
impl crate::Readable for SISTAT_SPEC {}
#[doc = "`reset()` method sets SISTAT to value 0"]
impl crate::Resettable for SISTAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
