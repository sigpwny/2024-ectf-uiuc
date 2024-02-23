#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `EN` reader - Real Time Clock Enable. This bit enables the Real Time Clock. This bit can only be written when WE=1 and BUSY =0. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "Real Time Clock Enable. This bit enables the Real Time Clock. This bit can only be written when WE=1 and BUSY =0. Change to this bit is effective only after BUSY is cleared from 1 to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DIS,
            true => EN_A::EN,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN_A::DIS
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN_A::EN
    }
}
#[doc = "Field `EN` writer - Real Time Clock Enable. This bit enables the Real Time Clock. This bit can only be written when WE=1 and BUSY =0. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN_A>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::EN)
    }
}
#[doc = "Field `TOD_ALARM_IE` reader - Alarm Time-of-Day Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
pub type TOD_ALARM_IE_R = crate::BitReader<TOD_ALARM_IE_A>;
#[doc = "Alarm Time-of-Day Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOD_ALARM_IE_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<TOD_ALARM_IE_A> for bool {
    #[inline(always)]
    fn from(variant: TOD_ALARM_IE_A) -> Self {
        variant as u8 != 0
    }
}
impl TOD_ALARM_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TOD_ALARM_IE_A {
        match self.bits {
            false => TOD_ALARM_IE_A::DIS,
            true => TOD_ALARM_IE_A::EN,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TOD_ALARM_IE_A::DIS
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TOD_ALARM_IE_A::EN
    }
}
#[doc = "Field `TOD_ALARM_IE` writer - Alarm Time-of-Day Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
pub type TOD_ALARM_IE_W<'a, REG> = crate::BitWriter<'a, REG, TOD_ALARM_IE_A>;
impl<'a, REG> TOD_ALARM_IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TOD_ALARM_IE_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TOD_ALARM_IE_A::EN)
    }
}
#[doc = "Field `SSEC_ALARM_IE` reader - Alarm Sub-second Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
pub type SSEC_ALARM_IE_R = crate::BitReader<SSEC_ALARM_IE_A>;
#[doc = "Alarm Sub-second Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSEC_ALARM_IE_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<SSEC_ALARM_IE_A> for bool {
    #[inline(always)]
    fn from(variant: SSEC_ALARM_IE_A) -> Self {
        variant as u8 != 0
    }
}
impl SSEC_ALARM_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSEC_ALARM_IE_A {
        match self.bits {
            false => SSEC_ALARM_IE_A::DIS,
            true => SSEC_ALARM_IE_A::EN,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SSEC_ALARM_IE_A::DIS
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SSEC_ALARM_IE_A::EN
    }
}
#[doc = "Field `SSEC_ALARM_IE` writer - Alarm Sub-second Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
pub type SSEC_ALARM_IE_W<'a, REG> = crate::BitWriter<'a, REG, SSEC_ALARM_IE_A>;
impl<'a, REG> SSEC_ALARM_IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(SSEC_ALARM_IE_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(SSEC_ALARM_IE_A::EN)
    }
}
#[doc = "Field `BUSY` reader - RTC Busy. This bit is set to 1 by hardware when changes to RTC registers required a synchronized version of the register to be in place. This bit is automatically cleared by hardware."]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "RTC Busy. This bit is set to 1 by hardware when changes to RTC registers required a synchronized version of the register to be in place. This bit is automatically cleared by hardware.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: Idle."]
    IDLE = 0,
    #[doc = "1: Busy."]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::IDLE,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "Idle."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY_A::IDLE
    }
    #[doc = "Busy."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
}
#[doc = "Field `RDY` reader - RTC Ready. This bit is set to 1 by hardware when the RTC count registers update. It can be cleared to 0 by software at any time. It will also be cleared to 0 by hardware just prior to an update of the RTC count register."]
pub type RDY_R = crate::BitReader<RDY_A>;
#[doc = "RTC Ready. This bit is set to 1 by hardware when the RTC count registers update. It can be cleared to 0 by software at any time. It will also be cleared to 0 by hardware just prior to an update of the RTC count register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDY_A {
    #[doc = "0: Register has not updated."]
    BUSY = 0,
    #[doc = "1: Ready."]
    READY = 1,
}
impl From<RDY_A> for bool {
    #[inline(always)]
    fn from(variant: RDY_A) -> Self {
        variant as u8 != 0
    }
}
impl RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDY_A {
        match self.bits {
            false => RDY_A::BUSY,
            true => RDY_A::READY,
        }
    }
    #[doc = "Register has not updated."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RDY_A::BUSY
    }
    #[doc = "Ready."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == RDY_A::READY
    }
}
#[doc = "Field `RDY` writer - RTC Ready. This bit is set to 1 by hardware when the RTC count registers update. It can be cleared to 0 by software at any time. It will also be cleared to 0 by hardware just prior to an update of the RTC count register."]
pub type RDY_W<'a, REG> = crate::BitWriter<'a, REG, RDY_A>;
impl<'a, REG> RDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Register has not updated."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(RDY_A::BUSY)
    }
    #[doc = "Ready."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(RDY_A::READY)
    }
}
#[doc = "Field `RDY_IE` reader - RTC Ready Interrupt Enable."]
pub type RDY_IE_R = crate::BitReader<RDY_IE_A>;
#[doc = "RTC Ready Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDY_IE_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<RDY_IE_A> for bool {
    #[inline(always)]
    fn from(variant: RDY_IE_A) -> Self {
        variant as u8 != 0
    }
}
impl RDY_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDY_IE_A {
        match self.bits {
            false => RDY_IE_A::DIS,
            true => RDY_IE_A::EN,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RDY_IE_A::DIS
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RDY_IE_A::EN
    }
}
#[doc = "Field `RDY_IE` writer - RTC Ready Interrupt Enable."]
pub type RDY_IE_W<'a, REG> = crate::BitWriter<'a, REG, RDY_IE_A>;
impl<'a, REG> RDY_IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RDY_IE_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RDY_IE_A::EN)
    }
}
#[doc = "Field `TOD_ALARM` reader - Time-of-Day Alarm Interrupt Flag. This alarm is qualified as wake-up source to the processor."]
pub type TOD_ALARM_R = crate::BitReader<TOD_ALARM_A>;
#[doc = "Time-of-Day Alarm Interrupt Flag. This alarm is qualified as wake-up source to the processor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOD_ALARM_A {
    #[doc = "0: Not active"]
    INACTIVE = 0,
    #[doc = "1: Active"]
    PENDING = 1,
}
impl From<TOD_ALARM_A> for bool {
    #[inline(always)]
    fn from(variant: TOD_ALARM_A) -> Self {
        variant as u8 != 0
    }
}
impl TOD_ALARM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TOD_ALARM_A {
        match self.bits {
            false => TOD_ALARM_A::INACTIVE,
            true => TOD_ALARM_A::PENDING,
        }
    }
    #[doc = "Not active"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == TOD_ALARM_A::INACTIVE
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TOD_ALARM_A::PENDING
    }
}
#[doc = "Field `SSEC_ALARM` reader - Sub-second Alarm Interrupt Flag. This alarm is qualified as wake-up source to the processor."]
pub type SSEC_ALARM_R = crate::BitReader<SSEC_ALARM_A>;
#[doc = "Sub-second Alarm Interrupt Flag. This alarm is qualified as wake-up source to the processor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSEC_ALARM_A {
    #[doc = "0: Not active"]
    INACTIVE = 0,
    #[doc = "1: Active"]
    PENDING = 1,
}
impl From<SSEC_ALARM_A> for bool {
    #[inline(always)]
    fn from(variant: SSEC_ALARM_A) -> Self {
        variant as u8 != 0
    }
}
impl SSEC_ALARM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSEC_ALARM_A {
        match self.bits {
            false => SSEC_ALARM_A::INACTIVE,
            true => SSEC_ALARM_A::PENDING,
        }
    }
    #[doc = "Not active"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == SSEC_ALARM_A::INACTIVE
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SSEC_ALARM_A::PENDING
    }
}
#[doc = "Field `SQW_EN` reader - Square Wave Output Enable."]
pub type SQW_EN_R = crate::BitReader<SQW_EN_A>;
#[doc = "Square Wave Output Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SQW_EN_A {
    #[doc = "0: Not active"]
    INACTIVE = 0,
    #[doc = "1: Active"]
    PENDING = 1,
}
impl From<SQW_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SQW_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SQW_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SQW_EN_A {
        match self.bits {
            false => SQW_EN_A::INACTIVE,
            true => SQW_EN_A::PENDING,
        }
    }
    #[doc = "Not active"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == SQW_EN_A::INACTIVE
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SQW_EN_A::PENDING
    }
}
#[doc = "Field `SQW_EN` writer - Square Wave Output Enable."]
pub type SQW_EN_W<'a, REG> = crate::BitWriter<'a, REG, SQW_EN_A>;
impl<'a, REG> SQW_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not active"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(SQW_EN_A::INACTIVE)
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(SQW_EN_A::PENDING)
    }
}
#[doc = "Field `SQW_SEL` reader - Frequency Output Selection. When SQE=1, these bits specify the output frequency on the SQW pin."]
pub type SQW_SEL_R = crate::FieldReader<SQW_SEL_A>;
#[doc = "Frequency Output Selection. When SQE=1, these bits specify the output frequency on the SQW pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SQW_SEL_A {
    #[doc = "0: 1 Hz (Compensated)."]
    FREQ1HZ = 0,
    #[doc = "1: 512 Hz (Compensated)."]
    FREQ512HZ = 1,
    #[doc = "2: 4 KHz."]
    FREQ4KHZ = 2,
    #[doc = "3: RTC Input Clock / 8."]
    CLK_DIV8 = 3,
}
impl From<SQW_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SQW_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SQW_SEL_A {
    type Ux = u8;
}
impl SQW_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SQW_SEL_A {
        match self.bits {
            0 => SQW_SEL_A::FREQ1HZ,
            1 => SQW_SEL_A::FREQ512HZ,
            2 => SQW_SEL_A::FREQ4KHZ,
            3 => SQW_SEL_A::CLK_DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "1 Hz (Compensated)."]
    #[inline(always)]
    pub fn is_freq1hz(&self) -> bool {
        *self == SQW_SEL_A::FREQ1HZ
    }
    #[doc = "512 Hz (Compensated)."]
    #[inline(always)]
    pub fn is_freq512hz(&self) -> bool {
        *self == SQW_SEL_A::FREQ512HZ
    }
    #[doc = "4 KHz."]
    #[inline(always)]
    pub fn is_freq4khz(&self) -> bool {
        *self == SQW_SEL_A::FREQ4KHZ
    }
    #[doc = "RTC Input Clock / 8."]
    #[inline(always)]
    pub fn is_clk_div8(&self) -> bool {
        *self == SQW_SEL_A::CLK_DIV8
    }
}
#[doc = "Field `SQW_SEL` writer - Frequency Output Selection. When SQE=1, these bits specify the output frequency on the SQW pin."]
pub type SQW_SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SQW_SEL_A>;
impl<'a, REG> SQW_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 Hz (Compensated)."]
    #[inline(always)]
    pub fn freq1hz(self) -> &'a mut crate::W<REG> {
        self.variant(SQW_SEL_A::FREQ1HZ)
    }
    #[doc = "512 Hz (Compensated)."]
    #[inline(always)]
    pub fn freq512hz(self) -> &'a mut crate::W<REG> {
        self.variant(SQW_SEL_A::FREQ512HZ)
    }
    #[doc = "4 KHz."]
    #[inline(always)]
    pub fn freq4khz(self) -> &'a mut crate::W<REG> {
        self.variant(SQW_SEL_A::FREQ4KHZ)
    }
    #[doc = "RTC Input Clock / 8."]
    #[inline(always)]
    pub fn clk_div8(self) -> &'a mut crate::W<REG> {
        self.variant(SQW_SEL_A::CLK_DIV8)
    }
}
#[doc = "Field `RD_EN` reader - Asynchronous Counter Read Enable."]
pub type RD_EN_R = crate::BitReader;
#[doc = "Field `RD_EN` writer - Asynchronous Counter Read Enable."]
pub type RD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_EN` reader - Write Enable. This register bit serves as a protection mechanism against unintentional writes to critical RTC bits."]
pub type WR_EN_R = crate::BitReader<WR_EN_A>;
#[doc = "Write Enable. This register bit serves as a protection mechanism against unintentional writes to critical RTC bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WR_EN_A {
    #[doc = "0: Not active"]
    INACTIVE = 0,
    #[doc = "1: Active"]
    PENDING = 1,
}
impl From<WR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WR_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl WR_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WR_EN_A {
        match self.bits {
            false => WR_EN_A::INACTIVE,
            true => WR_EN_A::PENDING,
        }
    }
    #[doc = "Not active"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == WR_EN_A::INACTIVE
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == WR_EN_A::PENDING
    }
}
#[doc = "Field `WR_EN` writer - Write Enable. This register bit serves as a protection mechanism against unintentional writes to critical RTC bits."]
pub type WR_EN_W<'a, REG> = crate::BitWriter<'a, REG, WR_EN_A>;
impl<'a, REG> WR_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not active"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(WR_EN_A::INACTIVE)
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(WR_EN_A::PENDING)
    }
}
impl R {
    #[doc = "Bit 0 - Real Time Clock Enable. This bit enables the Real Time Clock. This bit can only be written when WE=1 and BUSY =0. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm Time-of-Day Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
    #[inline(always)]
    pub fn tod_alarm_ie(&self) -> TOD_ALARM_IE_R {
        TOD_ALARM_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Alarm Sub-second Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
    #[inline(always)]
    pub fn ssec_alarm_ie(&self) -> SSEC_ALARM_IE_R {
        SSEC_ALARM_IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC Busy. This bit is set to 1 by hardware when changes to RTC registers required a synchronized version of the register to be in place. This bit is automatically cleared by hardware."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC Ready. This bit is set to 1 by hardware when the RTC count registers update. It can be cleared to 0 by software at any time. It will also be cleared to 0 by hardware just prior to an update of the RTC count register."]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC Ready Interrupt Enable."]
    #[inline(always)]
    pub fn rdy_ie(&self) -> RDY_IE_R {
        RDY_IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Time-of-Day Alarm Interrupt Flag. This alarm is qualified as wake-up source to the processor."]
    #[inline(always)]
    pub fn tod_alarm(&self) -> TOD_ALARM_R {
        TOD_ALARM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sub-second Alarm Interrupt Flag. This alarm is qualified as wake-up source to the processor."]
    #[inline(always)]
    pub fn ssec_alarm(&self) -> SSEC_ALARM_R {
        SSEC_ALARM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Square Wave Output Enable."]
    #[inline(always)]
    pub fn sqw_en(&self) -> SQW_EN_R {
        SQW_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Frequency Output Selection. When SQE=1, these bits specify the output frequency on the SQW pin."]
    #[inline(always)]
    pub fn sqw_sel(&self) -> SQW_SEL_R {
        SQW_SEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 14 - Asynchronous Counter Read Enable."]
    #[inline(always)]
    pub fn rd_en(&self) -> RD_EN_R {
        RD_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write Enable. This register bit serves as a protection mechanism against unintentional writes to critical RTC bits."]
    #[inline(always)]
    pub fn wr_en(&self) -> WR_EN_R {
        WR_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Enable. This bit enables the Real Time Clock. This bit can only be written when WE=1 and BUSY =0. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CTRL_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm Time-of-Day Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
    #[inline(always)]
    #[must_use]
    pub fn tod_alarm_ie(&mut self) -> TOD_ALARM_IE_W<CTRL_SPEC> {
        TOD_ALARM_IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Alarm Sub-second Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
    #[inline(always)]
    #[must_use]
    pub fn ssec_alarm_ie(&mut self) -> SSEC_ALARM_IE_W<CTRL_SPEC> {
        SSEC_ALARM_IE_W::new(self, 2)
    }
    #[doc = "Bit 4 - RTC Ready. This bit is set to 1 by hardware when the RTC count registers update. It can be cleared to 0 by software at any time. It will also be cleared to 0 by hardware just prior to an update of the RTC count register."]
    #[inline(always)]
    #[must_use]
    pub fn rdy(&mut self) -> RDY_W<CTRL_SPEC> {
        RDY_W::new(self, 4)
    }
    #[doc = "Bit 5 - RTC Ready Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn rdy_ie(&mut self) -> RDY_IE_W<CTRL_SPEC> {
        RDY_IE_W::new(self, 5)
    }
    #[doc = "Bit 8 - Square Wave Output Enable."]
    #[inline(always)]
    #[must_use]
    pub fn sqw_en(&mut self) -> SQW_EN_W<CTRL_SPEC> {
        SQW_EN_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - Frequency Output Selection. When SQE=1, these bits specify the output frequency on the SQW pin."]
    #[inline(always)]
    #[must_use]
    pub fn sqw_sel(&mut self) -> SQW_SEL_W<CTRL_SPEC> {
        SQW_SEL_W::new(self, 9)
    }
    #[doc = "Bit 14 - Asynchronous Counter Read Enable."]
    #[inline(always)]
    #[must_use]
    pub fn rd_en(&mut self) -> RD_EN_W<CTRL_SPEC> {
        RD_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Write Enable. This register bit serves as a protection mechanism against unintentional writes to critical RTC bits."]
    #[inline(always)]
    #[must_use]
    pub fn wr_en(&mut self) -> WR_EN_W<CTRL_SPEC> {
        WR_EN_W::new(self, 15)
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
#[doc = "RTC Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0x08"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
