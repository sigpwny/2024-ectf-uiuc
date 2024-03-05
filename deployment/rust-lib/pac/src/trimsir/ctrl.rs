#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `VDDA_LIMITLO` reader - VDDA Low Trim Limit."]
pub type VDDA_LIMITLO_R = crate::FieldReader;
#[doc = "Field `VDDA_LIMITLO` writer - VDDA Low Trim Limit."]
pub type VDDA_LIMITLO_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `VDDA_LIMITHI` reader - VDDA High Trim Limit."]
pub type VDDA_LIMITHI_R = crate::FieldReader;
#[doc = "Field `VDDA_LIMITHI` writer - VDDA High Trim Limit."]
pub type VDDA_LIMITHI_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `IPO_LIMITHI` reader - IPO High Trim Limit."]
pub type IPO_LIMITHI_R = crate::FieldReader<u16>;
#[doc = "Field `IPO_LIMITHI` writer - IPO High Trim Limit."]
pub type IPO_LIMITHI_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `INRO_SEL` reader - INRO Clock Select."]
pub type INRO_SEL_R = crate::FieldReader<INRO_SEL_A>;
#[doc = "INRO Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INRO_SEL_A {
    #[doc = "0: `0`"]
    _8KHZ = 0,
    #[doc = "1: `1`"]
    _16KHZ = 1,
    #[doc = "2: `10`"]
    _30KHZ = 2,
}
impl From<INRO_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INRO_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INRO_SEL_A {
    type Ux = u8;
}
impl INRO_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INRO_SEL_A> {
        match self.bits {
            0 => Some(INRO_SEL_A::_8KHZ),
            1 => Some(INRO_SEL_A::_16KHZ),
            2 => Some(INRO_SEL_A::_30KHZ),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_8khz(&self) -> bool {
        *self == INRO_SEL_A::_8KHZ
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_16khz(&self) -> bool {
        *self == INRO_SEL_A::_16KHZ
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_30khz(&self) -> bool {
        *self == INRO_SEL_A::_30KHZ
    }
}
#[doc = "Field `INRO_SEL` writer - INRO Clock Select."]
pub type INRO_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, INRO_SEL_A>;
impl<'a, REG> INRO_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _8khz(self) -> &'a mut crate::W<REG> {
        self.variant(INRO_SEL_A::_8KHZ)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _16khz(self) -> &'a mut crate::W<REG> {
        self.variant(INRO_SEL_A::_16KHZ)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _30khz(self) -> &'a mut crate::W<REG> {
        self.variant(INRO_SEL_A::_30KHZ)
    }
}
#[doc = "Field `INRO_TRIM` reader - INRO Clock Trim."]
pub type INRO_TRIM_R = crate::FieldReader;
#[doc = "Field `INRO_TRIM` writer - INRO Clock Trim."]
pub type INRO_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:6 - VDDA Low Trim Limit."]
    #[inline(always)]
    pub fn vdda_limitlo(&self) -> VDDA_LIMITLO_R {
        VDDA_LIMITLO_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - VDDA High Trim Limit."]
    #[inline(always)]
    pub fn vdda_limithi(&self) -> VDDA_LIMITHI_R {
        VDDA_LIMITHI_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 15:23 - IPO High Trim Limit."]
    #[inline(always)]
    pub fn ipo_limithi(&self) -> IPO_LIMITHI_R {
        IPO_LIMITHI_R::new(((self.bits >> 15) & 0x01ff) as u16)
    }
    #[doc = "Bits 24:25 - INRO Clock Select."]
    #[inline(always)]
    pub fn inro_sel(&self) -> INRO_SEL_R {
        INRO_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 29:31 - INRO Clock Trim."]
    #[inline(always)]
    pub fn inro_trim(&self) -> INRO_TRIM_R {
        INRO_TRIM_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - VDDA Low Trim Limit."]
    #[inline(always)]
    #[must_use]
    pub fn vdda_limitlo(&mut self) -> VDDA_LIMITLO_W<CTRL_SPEC> {
        VDDA_LIMITLO_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - VDDA High Trim Limit."]
    #[inline(always)]
    #[must_use]
    pub fn vdda_limithi(&mut self) -> VDDA_LIMITHI_W<CTRL_SPEC> {
        VDDA_LIMITHI_W::new(self, 8)
    }
    #[doc = "Bits 15:23 - IPO High Trim Limit."]
    #[inline(always)]
    #[must_use]
    pub fn ipo_limithi(&mut self) -> IPO_LIMITHI_W<CTRL_SPEC> {
        IPO_LIMITHI_W::new(self, 15)
    }
    #[doc = "Bits 24:25 - INRO Clock Select."]
    #[inline(always)]
    #[must_use]
    pub fn inro_sel(&mut self) -> INRO_SEL_W<CTRL_SPEC> {
        INRO_SEL_W::new(self, 24)
    }
    #[doc = "Bits 29:31 - INRO Clock Trim."]
    #[inline(always)]
    #[must_use]
    pub fn inro_trim(&mut self) -> INRO_TRIM_W<CTRL_SPEC> {
        INRO_TRIM_W::new(self, 29)
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
#[doc = "Control Trim System Initialization Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
