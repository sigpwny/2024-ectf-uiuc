#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<CLKCTRL_SPEC>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<CLKCTRL_SPEC>;
#[doc = "Field `LO` reader - Low duty cycle control. In timer mode, reload\\[7:0\\]."]
pub type LO_R = crate::FieldReader<LO_A>;
#[doc = "Low duty cycle control. In timer mode, reload\\[7:0\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LO_A {
    #[doc = "0: Duty cycle control of serial clock generation is disabled."]
    DIS = 0,
}
impl From<LO_A> for u8 {
    #[inline(always)]
    fn from(variant: LO_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LO_A {
    type Ux = u8;
}
impl LO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LO_A> {
        match self.bits {
            0 => Some(LO_A::DIS),
            _ => None,
        }
    }
    #[doc = "Duty cycle control of serial clock generation is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == LO_A::DIS
    }
}
#[doc = "Field `LO` writer - Low duty cycle control. In timer mode, reload\\[7:0\\]."]
pub type LO_W<'a, REG> = crate::FieldWriter<'a, REG, 8, LO_A>;
impl<'a, REG> LO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Duty cycle control of serial clock generation is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(LO_A::DIS)
    }
}
#[doc = "Field `HI` reader - High duty cycle control. In timer mode, reload\\[15:8\\]."]
pub type HI_R = crate::FieldReader<HI_A>;
#[doc = "High duty cycle control. In timer mode, reload\\[15:8\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HI_A {
    #[doc = "0: Duty cycle control of serial clock generation is disabled."]
    DIS = 0,
}
impl From<HI_A> for u8 {
    #[inline(always)]
    fn from(variant: HI_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HI_A {
    type Ux = u8;
}
impl HI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HI_A> {
        match self.bits {
            0 => Some(HI_A::DIS),
            _ => None,
        }
    }
    #[doc = "Duty cycle control of serial clock generation is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == HI_A::DIS
    }
}
#[doc = "Field `HI` writer - High duty cycle control. In timer mode, reload\\[15:8\\]."]
pub type HI_W<'a, REG> = crate::FieldWriter<'a, REG, 8, HI_A>;
impl<'a, REG> HI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Duty cycle control of serial clock generation is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(HI_A::DIS)
    }
}
#[doc = "Field `CLKDIV` reader - System Clock scale factor. Scales the AMBA clock by 2^SCALE before generating serial clock."]
pub type CLKDIV_R = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - System Clock scale factor. Scales the AMBA clock by 2^SCALE before generating serial clock."]
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Low duty cycle control. In timer mode, reload\\[7:0\\]."]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - High duty cycle control. In timer mode, reload\\[15:8\\]."]
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - System Clock scale factor. Scales the AMBA clock by 2^SCALE before generating serial clock."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low duty cycle control. In timer mode, reload\\[7:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn lo(&mut self) -> LO_W<CLKCTRL_SPEC> {
        LO_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - High duty cycle control. In timer mode, reload\\[15:8\\]."]
    #[inline(always)]
    #[must_use]
    pub fn hi(&mut self) -> HI_W<CLKCTRL_SPEC> {
        HI_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - System Clock scale factor. Scales the AMBA clock by 2^SCALE before generating serial clock."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<CLKCTRL_SPEC> {
        CLKDIV_W::new(self, 16)
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
#[doc = "Register for controlling SPI clock rate.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKCTRL_SPEC;
impl crate::RegisterSpec for CLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctrl::R`](R) reader structure"]
impl crate::Readable for CLKCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkctrl::W`](W) writer structure"]
impl crate::Writable for CLKCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCTRL to value 0"]
impl crate::Resettable for CLKCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
