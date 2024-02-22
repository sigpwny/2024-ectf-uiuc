#[doc = "Register `BUCK_OUT_READY` reader"]
pub type R = crate::R<BUCK_OUT_READY_SPEC>;
#[doc = "Field `BUCKOUTRDYA` reader - When set, indicates that the output voltage has reached its regulated value"]
pub type BUCKOUTRDYA_R = crate::BitReader<BUCKOUTRDYA_A>;
#[doc = "When set, indicates that the output voltage has reached its regulated value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUCKOUTRDYA_A {
    #[doc = "0: Output voltage not in range"]
    NOTRDY = 0,
    #[doc = "1: Output voltage in range"]
    RDY = 1,
}
impl From<BUCKOUTRDYA_A> for bool {
    #[inline(always)]
    fn from(variant: BUCKOUTRDYA_A) -> Self {
        variant as u8 != 0
    }
}
impl BUCKOUTRDYA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUCKOUTRDYA_A {
        match self.bits {
            false => BUCKOUTRDYA_A::NOTRDY,
            true => BUCKOUTRDYA_A::RDY,
        }
    }
    #[doc = "Output voltage not in range"]
    #[inline(always)]
    pub fn is_notrdy(&self) -> bool {
        *self == BUCKOUTRDYA_A::NOTRDY
    }
    #[doc = "Output voltage in range"]
    #[inline(always)]
    pub fn is_rdy(&self) -> bool {
        *self == BUCKOUTRDYA_A::RDY
    }
}
#[doc = "Field `BUCKOUTRDYB` reader - When set, indicates that the output voltage has reached its regulated value"]
pub use BUCKOUTRDYA_R as BUCKOUTRDYB_R;
#[doc = "Field `BUCKOUTRDYC` reader - When set, indicates that the output voltage has reached its regulated value"]
pub use BUCKOUTRDYA_R as BUCKOUTRDYC_R;
#[doc = "Field `BUCKOUTRDYD` reader - When set, indicates that the output voltage has reached its regulated value"]
pub use BUCKOUTRDYA_R as BUCKOUTRDYD_R;
impl R {
    #[doc = "Bit 0 - When set, indicates that the output voltage has reached its regulated value"]
    #[inline(always)]
    pub fn buckoutrdya(&self) -> BUCKOUTRDYA_R {
        BUCKOUTRDYA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set, indicates that the output voltage has reached its regulated value"]
    #[inline(always)]
    pub fn buckoutrdyb(&self) -> BUCKOUTRDYB_R {
        BUCKOUTRDYB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set, indicates that the output voltage has reached its regulated value"]
    #[inline(always)]
    pub fn buckoutrdyc(&self) -> BUCKOUTRDYC_R {
        BUCKOUTRDYC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set, indicates that the output voltage has reached its regulated value"]
    #[inline(always)]
    pub fn buckoutrdyd(&self) -> BUCKOUTRDYD_R {
        BUCKOUTRDYD_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Buck Regulator Output Ready Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buck_out_ready::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUCK_OUT_READY_SPEC;
impl crate::RegisterSpec for BUCK_OUT_READY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buck_out_ready::R`](R) reader structure"]
impl crate::Readable for BUCK_OUT_READY_SPEC {}
#[doc = "`reset()` method sets BUCK_OUT_READY to value 0"]
impl crate::Resettable for BUCK_OUT_READY_SPEC {
    const RESET_VALUE: u32 = 0;
}
