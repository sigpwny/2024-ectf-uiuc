#[doc = "Register `PCLKDIV` reader"]
pub type R = crate::R<PCLKDIV_SPEC>;
#[doc = "Register `PCLKDIV` writer"]
pub type W = crate::W<PCLKDIV_SPEC>;
#[doc = "Field `ADCFRQ` reader - ADC clock Frequency. These bits define the ADC clock frequency. fADC = fPCLK / (ADCFRQ)"]
pub type ADCFRQ_R = crate::FieldReader;
#[doc = "Field `ADCFRQ` writer - ADC clock Frequency. These bits define the ADC clock frequency. fADC = fPCLK / (ADCFRQ)"]
pub type ADCFRQ_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNNCLKDIV` reader - CNN Clock Divider."]
pub type CNNCLKDIV_R = crate::FieldReader<CNNCLKDIV_A>;
#[doc = "CNN Clock Divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CNNCLKDIV_A {
    #[doc = "0: `0`"]
    DIV2 = 0,
    #[doc = "1: `1`"]
    DIV4 = 1,
    #[doc = "2: `10`"]
    DIV8 = 2,
    #[doc = "3: `11`"]
    DIV16 = 3,
    #[doc = "4: `100`"]
    DIV1 = 4,
}
impl From<CNNCLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CNNCLKDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CNNCLKDIV_A {
    type Ux = u8;
}
impl CNNCLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CNNCLKDIV_A> {
        match self.bits {
            0 => Some(CNNCLKDIV_A::DIV2),
            1 => Some(CNNCLKDIV_A::DIV4),
            2 => Some(CNNCLKDIV_A::DIV8),
            3 => Some(CNNCLKDIV_A::DIV16),
            4 => Some(CNNCLKDIV_A::DIV1),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CNNCLKDIV_A::DIV2
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CNNCLKDIV_A::DIV4
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CNNCLKDIV_A::DIV8
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CNNCLKDIV_A::DIV16
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CNNCLKDIV_A::DIV1
    }
}
#[doc = "Field `CNNCLKDIV` writer - CNN Clock Divider."]
pub type CNNCLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CNNCLKDIV_A>;
impl<'a, REG> CNNCLKDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(CNNCLKDIV_A::DIV2)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(CNNCLKDIV_A::DIV4)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(CNNCLKDIV_A::DIV8)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(CNNCLKDIV_A::DIV16)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(CNNCLKDIV_A::DIV1)
    }
}
#[doc = "Field `CNNCLKSEL` reader - CNN Clock Select."]
pub type CNNCLKSEL_R = crate::BitReader<CNNCLKSEL_A>;
#[doc = "CNN Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNNCLKSEL_A {
    #[doc = "0: `0`"]
    PCLK = 0,
    #[doc = "1: `1`"]
    ISO = 1,
}
impl From<CNNCLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CNNCLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CNNCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CNNCLKSEL_A {
        match self.bits {
            false => CNNCLKSEL_A::PCLK,
            true => CNNCLKSEL_A::ISO,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == CNNCLKSEL_A::PCLK
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == CNNCLKSEL_A::ISO
    }
}
#[doc = "Field `CNNCLKSEL` writer - CNN Clock Select."]
pub type CNNCLKSEL_W<'a, REG> = crate::BitWriter<'a, REG, CNNCLKSEL_A>;
impl<'a, REG> CNNCLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(CNNCLKSEL_A::PCLK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(CNNCLKSEL_A::ISO)
    }
}
impl R {
    #[doc = "Bits 10:13 - ADC clock Frequency. These bits define the ADC clock frequency. fADC = fPCLK / (ADCFRQ)"]
    #[inline(always)]
    pub fn adcfrq(&self) -> ADCFRQ_R {
        ADCFRQ_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:16 - CNN Clock Divider."]
    #[inline(always)]
    pub fn cnnclkdiv(&self) -> CNNCLKDIV_R {
        CNNCLKDIV_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 17 - CNN Clock Select."]
    #[inline(always)]
    pub fn cnnclksel(&self) -> CNNCLKSEL_R {
        CNNCLKSEL_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 10:13 - ADC clock Frequency. These bits define the ADC clock frequency. fADC = fPCLK / (ADCFRQ)"]
    #[inline(always)]
    #[must_use]
    pub fn adcfrq(&mut self) -> ADCFRQ_W<PCLKDIV_SPEC> {
        ADCFRQ_W::new(self, 10)
    }
    #[doc = "Bits 14:16 - CNN Clock Divider."]
    #[inline(always)]
    #[must_use]
    pub fn cnnclkdiv(&mut self) -> CNNCLKDIV_W<PCLKDIV_SPEC> {
        CNNCLKDIV_W::new(self, 14)
    }
    #[doc = "Bit 17 - CNN Clock Select."]
    #[inline(always)]
    #[must_use]
    pub fn cnnclksel(&mut self) -> CNNCLKSEL_W<PCLKDIV_SPEC> {
        CNNCLKSEL_W::new(self, 17)
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
#[doc = "Peripheral Clock Divider.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pclkdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pclkdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCLKDIV_SPEC;
impl crate::RegisterSpec for PCLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pclkdiv::R`](R) reader structure"]
impl crate::Readable for PCLKDIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pclkdiv::W`](W) writer structure"]
impl crate::Writable for PCLKDIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCLKDIV to value 0x01"]
impl crate::Resettable for PCLKDIV_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
