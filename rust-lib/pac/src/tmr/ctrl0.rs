#[doc = "Register `CTRL0` reader"]
pub type R = crate::R<CTRL0_SPEC>;
#[doc = "Register `CTRL0` writer"]
pub type W = crate::W<CTRL0_SPEC>;
#[doc = "Field `MODE_A` reader - Mode Select for Timer A"]
pub type MODE_A_R = crate::FieldReader<MODE_A_A>;
#[doc = "Mode Select for Timer A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A_A {
    #[doc = "0: One-Shot Mode"]
    ONE_SHOT = 0,
    #[doc = "1: Continuous Mode"]
    CONTINUOUS = 1,
    #[doc = "2: Counter Mode"]
    COUNTER = 2,
    #[doc = "3: PWM Mode"]
    PWM = 3,
    #[doc = "4: Capture Mode"]
    CAPTURE = 4,
    #[doc = "5: Compare Mode"]
    COMPARE = 5,
    #[doc = "6: Gated Mode"]
    GATED = 6,
    #[doc = "7: Capture/Compare Mode"]
    CAPCOMP = 7,
    #[doc = "8: Dual Edge Capture Mode"]
    DUAL_EDGE = 8,
    #[doc = "14: Inactive Gated Mode"]
    IGATED = 14,
}
impl From<MODE_A_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A_A {
    type Ux = u8;
}
impl MODE_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE_A_A> {
        match self.bits {
            0 => Some(MODE_A_A::ONE_SHOT),
            1 => Some(MODE_A_A::CONTINUOUS),
            2 => Some(MODE_A_A::COUNTER),
            3 => Some(MODE_A_A::PWM),
            4 => Some(MODE_A_A::CAPTURE),
            5 => Some(MODE_A_A::COMPARE),
            6 => Some(MODE_A_A::GATED),
            7 => Some(MODE_A_A::CAPCOMP),
            8 => Some(MODE_A_A::DUAL_EDGE),
            14 => Some(MODE_A_A::IGATED),
            _ => None,
        }
    }
    #[doc = "One-Shot Mode"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == MODE_A_A::ONE_SHOT
    }
    #[doc = "Continuous Mode"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == MODE_A_A::CONTINUOUS
    }
    #[doc = "Counter Mode"]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        *self == MODE_A_A::COUNTER
    }
    #[doc = "PWM Mode"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == MODE_A_A::PWM
    }
    #[doc = "Capture Mode"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == MODE_A_A::CAPTURE
    }
    #[doc = "Compare Mode"]
    #[inline(always)]
    pub fn is_compare(&self) -> bool {
        *self == MODE_A_A::COMPARE
    }
    #[doc = "Gated Mode"]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == MODE_A_A::GATED
    }
    #[doc = "Capture/Compare Mode"]
    #[inline(always)]
    pub fn is_capcomp(&self) -> bool {
        *self == MODE_A_A::CAPCOMP
    }
    #[doc = "Dual Edge Capture Mode"]
    #[inline(always)]
    pub fn is_dual_edge(&self) -> bool {
        *self == MODE_A_A::DUAL_EDGE
    }
    #[doc = "Inactive Gated Mode"]
    #[inline(always)]
    pub fn is_igated(&self) -> bool {
        *self == MODE_A_A::IGATED
    }
}
#[doc = "Field `MODE_A` writer - Mode Select for Timer A"]
pub type MODE_A_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MODE_A_A>;
impl<'a, REG> MODE_A_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One-Shot Mode"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A_A::ONE_SHOT)
    }
    #[doc = "Continuous Mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A_A::CONTINUOUS)
    }
    #[doc = "Counter Mode"]
    #[inline(always)]
    pub fn counter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A_A::COUNTER)
    }
    #[doc = "PWM Mode"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A_A::PWM)
    }
    #[doc = "Capture Mode"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A_A::CAPTURE)
    }
    #[doc = "Compare Mode"]
    #[inline(always)]
    pub fn compare(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A_A::COMPARE)
    }
    #[doc = "Gated Mode"]
    #[inline(always)]
    pub fn gated(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A_A::GATED)
    }
    #[doc = "Capture/Compare Mode"]
    #[inline(always)]
    pub fn capcomp(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A_A::CAPCOMP)
    }
    #[doc = "Dual Edge Capture Mode"]
    #[inline(always)]
    pub fn dual_edge(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A_A::DUAL_EDGE)
    }
    #[doc = "Inactive Gated Mode"]
    #[inline(always)]
    pub fn igated(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A_A::IGATED)
    }
}
#[doc = "Field `CLKDIV_A` reader - Clock Divider Select for Timer A"]
pub type CLKDIV_A_R = crate::FieldReader<CLKDIV_A_A>;
#[doc = "Clock Divider Select for Timer A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKDIV_A_A {
    #[doc = "0: Prescaler Divide-By-1"]
    DIV_BY_1 = 0,
    #[doc = "1: Prescaler Divide-By-2"]
    DIV_BY_2 = 1,
    #[doc = "2: Prescaler Divide-By-4"]
    DIV_BY_4 = 2,
    #[doc = "3: Prescaler Divide-By-8"]
    DIV_BY_8 = 3,
    #[doc = "4: Prescaler Divide-By-16"]
    DIV_BY_16 = 4,
    #[doc = "5: Prescaler Divide-By-32"]
    DIV_BY_32 = 5,
    #[doc = "6: Prescaler Divide-By-64"]
    DIV_BY_64 = 6,
    #[doc = "7: Prescaler Divide-By-128"]
    DIV_BY_128 = 7,
    #[doc = "8: Prescaler Divide-By-256"]
    DIV_BY_256 = 8,
    #[doc = "9: Prescaler Divide-By-512"]
    DIV_BY_512 = 9,
    #[doc = "10: Prescaler Divide-By-1024"]
    DIV_BY_1024 = 10,
    #[doc = "11: Prescaler Divide-By-2048"]
    DIV_BY_2048 = 11,
    #[doc = "12: TBD"]
    DIV_BY_4096 = 12,
}
impl From<CLKDIV_A_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKDIV_A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKDIV_A_A {
    type Ux = u8;
}
impl CLKDIV_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKDIV_A_A> {
        match self.bits {
            0 => Some(CLKDIV_A_A::DIV_BY_1),
            1 => Some(CLKDIV_A_A::DIV_BY_2),
            2 => Some(CLKDIV_A_A::DIV_BY_4),
            3 => Some(CLKDIV_A_A::DIV_BY_8),
            4 => Some(CLKDIV_A_A::DIV_BY_16),
            5 => Some(CLKDIV_A_A::DIV_BY_32),
            6 => Some(CLKDIV_A_A::DIV_BY_64),
            7 => Some(CLKDIV_A_A::DIV_BY_128),
            8 => Some(CLKDIV_A_A::DIV_BY_256),
            9 => Some(CLKDIV_A_A::DIV_BY_512),
            10 => Some(CLKDIV_A_A::DIV_BY_1024),
            11 => Some(CLKDIV_A_A::DIV_BY_2048),
            12 => Some(CLKDIV_A_A::DIV_BY_4096),
            _ => None,
        }
    }
    #[doc = "Prescaler Divide-By-1"]
    #[inline(always)]
    pub fn is_div_by_1(&self) -> bool {
        *self == CLKDIV_A_A::DIV_BY_1
    }
    #[doc = "Prescaler Divide-By-2"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == CLKDIV_A_A::DIV_BY_2
    }
    #[doc = "Prescaler Divide-By-4"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == CLKDIV_A_A::DIV_BY_4
    }
    #[doc = "Prescaler Divide-By-8"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == CLKDIV_A_A::DIV_BY_8
    }
    #[doc = "Prescaler Divide-By-16"]
    #[inline(always)]
    pub fn is_div_by_16(&self) -> bool {
        *self == CLKDIV_A_A::DIV_BY_16
    }
    #[doc = "Prescaler Divide-By-32"]
    #[inline(always)]
    pub fn is_div_by_32(&self) -> bool {
        *self == CLKDIV_A_A::DIV_BY_32
    }
    #[doc = "Prescaler Divide-By-64"]
    #[inline(always)]
    pub fn is_div_by_64(&self) -> bool {
        *self == CLKDIV_A_A::DIV_BY_64
    }
    #[doc = "Prescaler Divide-By-128"]
    #[inline(always)]
    pub fn is_div_by_128(&self) -> bool {
        *self == CLKDIV_A_A::DIV_BY_128
    }
    #[doc = "Prescaler Divide-By-256"]
    #[inline(always)]
    pub fn is_div_by_256(&self) -> bool {
        *self == CLKDIV_A_A::DIV_BY_256
    }
    #[doc = "Prescaler Divide-By-512"]
    #[inline(always)]
    pub fn is_div_by_512(&self) -> bool {
        *self == CLKDIV_A_A::DIV_BY_512
    }
    #[doc = "Prescaler Divide-By-1024"]
    #[inline(always)]
    pub fn is_div_by_1024(&self) -> bool {
        *self == CLKDIV_A_A::DIV_BY_1024
    }
    #[doc = "Prescaler Divide-By-2048"]
    #[inline(always)]
    pub fn is_div_by_2048(&self) -> bool {
        *self == CLKDIV_A_A::DIV_BY_2048
    }
    #[doc = "TBD"]
    #[inline(always)]
    pub fn is_div_by_4096(&self) -> bool {
        *self == CLKDIV_A_A::DIV_BY_4096
    }
}
#[doc = "Field `CLKDIV_A` writer - Clock Divider Select for Timer A"]
pub type CLKDIV_A_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CLKDIV_A_A>;
impl<'a, REG> CLKDIV_A_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaler Divide-By-1"]
    #[inline(always)]
    pub fn div_by_1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A_A::DIV_BY_1)
    }
    #[doc = "Prescaler Divide-By-2"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A_A::DIV_BY_2)
    }
    #[doc = "Prescaler Divide-By-4"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A_A::DIV_BY_4)
    }
    #[doc = "Prescaler Divide-By-8"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A_A::DIV_BY_8)
    }
    #[doc = "Prescaler Divide-By-16"]
    #[inline(always)]
    pub fn div_by_16(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A_A::DIV_BY_16)
    }
    #[doc = "Prescaler Divide-By-32"]
    #[inline(always)]
    pub fn div_by_32(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A_A::DIV_BY_32)
    }
    #[doc = "Prescaler Divide-By-64"]
    #[inline(always)]
    pub fn div_by_64(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A_A::DIV_BY_64)
    }
    #[doc = "Prescaler Divide-By-128"]
    #[inline(always)]
    pub fn div_by_128(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A_A::DIV_BY_128)
    }
    #[doc = "Prescaler Divide-By-256"]
    #[inline(always)]
    pub fn div_by_256(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A_A::DIV_BY_256)
    }
    #[doc = "Prescaler Divide-By-512"]
    #[inline(always)]
    pub fn div_by_512(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A_A::DIV_BY_512)
    }
    #[doc = "Prescaler Divide-By-1024"]
    #[inline(always)]
    pub fn div_by_1024(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A_A::DIV_BY_1024)
    }
    #[doc = "Prescaler Divide-By-2048"]
    #[inline(always)]
    pub fn div_by_2048(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A_A::DIV_BY_2048)
    }
    #[doc = "TBD"]
    #[inline(always)]
    pub fn div_by_4096(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A_A::DIV_BY_4096)
    }
}
#[doc = "Field `POL_A` reader - Timer Polarity for Timer A"]
pub type POL_A_R = crate::BitReader;
#[doc = "Field `POL_A` writer - Timer Polarity for Timer A"]
pub type POL_A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSYNC_A` reader - PWM Synchronization Mode for Timer A"]
pub type PWMSYNC_A_R = crate::BitReader;
#[doc = "Field `PWMSYNC_A` writer - PWM Synchronization Mode for Timer A"]
pub type PWMSYNC_A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOLHPOL_A` reader - PWM Phase A (Non-Overlapping High) Polarity for Timer A"]
pub type NOLHPOL_A_R = crate::BitReader;
#[doc = "Field `NOLHPOL_A` writer - PWM Phase A (Non-Overlapping High) Polarity for Timer A"]
pub type NOLHPOL_A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOLLPOL_A` reader - PWM Phase A-Prime (Non-Overlapping Low) Polarity for Timer A"]
pub type NOLLPOL_A_R = crate::BitReader;
#[doc = "Field `NOLLPOL_A` writer - PWM Phase A-Prime (Non-Overlapping Low) Polarity for Timer A"]
pub type NOLLPOL_A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMCKBD_A` reader - PWM Phase A-Prime Output Disable for Timer A"]
pub type PWMCKBD_A_R = crate::BitReader;
#[doc = "Field `PWMCKBD_A` writer - PWM Phase A-Prime Output Disable for Timer A"]
pub type PWMCKBD_A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_A` reader - Resets all flip flops in the CLK_TMR domain for Timer A. Self-clears."]
pub type RST_A_R = crate::BitReader;
#[doc = "Field `RST_A` writer - Resets all flip flops in the CLK_TMR domain for Timer A. Self-clears."]
pub type RST_A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKEN_A` reader - Write 1 to Enable CLK_TMR for Timer A"]
pub type CLKEN_A_R = crate::BitReader;
#[doc = "Field `CLKEN_A` writer - Write 1 to Enable CLK_TMR for Timer A"]
pub type CLKEN_A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_A` reader - Enable for Timer A"]
pub type EN_A_R = crate::BitReader;
#[doc = "Field `EN_A` writer - Enable for Timer A"]
pub type EN_A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE_B` reader - Mode Select for Timer B"]
pub type MODE_B_R = crate::FieldReader<MODE_B_A>;
#[doc = "Mode Select for Timer B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_B_A {
    #[doc = "0: One-Shot Mode"]
    ONE_SHOT = 0,
    #[doc = "1: Continuous Mode"]
    CONTINUOUS = 1,
    #[doc = "2: Counter Mode"]
    COUNTER = 2,
    #[doc = "3: PWM Mode"]
    PWM = 3,
    #[doc = "4: Capture Mode"]
    CAPTURE = 4,
    #[doc = "5: Compare Mode"]
    COMPARE = 5,
    #[doc = "6: Gated Mode"]
    GATED = 6,
    #[doc = "7: Capture/Compare Mode"]
    CAPCOMP = 7,
    #[doc = "8: Dual Edge Capture Mode"]
    DUAL_EDGE = 8,
    #[doc = "14: Inactive Gated Mode"]
    IGATED = 14,
}
impl From<MODE_B_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_B_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_B_A {
    type Ux = u8;
}
impl MODE_B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE_B_A> {
        match self.bits {
            0 => Some(MODE_B_A::ONE_SHOT),
            1 => Some(MODE_B_A::CONTINUOUS),
            2 => Some(MODE_B_A::COUNTER),
            3 => Some(MODE_B_A::PWM),
            4 => Some(MODE_B_A::CAPTURE),
            5 => Some(MODE_B_A::COMPARE),
            6 => Some(MODE_B_A::GATED),
            7 => Some(MODE_B_A::CAPCOMP),
            8 => Some(MODE_B_A::DUAL_EDGE),
            14 => Some(MODE_B_A::IGATED),
            _ => None,
        }
    }
    #[doc = "One-Shot Mode"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == MODE_B_A::ONE_SHOT
    }
    #[doc = "Continuous Mode"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == MODE_B_A::CONTINUOUS
    }
    #[doc = "Counter Mode"]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        *self == MODE_B_A::COUNTER
    }
    #[doc = "PWM Mode"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == MODE_B_A::PWM
    }
    #[doc = "Capture Mode"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == MODE_B_A::CAPTURE
    }
    #[doc = "Compare Mode"]
    #[inline(always)]
    pub fn is_compare(&self) -> bool {
        *self == MODE_B_A::COMPARE
    }
    #[doc = "Gated Mode"]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == MODE_B_A::GATED
    }
    #[doc = "Capture/Compare Mode"]
    #[inline(always)]
    pub fn is_capcomp(&self) -> bool {
        *self == MODE_B_A::CAPCOMP
    }
    #[doc = "Dual Edge Capture Mode"]
    #[inline(always)]
    pub fn is_dual_edge(&self) -> bool {
        *self == MODE_B_A::DUAL_EDGE
    }
    #[doc = "Inactive Gated Mode"]
    #[inline(always)]
    pub fn is_igated(&self) -> bool {
        *self == MODE_B_A::IGATED
    }
}
#[doc = "Field `MODE_B` writer - Mode Select for Timer B"]
pub type MODE_B_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MODE_B_A>;
impl<'a, REG> MODE_B_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One-Shot Mode"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_B_A::ONE_SHOT)
    }
    #[doc = "Continuous Mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_B_A::CONTINUOUS)
    }
    #[doc = "Counter Mode"]
    #[inline(always)]
    pub fn counter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_B_A::COUNTER)
    }
    #[doc = "PWM Mode"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_B_A::PWM)
    }
    #[doc = "Capture Mode"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_B_A::CAPTURE)
    }
    #[doc = "Compare Mode"]
    #[inline(always)]
    pub fn compare(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_B_A::COMPARE)
    }
    #[doc = "Gated Mode"]
    #[inline(always)]
    pub fn gated(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_B_A::GATED)
    }
    #[doc = "Capture/Compare Mode"]
    #[inline(always)]
    pub fn capcomp(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_B_A::CAPCOMP)
    }
    #[doc = "Dual Edge Capture Mode"]
    #[inline(always)]
    pub fn dual_edge(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_B_A::DUAL_EDGE)
    }
    #[doc = "Inactive Gated Mode"]
    #[inline(always)]
    pub fn igated(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_B_A::IGATED)
    }
}
#[doc = "Field `CLKDIV_B` reader - Clock Divider Select for Timer B"]
pub type CLKDIV_B_R = crate::FieldReader<CLKDIV_B_A>;
#[doc = "Clock Divider Select for Timer B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKDIV_B_A {
    #[doc = "0: Prescaler Divide-By-1"]
    DIV_BY_1 = 0,
    #[doc = "1: Prescaler Divide-By-2"]
    DIV_BY_2 = 1,
    #[doc = "2: Prescaler Divide-By-4"]
    DIV_BY_4 = 2,
    #[doc = "3: Prescaler Divide-By-8"]
    DIV_BY_8 = 3,
    #[doc = "4: Prescaler Divide-By-16"]
    DIV_BY_16 = 4,
    #[doc = "5: Prescaler Divide-By-32"]
    DIV_BY_32 = 5,
    #[doc = "6: Prescaler Divide-By-64"]
    DIV_BY_64 = 6,
    #[doc = "7: Prescaler Divide-By-128"]
    DIV_BY_128 = 7,
    #[doc = "8: Prescaler Divide-By-256"]
    DIV_BY_256 = 8,
    #[doc = "9: Prescaler Divide-By-512"]
    DIV_BY_512 = 9,
    #[doc = "10: Prescaler Divide-By-1024"]
    DIV_BY_1024 = 10,
    #[doc = "11: Prescaler Divide-By-2048"]
    DIV_BY_2048 = 11,
    #[doc = "12: TBD"]
    DIV_BY_4096 = 12,
}
impl From<CLKDIV_B_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKDIV_B_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKDIV_B_A {
    type Ux = u8;
}
impl CLKDIV_B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKDIV_B_A> {
        match self.bits {
            0 => Some(CLKDIV_B_A::DIV_BY_1),
            1 => Some(CLKDIV_B_A::DIV_BY_2),
            2 => Some(CLKDIV_B_A::DIV_BY_4),
            3 => Some(CLKDIV_B_A::DIV_BY_8),
            4 => Some(CLKDIV_B_A::DIV_BY_16),
            5 => Some(CLKDIV_B_A::DIV_BY_32),
            6 => Some(CLKDIV_B_A::DIV_BY_64),
            7 => Some(CLKDIV_B_A::DIV_BY_128),
            8 => Some(CLKDIV_B_A::DIV_BY_256),
            9 => Some(CLKDIV_B_A::DIV_BY_512),
            10 => Some(CLKDIV_B_A::DIV_BY_1024),
            11 => Some(CLKDIV_B_A::DIV_BY_2048),
            12 => Some(CLKDIV_B_A::DIV_BY_4096),
            _ => None,
        }
    }
    #[doc = "Prescaler Divide-By-1"]
    #[inline(always)]
    pub fn is_div_by_1(&self) -> bool {
        *self == CLKDIV_B_A::DIV_BY_1
    }
    #[doc = "Prescaler Divide-By-2"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == CLKDIV_B_A::DIV_BY_2
    }
    #[doc = "Prescaler Divide-By-4"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == CLKDIV_B_A::DIV_BY_4
    }
    #[doc = "Prescaler Divide-By-8"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == CLKDIV_B_A::DIV_BY_8
    }
    #[doc = "Prescaler Divide-By-16"]
    #[inline(always)]
    pub fn is_div_by_16(&self) -> bool {
        *self == CLKDIV_B_A::DIV_BY_16
    }
    #[doc = "Prescaler Divide-By-32"]
    #[inline(always)]
    pub fn is_div_by_32(&self) -> bool {
        *self == CLKDIV_B_A::DIV_BY_32
    }
    #[doc = "Prescaler Divide-By-64"]
    #[inline(always)]
    pub fn is_div_by_64(&self) -> bool {
        *self == CLKDIV_B_A::DIV_BY_64
    }
    #[doc = "Prescaler Divide-By-128"]
    #[inline(always)]
    pub fn is_div_by_128(&self) -> bool {
        *self == CLKDIV_B_A::DIV_BY_128
    }
    #[doc = "Prescaler Divide-By-256"]
    #[inline(always)]
    pub fn is_div_by_256(&self) -> bool {
        *self == CLKDIV_B_A::DIV_BY_256
    }
    #[doc = "Prescaler Divide-By-512"]
    #[inline(always)]
    pub fn is_div_by_512(&self) -> bool {
        *self == CLKDIV_B_A::DIV_BY_512
    }
    #[doc = "Prescaler Divide-By-1024"]
    #[inline(always)]
    pub fn is_div_by_1024(&self) -> bool {
        *self == CLKDIV_B_A::DIV_BY_1024
    }
    #[doc = "Prescaler Divide-By-2048"]
    #[inline(always)]
    pub fn is_div_by_2048(&self) -> bool {
        *self == CLKDIV_B_A::DIV_BY_2048
    }
    #[doc = "TBD"]
    #[inline(always)]
    pub fn is_div_by_4096(&self) -> bool {
        *self == CLKDIV_B_A::DIV_BY_4096
    }
}
#[doc = "Field `CLKDIV_B` writer - Clock Divider Select for Timer B"]
pub type CLKDIV_B_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CLKDIV_B_A>;
impl<'a, REG> CLKDIV_B_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaler Divide-By-1"]
    #[inline(always)]
    pub fn div_by_1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_B_A::DIV_BY_1)
    }
    #[doc = "Prescaler Divide-By-2"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_B_A::DIV_BY_2)
    }
    #[doc = "Prescaler Divide-By-4"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_B_A::DIV_BY_4)
    }
    #[doc = "Prescaler Divide-By-8"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_B_A::DIV_BY_8)
    }
    #[doc = "Prescaler Divide-By-16"]
    #[inline(always)]
    pub fn div_by_16(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_B_A::DIV_BY_16)
    }
    #[doc = "Prescaler Divide-By-32"]
    #[inline(always)]
    pub fn div_by_32(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_B_A::DIV_BY_32)
    }
    #[doc = "Prescaler Divide-By-64"]
    #[inline(always)]
    pub fn div_by_64(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_B_A::DIV_BY_64)
    }
    #[doc = "Prescaler Divide-By-128"]
    #[inline(always)]
    pub fn div_by_128(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_B_A::DIV_BY_128)
    }
    #[doc = "Prescaler Divide-By-256"]
    #[inline(always)]
    pub fn div_by_256(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_B_A::DIV_BY_256)
    }
    #[doc = "Prescaler Divide-By-512"]
    #[inline(always)]
    pub fn div_by_512(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_B_A::DIV_BY_512)
    }
    #[doc = "Prescaler Divide-By-1024"]
    #[inline(always)]
    pub fn div_by_1024(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_B_A::DIV_BY_1024)
    }
    #[doc = "Prescaler Divide-By-2048"]
    #[inline(always)]
    pub fn div_by_2048(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_B_A::DIV_BY_2048)
    }
    #[doc = "TBD"]
    #[inline(always)]
    pub fn div_by_4096(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_B_A::DIV_BY_4096)
    }
}
#[doc = "Field `POL_B` reader - Timer Polarity for Timer B"]
pub type POL_B_R = crate::BitReader;
#[doc = "Field `POL_B` writer - Timer Polarity for Timer B"]
pub type POL_B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSYNC_B` reader - PWM Synchronization Mode for Timer B"]
pub type PWMSYNC_B_R = crate::BitReader;
#[doc = "Field `PWMSYNC_B` writer - PWM Synchronization Mode for Timer B"]
pub type PWMSYNC_B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOLHPOL_B` reader - PWM Phase A (Non-Overlapping High) Polarity for Timer B"]
pub type NOLHPOL_B_R = crate::BitReader;
#[doc = "Field `NOLHPOL_B` writer - PWM Phase A (Non-Overlapping High) Polarity for Timer B"]
pub type NOLHPOL_B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOLLPOL_B` reader - PWM Phase A-Prime (Non-Overlapping Low) Polarity for Timer B"]
pub type NOLLPOL_B_R = crate::BitReader;
#[doc = "Field `NOLLPOL_B` writer - PWM Phase A-Prime (Non-Overlapping Low) Polarity for Timer B"]
pub type NOLLPOL_B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMCKBD_B` reader - PWM Phase A-Prime Output Disable for Timer B"]
pub type PWMCKBD_B_R = crate::BitReader;
#[doc = "Field `PWMCKBD_B` writer - PWM Phase A-Prime Output Disable for Timer B"]
pub type PWMCKBD_B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_B` reader - Resets all flip flops in the CLK_TMR domain for Timer B. Self-clears."]
pub type RST_B_R = crate::BitReader;
#[doc = "Field `RST_B` writer - Resets all flip flops in the CLK_TMR domain for Timer B. Self-clears."]
pub type RST_B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKEN_B` reader - Write 1 to Enable CLK_TMR for Timer B"]
pub type CLKEN_B_R = crate::BitReader;
#[doc = "Field `CLKEN_B` writer - Write 1 to Enable CLK_TMR for Timer B"]
pub type CLKEN_B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_B` reader - Enable for Timer B"]
pub type EN_B_R = crate::BitReader;
#[doc = "Field `EN_B` writer - Enable for Timer B"]
pub type EN_B_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Mode Select for Timer A"]
    #[inline(always)]
    pub fn mode_a(&self) -> MODE_A_R {
        MODE_A_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Clock Divider Select for Timer A"]
    #[inline(always)]
    pub fn clkdiv_a(&self) -> CLKDIV_A_R {
        CLKDIV_A_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Timer Polarity for Timer A"]
    #[inline(always)]
    pub fn pol_a(&self) -> POL_A_R {
        POL_A_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PWM Synchronization Mode for Timer A"]
    #[inline(always)]
    pub fn pwmsync_a(&self) -> PWMSYNC_A_R {
        PWMSYNC_A_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PWM Phase A (Non-Overlapping High) Polarity for Timer A"]
    #[inline(always)]
    pub fn nolhpol_a(&self) -> NOLHPOL_A_R {
        NOLHPOL_A_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PWM Phase A-Prime (Non-Overlapping Low) Polarity for Timer A"]
    #[inline(always)]
    pub fn nollpol_a(&self) -> NOLLPOL_A_R {
        NOLLPOL_A_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PWM Phase A-Prime Output Disable for Timer A"]
    #[inline(always)]
    pub fn pwmckbd_a(&self) -> PWMCKBD_A_R {
        PWMCKBD_A_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Resets all flip flops in the CLK_TMR domain for Timer A. Self-clears."]
    #[inline(always)]
    pub fn rst_a(&self) -> RST_A_R {
        RST_A_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write 1 to Enable CLK_TMR for Timer A"]
    #[inline(always)]
    pub fn clken_a(&self) -> CLKEN_A_R {
        CLKEN_A_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable for Timer A"]
    #[inline(always)]
    pub fn en_a(&self) -> EN_A_R {
        EN_A_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Mode Select for Timer B"]
    #[inline(always)]
    pub fn mode_b(&self) -> MODE_B_R {
        MODE_B_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Clock Divider Select for Timer B"]
    #[inline(always)]
    pub fn clkdiv_b(&self) -> CLKDIV_B_R {
        CLKDIV_B_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Timer Polarity for Timer B"]
    #[inline(always)]
    pub fn pol_b(&self) -> POL_B_R {
        POL_B_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PWM Synchronization Mode for Timer B"]
    #[inline(always)]
    pub fn pwmsync_b(&self) -> PWMSYNC_B_R {
        PWMSYNC_B_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PWM Phase A (Non-Overlapping High) Polarity for Timer B"]
    #[inline(always)]
    pub fn nolhpol_b(&self) -> NOLHPOL_B_R {
        NOLHPOL_B_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PWM Phase A-Prime (Non-Overlapping Low) Polarity for Timer B"]
    #[inline(always)]
    pub fn nollpol_b(&self) -> NOLLPOL_B_R {
        NOLLPOL_B_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PWM Phase A-Prime Output Disable for Timer B"]
    #[inline(always)]
    pub fn pwmckbd_b(&self) -> PWMCKBD_B_R {
        PWMCKBD_B_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Resets all flip flops in the CLK_TMR domain for Timer B. Self-clears."]
    #[inline(always)]
    pub fn rst_b(&self) -> RST_B_R {
        RST_B_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Write 1 to Enable CLK_TMR for Timer B"]
    #[inline(always)]
    pub fn clken_b(&self) -> CLKEN_B_R {
        CLKEN_B_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable for Timer B"]
    #[inline(always)]
    pub fn en_b(&self) -> EN_B_R {
        EN_B_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Mode Select for Timer A"]
    #[inline(always)]
    #[must_use]
    pub fn mode_a(&mut self) -> MODE_A_W<CTRL0_SPEC> {
        MODE_A_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Clock Divider Select for Timer A"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv_a(&mut self) -> CLKDIV_A_W<CTRL0_SPEC> {
        CLKDIV_A_W::new(self, 4)
    }
    #[doc = "Bit 8 - Timer Polarity for Timer A"]
    #[inline(always)]
    #[must_use]
    pub fn pol_a(&mut self) -> POL_A_W<CTRL0_SPEC> {
        POL_A_W::new(self, 8)
    }
    #[doc = "Bit 9 - PWM Synchronization Mode for Timer A"]
    #[inline(always)]
    #[must_use]
    pub fn pwmsync_a(&mut self) -> PWMSYNC_A_W<CTRL0_SPEC> {
        PWMSYNC_A_W::new(self, 9)
    }
    #[doc = "Bit 10 - PWM Phase A (Non-Overlapping High) Polarity for Timer A"]
    #[inline(always)]
    #[must_use]
    pub fn nolhpol_a(&mut self) -> NOLHPOL_A_W<CTRL0_SPEC> {
        NOLHPOL_A_W::new(self, 10)
    }
    #[doc = "Bit 11 - PWM Phase A-Prime (Non-Overlapping Low) Polarity for Timer A"]
    #[inline(always)]
    #[must_use]
    pub fn nollpol_a(&mut self) -> NOLLPOL_A_W<CTRL0_SPEC> {
        NOLLPOL_A_W::new(self, 11)
    }
    #[doc = "Bit 12 - PWM Phase A-Prime Output Disable for Timer A"]
    #[inline(always)]
    #[must_use]
    pub fn pwmckbd_a(&mut self) -> PWMCKBD_A_W<CTRL0_SPEC> {
        PWMCKBD_A_W::new(self, 12)
    }
    #[doc = "Bit 13 - Resets all flip flops in the CLK_TMR domain for Timer A. Self-clears."]
    #[inline(always)]
    #[must_use]
    pub fn rst_a(&mut self) -> RST_A_W<CTRL0_SPEC> {
        RST_A_W::new(self, 13)
    }
    #[doc = "Bit 14 - Write 1 to Enable CLK_TMR for Timer A"]
    #[inline(always)]
    #[must_use]
    pub fn clken_a(&mut self) -> CLKEN_A_W<CTRL0_SPEC> {
        CLKEN_A_W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable for Timer A"]
    #[inline(always)]
    #[must_use]
    pub fn en_a(&mut self) -> EN_A_W<CTRL0_SPEC> {
        EN_A_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Mode Select for Timer B"]
    #[inline(always)]
    #[must_use]
    pub fn mode_b(&mut self) -> MODE_B_W<CTRL0_SPEC> {
        MODE_B_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Clock Divider Select for Timer B"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv_b(&mut self) -> CLKDIV_B_W<CTRL0_SPEC> {
        CLKDIV_B_W::new(self, 20)
    }
    #[doc = "Bit 24 - Timer Polarity for Timer B"]
    #[inline(always)]
    #[must_use]
    pub fn pol_b(&mut self) -> POL_B_W<CTRL0_SPEC> {
        POL_B_W::new(self, 24)
    }
    #[doc = "Bit 25 - PWM Synchronization Mode for Timer B"]
    #[inline(always)]
    #[must_use]
    pub fn pwmsync_b(&mut self) -> PWMSYNC_B_W<CTRL0_SPEC> {
        PWMSYNC_B_W::new(self, 25)
    }
    #[doc = "Bit 26 - PWM Phase A (Non-Overlapping High) Polarity for Timer B"]
    #[inline(always)]
    #[must_use]
    pub fn nolhpol_b(&mut self) -> NOLHPOL_B_W<CTRL0_SPEC> {
        NOLHPOL_B_W::new(self, 26)
    }
    #[doc = "Bit 27 - PWM Phase A-Prime (Non-Overlapping Low) Polarity for Timer B"]
    #[inline(always)]
    #[must_use]
    pub fn nollpol_b(&mut self) -> NOLLPOL_B_W<CTRL0_SPEC> {
        NOLLPOL_B_W::new(self, 27)
    }
    #[doc = "Bit 28 - PWM Phase A-Prime Output Disable for Timer B"]
    #[inline(always)]
    #[must_use]
    pub fn pwmckbd_b(&mut self) -> PWMCKBD_B_W<CTRL0_SPEC> {
        PWMCKBD_B_W::new(self, 28)
    }
    #[doc = "Bit 29 - Resets all flip flops in the CLK_TMR domain for Timer B. Self-clears."]
    #[inline(always)]
    #[must_use]
    pub fn rst_b(&mut self) -> RST_B_W<CTRL0_SPEC> {
        RST_B_W::new(self, 29)
    }
    #[doc = "Bit 30 - Write 1 to Enable CLK_TMR for Timer B"]
    #[inline(always)]
    #[must_use]
    pub fn clken_b(&mut self) -> CLKEN_B_W<CTRL0_SPEC> {
        CLKEN_B_W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable for Timer B"]
    #[inline(always)]
    #[must_use]
    pub fn en_b(&mut self) -> EN_B_W<CTRL0_SPEC> {
        EN_B_W::new(self, 31)
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
#[doc = "Timer Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL0_SPEC;
impl crate::RegisterSpec for CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl0::R`](R) reader structure"]
impl crate::Readable for CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl0::W`](W) writer structure"]
impl crate::Writable for CTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL0 to value 0"]
impl crate::Resettable for CTRL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
