#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `TMODE` reader - Timer Mode."]
pub type TMODE_R = crate::FieldReader<TMODE_A>;
#[doc = "Timer Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMODE_A {
    #[doc = "0: One Shot Mode."]
    ONE_SHOT = 0,
    #[doc = "1: Continuous Mode."]
    CONTINUOUS = 1,
    #[doc = "2: Counter Mode."]
    COUNTER = 2,
    #[doc = "4: Capture Mode."]
    CAPTURE = 4,
    #[doc = "5: Compare Mode."]
    COMPARE = 5,
    #[doc = "6: Gated Mode."]
    GATED = 6,
    #[doc = "7: Capture/Compare Mode."]
    CAPTURE_COMPARE = 7,
}
impl From<TMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TMODE_A {
    type Ux = u8;
}
impl TMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TMODE_A> {
        match self.bits {
            0 => Some(TMODE_A::ONE_SHOT),
            1 => Some(TMODE_A::CONTINUOUS),
            2 => Some(TMODE_A::COUNTER),
            4 => Some(TMODE_A::CAPTURE),
            5 => Some(TMODE_A::COMPARE),
            6 => Some(TMODE_A::GATED),
            7 => Some(TMODE_A::CAPTURE_COMPARE),
            _ => None,
        }
    }
    #[doc = "One Shot Mode."]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == TMODE_A::ONE_SHOT
    }
    #[doc = "Continuous Mode."]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == TMODE_A::CONTINUOUS
    }
    #[doc = "Counter Mode."]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        *self == TMODE_A::COUNTER
    }
    #[doc = "Capture Mode."]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == TMODE_A::CAPTURE
    }
    #[doc = "Compare Mode."]
    #[inline(always)]
    pub fn is_compare(&self) -> bool {
        *self == TMODE_A::COMPARE
    }
    #[doc = "Gated Mode."]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == TMODE_A::GATED
    }
    #[doc = "Capture/Compare Mode."]
    #[inline(always)]
    pub fn is_capture_compare(&self) -> bool {
        *self == TMODE_A::CAPTURE_COMPARE
    }
}
#[doc = "Field `TMODE` writer - Timer Mode."]
pub type TMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TMODE_A>;
impl<'a, REG> TMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One Shot Mode."]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut crate::W<REG> {
        self.variant(TMODE_A::ONE_SHOT)
    }
    #[doc = "Continuous Mode."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(TMODE_A::CONTINUOUS)
    }
    #[doc = "Counter Mode."]
    #[inline(always)]
    pub fn counter(self) -> &'a mut crate::W<REG> {
        self.variant(TMODE_A::COUNTER)
    }
    #[doc = "Capture Mode."]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(TMODE_A::CAPTURE)
    }
    #[doc = "Compare Mode."]
    #[inline(always)]
    pub fn compare(self) -> &'a mut crate::W<REG> {
        self.variant(TMODE_A::COMPARE)
    }
    #[doc = "Gated Mode."]
    #[inline(always)]
    pub fn gated(self) -> &'a mut crate::W<REG> {
        self.variant(TMODE_A::GATED)
    }
    #[doc = "Capture/Compare Mode."]
    #[inline(always)]
    pub fn capture_compare(self) -> &'a mut crate::W<REG> {
        self.variant(TMODE_A::CAPTURE_COMPARE)
    }
}
#[doc = "Field `PRES` reader - Prescaler. Set the Timer's prescaler value. The prescaler divides the PCLK input to the timer and sets the Timer's Count Clock, F_CNT_CLK = PCLK(HZ)/prescaler. The Timer's prescaler setting is a 4-bit value with pres3:pres\\[2:0\\]."]
pub type PRES_R = crate::FieldReader<PRES_A>;
#[doc = "Prescaler. Set the Timer's prescaler value. The prescaler divides the PCLK input to the timer and sets the Timer's Count Clock, F_CNT_CLK = PCLK(HZ)/prescaler. The Timer's prescaler setting is a 4-bit value with pres3:pres\\[2:0\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRES_A {
    #[doc = "0: Divide by 1."]
    DIV1 = 0,
    #[doc = "1: Divide by 2."]
    DIV2 = 1,
    #[doc = "2: Divide by 4."]
    DIV4 = 2,
    #[doc = "3: Divide by 8."]
    DIV8 = 3,
    #[doc = "4: Divide by 16."]
    DIV16 = 4,
    #[doc = "5: Divide by 32."]
    DIV32 = 5,
    #[doc = "6: Divide by 64."]
    DIV64 = 6,
    #[doc = "7: Divide by 128."]
    DIV128 = 7,
}
impl From<PRES_A> for u8 {
    #[inline(always)]
    fn from(variant: PRES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRES_A {
    type Ux = u8;
}
impl PRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRES_A {
        match self.bits {
            0 => PRES_A::DIV1,
            1 => PRES_A::DIV2,
            2 => PRES_A::DIV4,
            3 => PRES_A::DIV8,
            4 => PRES_A::DIV16,
            5 => PRES_A::DIV32,
            6 => PRES_A::DIV64,
            7 => PRES_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRES_A::DIV1
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRES_A::DIV2
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRES_A::DIV4
    }
    #[doc = "Divide by 8."]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRES_A::DIV8
    }
    #[doc = "Divide by 16."]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRES_A::DIV16
    }
    #[doc = "Divide by 32."]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRES_A::DIV32
    }
    #[doc = "Divide by 64."]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRES_A::DIV64
    }
    #[doc = "Divide by 128."]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRES_A::DIV128
    }
}
#[doc = "Field `PRES` writer - Prescaler. Set the Timer's prescaler value. The prescaler divides the PCLK input to the timer and sets the Timer's Count Clock, F_CNT_CLK = PCLK(HZ)/prescaler. The Timer's prescaler setting is a 4-bit value with pres3:pres\\[2:0\\]."]
pub type PRES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, PRES_A>;
impl<'a, REG> PRES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::DIV1)
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::DIV2)
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::DIV4)
    }
    #[doc = "Divide by 8."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::DIV8)
    }
    #[doc = "Divide by 16."]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::DIV16)
    }
    #[doc = "Divide by 32."]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::DIV32)
    }
    #[doc = "Divide by 64."]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::DIV64)
    }
    #[doc = "Divide by 128."]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::DIV128)
    }
}
#[doc = "Field `TPOL` reader - Timer input/output polarity bit."]
pub type TPOL_R = crate::BitReader<TPOL_A>;
#[doc = "Timer input/output polarity bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPOL_A {
    #[doc = "0: Active High."]
    ACTIVE_HI = 0,
    #[doc = "1: Active Low."]
    ACTIVE_LO = 1,
}
impl From<TPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl TPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TPOL_A {
        match self.bits {
            false => TPOL_A::ACTIVE_HI,
            true => TPOL_A::ACTIVE_LO,
        }
    }
    #[doc = "Active High."]
    #[inline(always)]
    pub fn is_active_hi(&self) -> bool {
        *self == TPOL_A::ACTIVE_HI
    }
    #[doc = "Active Low."]
    #[inline(always)]
    pub fn is_active_lo(&self) -> bool {
        *self == TPOL_A::ACTIVE_LO
    }
}
#[doc = "Field `TPOL` writer - Timer input/output polarity bit."]
pub type TPOL_W<'a, REG> = crate::BitWriter<'a, REG, TPOL_A>;
impl<'a, REG> TPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active High."]
    #[inline(always)]
    pub fn active_hi(self) -> &'a mut crate::W<REG> {
        self.variant(TPOL_A::ACTIVE_HI)
    }
    #[doc = "Active Low."]
    #[inline(always)]
    pub fn active_lo(self) -> &'a mut crate::W<REG> {
        self.variant(TPOL_A::ACTIVE_LO)
    }
}
#[doc = "Field `TEN` reader - Timer Enable."]
pub type TEN_R = crate::BitReader<TEN_A>;
#[doc = "Timer Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<TEN_A> for bool {
    #[inline(always)]
    fn from(variant: TEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEN_A {
        match self.bits {
            false => TEN_A::DIS,
            true => TEN_A::EN,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TEN_A::DIS
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TEN_A::EN
    }
}
#[doc = "Field `TEN` writer - Timer Enable."]
pub type TEN_W<'a, REG> = crate::BitWriter<'a, REG, TEN_A>;
impl<'a, REG> TEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TEN_A::EN)
    }
}
#[doc = "Field `PRES3` reader - MSB of prescaler value."]
pub type PRES3_R = crate::BitReader;
#[doc = "Field `PRES3` writer - MSB of prescaler value."]
pub type PRES3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Timer Mode."]
    #[inline(always)]
    pub fn tmode(&self) -> TMODE_R {
        TMODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Prescaler. Set the Timer's prescaler value. The prescaler divides the PCLK input to the timer and sets the Timer's Count Clock, F_CNT_CLK = PCLK(HZ)/prescaler. The Timer's prescaler setting is a 4-bit value with pres3:pres\\[2:0\\]."]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Timer input/output polarity bit."]
    #[inline(always)]
    pub fn tpol(&self) -> TPOL_R {
        TPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer Enable."]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MSB of prescaler value."]
    #[inline(always)]
    pub fn pres3(&self) -> PRES3_R {
        PRES3_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Timer Mode."]
    #[inline(always)]
    #[must_use]
    pub fn tmode(&mut self) -> TMODE_W<CTRL_SPEC> {
        TMODE_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Prescaler. Set the Timer's prescaler value. The prescaler divides the PCLK input to the timer and sets the Timer's Count Clock, F_CNT_CLK = PCLK(HZ)/prescaler. The Timer's prescaler setting is a 4-bit value with pres3:pres\\[2:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn pres(&mut self) -> PRES_W<CTRL_SPEC> {
        PRES_W::new(self, 3)
    }
    #[doc = "Bit 6 - Timer input/output polarity bit."]
    #[inline(always)]
    #[must_use]
    pub fn tpol(&mut self) -> TPOL_W<CTRL_SPEC> {
        TPOL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Timer Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TEN_W<CTRL_SPEC> {
        TEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - MSB of prescaler value."]
    #[inline(always)]
    #[must_use]
    pub fn pres3(&mut self) -> PRES3_W<CTRL_SPEC> {
        PRES3_W::new(self, 8)
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
#[doc = "Timer Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
