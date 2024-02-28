#[doc = "Register `SSTIME` reader"]
pub type R = crate::R<SSTIME_SPEC>;
#[doc = "Register `SSTIME` writer"]
pub type W = crate::W<SSTIME_SPEC>;
#[doc = "Field `PRE` reader - Slave Select Pre delay 1."]
pub type PRE_R = crate::FieldReader<PRE_A>;
#[doc = "Slave Select Pre delay 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRE_A {
    #[doc = "0: 256 system clocks between SS active and first serial clock edge."]
    _256 = 0,
}
impl From<PRE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRE_A {
    type Ux = u8;
}
impl PRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRE_A> {
        match self.bits {
            0 => Some(PRE_A::_256),
            _ => None,
        }
    }
    #[doc = "256 system clocks between SS active and first serial clock edge."]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == PRE_A::_256
    }
}
#[doc = "Field `PRE` writer - Slave Select Pre delay 1."]
pub type PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 8, PRE_A>;
impl<'a, REG> PRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "256 system clocks between SS active and first serial clock edge."]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(PRE_A::_256)
    }
}
#[doc = "Field `POST` reader - Slave Select Post delay 2."]
pub type POST_R = crate::FieldReader<POST_A>;
#[doc = "Slave Select Post delay 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POST_A {
    #[doc = "0: 256 system clocks between last serial clock edge and SS inactive."]
    _256 = 0,
}
impl From<POST_A> for u8 {
    #[inline(always)]
    fn from(variant: POST_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for POST_A {
    type Ux = u8;
}
impl POST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<POST_A> {
        match self.bits {
            0 => Some(POST_A::_256),
            _ => None,
        }
    }
    #[doc = "256 system clocks between last serial clock edge and SS inactive."]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == POST_A::_256
    }
}
#[doc = "Field `POST` writer - Slave Select Post delay 2."]
pub type POST_W<'a, REG> = crate::FieldWriter<'a, REG, 8, POST_A>;
impl<'a, REG> POST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "256 system clocks between last serial clock edge and SS inactive."]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(POST_A::_256)
    }
}
#[doc = "Field `INACT` reader - Slave Select Inactive delay."]
pub type INACT_R = crate::FieldReader<INACT_A>;
#[doc = "Slave Select Inactive delay.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INACT_A {
    #[doc = "0: 256 system clocks between transactions."]
    _256 = 0,
}
impl From<INACT_A> for u8 {
    #[inline(always)]
    fn from(variant: INACT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INACT_A {
    type Ux = u8;
}
impl INACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INACT_A> {
        match self.bits {
            0 => Some(INACT_A::_256),
            _ => None,
        }
    }
    #[doc = "256 system clocks between transactions."]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == INACT_A::_256
    }
}
#[doc = "Field `INACT` writer - Slave Select Inactive delay."]
pub type INACT_W<'a, REG> = crate::FieldWriter<'a, REG, 8, INACT_A>;
impl<'a, REG> INACT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "256 system clocks between transactions."]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(INACT_A::_256)
    }
}
impl R {
    #[doc = "Bits 0:7 - Slave Select Pre delay 1."]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Slave Select Post delay 2."]
    #[inline(always)]
    pub fn post(&self) -> POST_R {
        POST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Slave Select Inactive delay."]
    #[inline(always)]
    pub fn inact(&self) -> INACT_R {
        INACT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slave Select Pre delay 1."]
    #[inline(always)]
    #[must_use]
    pub fn pre(&mut self) -> PRE_W<SSTIME_SPEC> {
        PRE_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Slave Select Post delay 2."]
    #[inline(always)]
    #[must_use]
    pub fn post(&mut self) -> POST_W<SSTIME_SPEC> {
        POST_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Slave Select Inactive delay."]
    #[inline(always)]
    #[must_use]
    pub fn inact(&mut self) -> INACT_W<SSTIME_SPEC> {
        INACT_W::new(self, 16)
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
#[doc = "Register for controlling SPI peripheral/Slave Select Timing.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sstime::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sstime::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSTIME_SPEC;
impl crate::RegisterSpec for SSTIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstime::R`](R) reader structure"]
impl crate::Readable for SSTIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sstime::W`](W) writer structure"]
impl crate::Writable for SSTIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSTIME to value 0"]
impl crate::Resettable for SSTIME_SPEC {
    const RESET_VALUE: u32 = 0;
}
