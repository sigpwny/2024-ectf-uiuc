#[doc = "Register `PM` reader"]
pub type R = crate::R<PM_SPEC>;
#[doc = "Register `PM` writer"]
pub type W = crate::W<PM_SPEC>;
#[doc = "Field `MODE` reader - Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode."]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Active Mode."]
    ACTIVE = 0,
    #[doc = "1: Cortex-M4 Active, RISC-V Sleep Mode."]
    SLEEP = 1,
    #[doc = "2: Standby Mode."]
    STANDBY = 2,
    #[doc = "4: Backup Mode."]
    BACKUP = 4,
    #[doc = "8: LPM or CM4 Deep Sleep Mode."]
    LPM = 8,
    #[doc = "9: UPM."]
    UPM = 9,
    #[doc = "10: Power Down Mode."]
    POWERDOWN = 10,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::ACTIVE),
            1 => Some(MODE_A::SLEEP),
            2 => Some(MODE_A::STANDBY),
            4 => Some(MODE_A::BACKUP),
            8 => Some(MODE_A::LPM),
            9 => Some(MODE_A::UPM),
            10 => Some(MODE_A::POWERDOWN),
            _ => None,
        }
    }
    #[doc = "Active Mode."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == MODE_A::ACTIVE
    }
    #[doc = "Cortex-M4 Active, RISC-V Sleep Mode."]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == MODE_A::SLEEP
    }
    #[doc = "Standby Mode."]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == MODE_A::STANDBY
    }
    #[doc = "Backup Mode."]
    #[inline(always)]
    pub fn is_backup(&self) -> bool {
        *self == MODE_A::BACKUP
    }
    #[doc = "LPM or CM4 Deep Sleep Mode."]
    #[inline(always)]
    pub fn is_lpm(&self) -> bool {
        *self == MODE_A::LPM
    }
    #[doc = "UPM."]
    #[inline(always)]
    pub fn is_upm(&self) -> bool {
        *self == MODE_A::UPM
    }
    #[doc = "Power Down Mode."]
    #[inline(always)]
    pub fn is_powerdown(&self) -> bool {
        *self == MODE_A::POWERDOWN
    }
}
#[doc = "Field `MODE` writer - Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode."]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Active Mode."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::ACTIVE)
    }
    #[doc = "Cortex-M4 Active, RISC-V Sleep Mode."]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::SLEEP)
    }
    #[doc = "Standby Mode."]
    #[inline(always)]
    pub fn standby(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::STANDBY)
    }
    #[doc = "Backup Mode."]
    #[inline(always)]
    pub fn backup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::BACKUP)
    }
    #[doc = "LPM or CM4 Deep Sleep Mode."]
    #[inline(always)]
    pub fn lpm(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::LPM)
    }
    #[doc = "UPM."]
    #[inline(always)]
    pub fn upm(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::UPM)
    }
    #[doc = "Power Down Mode."]
    #[inline(always)]
    pub fn powerdown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::POWERDOWN)
    }
}
#[doc = "Field `GPIO_WE` reader - GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set."]
pub type GPIO_WE_R = crate::BitReader<GPIO_WE_A>;
#[doc = "GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_WE_A {
    #[doc = "0: Wake Up Disable."]
    DIS = 0,
    #[doc = "1: Wake Up Enable."]
    EN = 1,
}
impl From<GPIO_WE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_WE_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_WE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIO_WE_A {
        match self.bits {
            false => GPIO_WE_A::DIS,
            true => GPIO_WE_A::EN,
        }
    }
    #[doc = "Wake Up Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO_WE_A::DIS
    }
    #[doc = "Wake Up Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GPIO_WE_A::EN
    }
}
#[doc = "Field `GPIO_WE` writer - GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set."]
pub type GPIO_WE_W<'a, REG> = crate::BitWriter<'a, REG, GPIO_WE_A>;
impl<'a, REG> GPIO_WE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake Up Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_WE_A::DIS)
    }
    #[doc = "Wake Up Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_WE_A::EN)
    }
}
#[doc = "Field `RTC_WE` reader - RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers."]
pub use GPIO_WE_R as RTC_WE_R;
#[doc = "Field `WUT_WE` reader - WUT Wake Up Enable. This bit enables the Wake-Up Timer as wakeup source."]
pub use GPIO_WE_R as WUT_WE_R;
#[doc = "Field `AINCOMP_WE` reader - AIN COMP Wake Up Enable. This bit enables AIN COMP as wakeup source."]
pub use GPIO_WE_R as AINCOMP_WE_R;
#[doc = "Field `RTC_WE` writer - RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers."]
pub use GPIO_WE_W as RTC_WE_W;
#[doc = "Field `WUT_WE` writer - WUT Wake Up Enable. This bit enables the Wake-Up Timer as wakeup source."]
pub use GPIO_WE_W as WUT_WE_W;
#[doc = "Field `AINCOMP_WE` writer - AIN COMP Wake Up Enable. This bit enables AIN COMP as wakeup source."]
pub use GPIO_WE_W as AINCOMP_WE_W;
#[doc = "Field `ISO_PD` reader - 60 MHz power down. This bit selects the 60 MHz clock power state in DEEPSLEEP mode."]
pub type ISO_PD_R = crate::BitReader<ISO_PD_A>;
#[doc = "60 MHz power down. This bit selects the 60 MHz clock power state in DEEPSLEEP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISO_PD_A {
    #[doc = "0: Mode is Active."]
    ACTIVE = 0,
    #[doc = "1: Powered down in DEEPSLEEP."]
    DEEPSLEEP = 1,
}
impl From<ISO_PD_A> for bool {
    #[inline(always)]
    fn from(variant: ISO_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl ISO_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ISO_PD_A {
        match self.bits {
            false => ISO_PD_A::ACTIVE,
            true => ISO_PD_A::DEEPSLEEP,
        }
    }
    #[doc = "Mode is Active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ISO_PD_A::ACTIVE
    }
    #[doc = "Powered down in DEEPSLEEP."]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == ISO_PD_A::DEEPSLEEP
    }
}
#[doc = "Field `ISO_PD` writer - 60 MHz power down. This bit selects the 60 MHz clock power state in DEEPSLEEP mode."]
pub type ISO_PD_W<'a, REG> = crate::BitWriter<'a, REG, ISO_PD_A>;
impl<'a, REG> ISO_PD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mode is Active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(ISO_PD_A::ACTIVE)
    }
    #[doc = "Powered down in DEEPSLEEP."]
    #[inline(always)]
    pub fn deepsleep(self) -> &'a mut crate::W<REG> {
        self.variant(ISO_PD_A::DEEPSLEEP)
    }
}
#[doc = "Field `IPO_PD` reader - 100 MHz power down. This bit selects 100 MHz clock power state in DEEPSLEEP mode."]
pub use ISO_PD_R as IPO_PD_R;
#[doc = "Field `IBRO_PD` reader - 7.3725 MHz power down. This bit selects 7.3725 MHz clock power state in DEEPSLEEP mode."]
pub use ISO_PD_R as IBRO_PD_R;
#[doc = "Field `IPO_PD` writer - 100 MHz power down. This bit selects 100 MHz clock power state in DEEPSLEEP mode."]
pub use ISO_PD_W as IPO_PD_W;
#[doc = "Field `IBRO_PD` writer - 7.3725 MHz power down. This bit selects 7.3725 MHz clock power state in DEEPSLEEP mode."]
pub use ISO_PD_W as IBRO_PD_W;
impl R {
    #[doc = "Bits 0:3 - Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set."]
    #[inline(always)]
    pub fn gpio_we(&self) -> GPIO_WE_R {
        GPIO_WE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers."]
    #[inline(always)]
    pub fn rtc_we(&self) -> RTC_WE_R {
        RTC_WE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - WUT Wake Up Enable. This bit enables the Wake-Up Timer as wakeup source."]
    #[inline(always)]
    pub fn wut_we(&self) -> WUT_WE_R {
        WUT_WE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - AIN COMP Wake Up Enable. This bit enables AIN COMP as wakeup source."]
    #[inline(always)]
    pub fn aincomp_we(&self) -> AINCOMP_WE_R {
        AINCOMP_WE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - 60 MHz power down. This bit selects the 60 MHz clock power state in DEEPSLEEP mode."]
    #[inline(always)]
    pub fn iso_pd(&self) -> ISO_PD_R {
        ISO_PD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 100 MHz power down. This bit selects 100 MHz clock power state in DEEPSLEEP mode."]
    #[inline(always)]
    pub fn ipo_pd(&self) -> IPO_PD_R {
        IPO_PD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 7.3725 MHz power down. This bit selects 7.3725 MHz clock power state in DEEPSLEEP mode."]
    #[inline(always)]
    pub fn ibro_pd(&self) -> IBRO_PD_R {
        IBRO_PD_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<PM_SPEC> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bit 4 - GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_we(&mut self) -> GPIO_WE_W<PM_SPEC> {
        GPIO_WE_W::new(self, 4)
    }
    #[doc = "Bit 5 - RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_we(&mut self) -> RTC_WE_W<PM_SPEC> {
        RTC_WE_W::new(self, 5)
    }
    #[doc = "Bit 7 - WUT Wake Up Enable. This bit enables the Wake-Up Timer as wakeup source."]
    #[inline(always)]
    #[must_use]
    pub fn wut_we(&mut self) -> WUT_WE_W<PM_SPEC> {
        WUT_WE_W::new(self, 7)
    }
    #[doc = "Bit 9 - AIN COMP Wake Up Enable. This bit enables AIN COMP as wakeup source."]
    #[inline(always)]
    #[must_use]
    pub fn aincomp_we(&mut self) -> AINCOMP_WE_W<PM_SPEC> {
        AINCOMP_WE_W::new(self, 9)
    }
    #[doc = "Bit 15 - 60 MHz power down. This bit selects the 60 MHz clock power state in DEEPSLEEP mode."]
    #[inline(always)]
    #[must_use]
    pub fn iso_pd(&mut self) -> ISO_PD_W<PM_SPEC> {
        ISO_PD_W::new(self, 15)
    }
    #[doc = "Bit 16 - 100 MHz power down. This bit selects 100 MHz clock power state in DEEPSLEEP mode."]
    #[inline(always)]
    #[must_use]
    pub fn ipo_pd(&mut self) -> IPO_PD_W<PM_SPEC> {
        IPO_PD_W::new(self, 16)
    }
    #[doc = "Bit 17 - 7.3725 MHz power down. This bit selects 7.3725 MHz clock power state in DEEPSLEEP mode."]
    #[inline(always)]
    #[must_use]
    pub fn ibro_pd(&mut self) -> IBRO_PD_W<PM_SPEC> {
        IBRO_PD_W::new(self, 17)
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
#[doc = "Power Management.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PM_SPEC;
impl crate::RegisterSpec for PM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pm::R`](R) reader structure"]
impl crate::Readable for PM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pm::W`](W) writer structure"]
impl crate::Writable for PM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PM to value 0"]
impl crate::Resettable for PM_SPEC {
    const RESET_VALUE: u32 = 0;
}
