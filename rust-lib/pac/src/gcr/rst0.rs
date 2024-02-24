#[doc = "Register `RST0` reader"]
pub type R = crate::R<RST0_SPEC>;
#[doc = "Register `RST0` writer"]
pub type W = crate::W<RST0_SPEC>;
#[doc = "Field `DMA` reader - DMA Reset."]
pub type DMA_R = crate::BitReader<RESET_A>;
#[doc = "DMA Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Starts Reset or indicates reset in progress."]
    BUSY = 1,
}
impl From<RESET_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESET_A {
        match self.bits {
            false => RESET_A::RESET_DONE,
            true => RESET_A::BUSY,
        }
    }
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == RESET_A::RESET_DONE
    }
    #[doc = "Starts Reset or indicates reset in progress."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RESET_A::BUSY
    }
}
#[doc = "Field `DMA` writer - DMA Reset."]
pub type DMA_W<'a, REG> = crate::BitWriter<'a, REG, RESET_A>;
impl<'a, REG> DMA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn reset_done(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_A::RESET_DONE)
    }
    #[doc = "Starts Reset or indicates reset in progress."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_A::BUSY)
    }
}
#[doc = "Field `WDT0` reader - Watchdog Timer 0 Reset."]
pub use DMA_R as WDT0_R;
#[doc = "Field `GPIO0` reader - GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states."]
pub use DMA_R as GPIO0_R;
#[doc = "Field `GPIO1` reader - GPIO1 Reset. Setting this bit to 1 resets GPIO1 pins to their default states."]
pub use DMA_R as GPIO1_R;
#[doc = "Field `TMR0` reader - Timer 0 Reset. Setting this bit to 1 resets Timer 0 blocks."]
pub use DMA_R as TMR0_R;
#[doc = "Field `TMR1` reader - Timer 1 Reset. Setting this bit to 1 resets Timer 1 blocks."]
pub use DMA_R as TMR1_R;
#[doc = "Field `TMR2` reader - Timer 2 Reset. Setting this bit to 1 resets Timer 2 blocks."]
pub use DMA_R as TMR2_R;
#[doc = "Field `TMR3` reader - Timer 3 Reset. Setting this bit to 1 resets Timer 3 blocks."]
pub use DMA_R as TMR3_R;
#[doc = "Field `UART0` reader - UART 0 Reset. Setting this bit to 1 resets all UART 0 blocks."]
pub use DMA_R as UART0_R;
#[doc = "Field `UART1` reader - UART 1 Reset. Setting this bit to 1 resets all UART 1 blocks."]
pub use DMA_R as UART1_R;
#[doc = "Field `SPI1` reader - SPI 1 Reset. Setting this bit to 1 resets all SPI 1 blocks."]
pub use DMA_R as SPI1_R;
#[doc = "Field `I2C0` reader - I2C 0 Reset."]
pub use DMA_R as I2C0_R;
#[doc = "Field `RTC` reader - Real Time Clock Reset."]
pub use DMA_R as RTC_R;
#[doc = "Field `SMPHR` reader - Semaphore Reset."]
pub use DMA_R as SMPHR_R;
#[doc = "Field `TRNG` reader - TRNG Reset. This reset is only available during the manufacture testing phase."]
pub use DMA_R as TRNG_R;
#[doc = "Field `CNN` reader - CNN Reset."]
pub use DMA_R as CNN_R;
#[doc = "Field `ADC` reader - ADC Reset."]
pub use DMA_R as ADC_R;
#[doc = "Field `UART2` reader - UART2 Reset. Setting this bit to 1 resets all UART 2 blocks."]
pub use DMA_R as UART2_R;
#[doc = "Field `SOFT` reader - Soft Reset. Setting this bit to 1 resets everything except the CPU and the watchdog timer."]
pub use DMA_R as SOFT_R;
#[doc = "Field `PERIPH` reader - Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset."]
pub use DMA_R as PERIPH_R;
#[doc = "Field `SYS` reader - System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer."]
pub use DMA_R as SYS_R;
#[doc = "Field `WDT0` writer - Watchdog Timer 0 Reset."]
pub use DMA_W as WDT0_W;
#[doc = "Field `GPIO0` writer - GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states."]
pub use DMA_W as GPIO0_W;
#[doc = "Field `GPIO1` writer - GPIO1 Reset. Setting this bit to 1 resets GPIO1 pins to their default states."]
pub use DMA_W as GPIO1_W;
#[doc = "Field `TMR0` writer - Timer 0 Reset. Setting this bit to 1 resets Timer 0 blocks."]
pub use DMA_W as TMR0_W;
#[doc = "Field `TMR1` writer - Timer 1 Reset. Setting this bit to 1 resets Timer 1 blocks."]
pub use DMA_W as TMR1_W;
#[doc = "Field `TMR2` writer - Timer 2 Reset. Setting this bit to 1 resets Timer 2 blocks."]
pub use DMA_W as TMR2_W;
#[doc = "Field `TMR3` writer - Timer 3 Reset. Setting this bit to 1 resets Timer 3 blocks."]
pub use DMA_W as TMR3_W;
#[doc = "Field `UART0` writer - UART 0 Reset. Setting this bit to 1 resets all UART 0 blocks."]
pub use DMA_W as UART0_W;
#[doc = "Field `UART1` writer - UART 1 Reset. Setting this bit to 1 resets all UART 1 blocks."]
pub use DMA_W as UART1_W;
#[doc = "Field `SPI1` writer - SPI 1 Reset. Setting this bit to 1 resets all SPI 1 blocks."]
pub use DMA_W as SPI1_W;
#[doc = "Field `I2C0` writer - I2C 0 Reset."]
pub use DMA_W as I2C0_W;
#[doc = "Field `RTC` writer - Real Time Clock Reset."]
pub use DMA_W as RTC_W;
#[doc = "Field `SMPHR` writer - Semaphore Reset."]
pub use DMA_W as SMPHR_W;
#[doc = "Field `TRNG` writer - TRNG Reset. This reset is only available during the manufacture testing phase."]
pub use DMA_W as TRNG_W;
#[doc = "Field `CNN` writer - CNN Reset."]
pub use DMA_W as CNN_W;
#[doc = "Field `ADC` writer - ADC Reset."]
pub use DMA_W as ADC_W;
#[doc = "Field `UART2` writer - UART2 Reset. Setting this bit to 1 resets all UART 2 blocks."]
pub use DMA_W as UART2_W;
#[doc = "Field `SOFT` writer - Soft Reset. Setting this bit to 1 resets everything except the CPU and the watchdog timer."]
pub use DMA_W as SOFT_W;
#[doc = "Field `PERIPH` writer - Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset."]
pub use DMA_W as PERIPH_W;
#[doc = "Field `SYS` writer - System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer."]
pub use DMA_W as SYS_W;
impl R {
    #[doc = "Bit 0 - DMA Reset."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Timer 0 Reset."]
    #[inline(always)]
    pub fn wdt0(&self) -> WDT0_R {
        WDT0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO1 Reset. Setting this bit to 1 resets GPIO1 pins to their default states."]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO1_R {
        GPIO1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 0 Reset. Setting this bit to 1 resets Timer 0 blocks."]
    #[inline(always)]
    pub fn tmr0(&self) -> TMR0_R {
        TMR0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer 1 Reset. Setting this bit to 1 resets Timer 1 blocks."]
    #[inline(always)]
    pub fn tmr1(&self) -> TMR1_R {
        TMR1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer 2 Reset. Setting this bit to 1 resets Timer 2 blocks."]
    #[inline(always)]
    pub fn tmr2(&self) -> TMR2_R {
        TMR2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer 3 Reset. Setting this bit to 1 resets Timer 3 blocks."]
    #[inline(always)]
    pub fn tmr3(&self) -> TMR3_R {
        TMR3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - UART 0 Reset. Setting this bit to 1 resets all UART 0 blocks."]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - UART 1 Reset. Setting this bit to 1 resets all UART 1 blocks."]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI 1 Reset. Setting this bit to 1 resets all SPI 1 blocks."]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - I2C 0 Reset."]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Real Time Clock Reset."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 22 - Semaphore Reset."]
    #[inline(always)]
    pub fn smphr(&self) -> SMPHR_R {
        SMPHR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - TRNG Reset. This reset is only available during the manufacture testing phase."]
    #[inline(always)]
    pub fn trng(&self) -> TRNG_R {
        TRNG_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CNN Reset."]
    #[inline(always)]
    pub fn cnn(&self) -> CNN_R {
        CNN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - ADC Reset."]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - UART2 Reset. Setting this bit to 1 resets all UART 2 blocks."]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Soft Reset. Setting this bit to 1 resets everything except the CPU and the watchdog timer."]
    #[inline(always)]
    pub fn soft(&self) -> SOFT_R {
        SOFT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset."]
    #[inline(always)]
    pub fn periph(&self) -> PERIPH_R {
        PERIPH_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer."]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Reset."]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<RST0_SPEC> {
        DMA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Watchdog Timer 0 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn wdt0(&mut self) -> WDT0_W<RST0_SPEC> {
        WDT0_W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states."]
    #[inline(always)]
    #[must_use]
    pub fn gpio0(&mut self) -> GPIO0_W<RST0_SPEC> {
        GPIO0_W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO1 Reset. Setting this bit to 1 resets GPIO1 pins to their default states."]
    #[inline(always)]
    #[must_use]
    pub fn gpio1(&mut self) -> GPIO1_W<RST0_SPEC> {
        GPIO1_W::new(self, 3)
    }
    #[doc = "Bit 5 - Timer 0 Reset. Setting this bit to 1 resets Timer 0 blocks."]
    #[inline(always)]
    #[must_use]
    pub fn tmr0(&mut self) -> TMR0_W<RST0_SPEC> {
        TMR0_W::new(self, 5)
    }
    #[doc = "Bit 6 - Timer 1 Reset. Setting this bit to 1 resets Timer 1 blocks."]
    #[inline(always)]
    #[must_use]
    pub fn tmr1(&mut self) -> TMR1_W<RST0_SPEC> {
        TMR1_W::new(self, 6)
    }
    #[doc = "Bit 7 - Timer 2 Reset. Setting this bit to 1 resets Timer 2 blocks."]
    #[inline(always)]
    #[must_use]
    pub fn tmr2(&mut self) -> TMR2_W<RST0_SPEC> {
        TMR2_W::new(self, 7)
    }
    #[doc = "Bit 8 - Timer 3 Reset. Setting this bit to 1 resets Timer 3 blocks."]
    #[inline(always)]
    #[must_use]
    pub fn tmr3(&mut self) -> TMR3_W<RST0_SPEC> {
        TMR3_W::new(self, 8)
    }
    #[doc = "Bit 11 - UART 0 Reset. Setting this bit to 1 resets all UART 0 blocks."]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> UART0_W<RST0_SPEC> {
        UART0_W::new(self, 11)
    }
    #[doc = "Bit 12 - UART 1 Reset. Setting this bit to 1 resets all UART 1 blocks."]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<RST0_SPEC> {
        UART1_W::new(self, 12)
    }
    #[doc = "Bit 13 - SPI 1 Reset. Setting this bit to 1 resets all SPI 1 blocks."]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<RST0_SPEC> {
        SPI1_W::new(self, 13)
    }
    #[doc = "Bit 16 - I2C 0 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2C0_W<RST0_SPEC> {
        I2C0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Real Time Clock Reset."]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<RST0_SPEC> {
        RTC_W::new(self, 17)
    }
    #[doc = "Bit 22 - Semaphore Reset."]
    #[inline(always)]
    #[must_use]
    pub fn smphr(&mut self) -> SMPHR_W<RST0_SPEC> {
        SMPHR_W::new(self, 22)
    }
    #[doc = "Bit 24 - TRNG Reset. This reset is only available during the manufacture testing phase."]
    #[inline(always)]
    #[must_use]
    pub fn trng(&mut self) -> TRNG_W<RST0_SPEC> {
        TRNG_W::new(self, 24)
    }
    #[doc = "Bit 25 - CNN Reset."]
    #[inline(always)]
    #[must_use]
    pub fn cnn(&mut self) -> CNN_W<RST0_SPEC> {
        CNN_W::new(self, 25)
    }
    #[doc = "Bit 26 - ADC Reset."]
    #[inline(always)]
    #[must_use]
    pub fn adc(&mut self) -> ADC_W<RST0_SPEC> {
        ADC_W::new(self, 26)
    }
    #[doc = "Bit 28 - UART2 Reset. Setting this bit to 1 resets all UART 2 blocks."]
    #[inline(always)]
    #[must_use]
    pub fn uart2(&mut self) -> UART2_W<RST0_SPEC> {
        UART2_W::new(self, 28)
    }
    #[doc = "Bit 29 - Soft Reset. Setting this bit to 1 resets everything except the CPU and the watchdog timer."]
    #[inline(always)]
    #[must_use]
    pub fn soft(&mut self) -> SOFT_W<RST0_SPEC> {
        SOFT_W::new(self, 29)
    }
    #[doc = "Bit 30 - Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset."]
    #[inline(always)]
    #[must_use]
    pub fn periph(&mut self) -> PERIPH_W<RST0_SPEC> {
        PERIPH_W::new(self, 30)
    }
    #[doc = "Bit 31 - System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer."]
    #[inline(always)]
    #[must_use]
    pub fn sys(&mut self) -> SYS_W<RST0_SPEC> {
        SYS_W::new(self, 31)
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
#[doc = "Reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RST0_SPEC;
impl crate::RegisterSpec for RST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst0::R`](R) reader structure"]
impl crate::Readable for RST0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rst0::W`](W) writer structure"]
impl crate::Writable for RST0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RST0 to value 0"]
impl crate::Resettable for RST0_SPEC {
    const RESET_VALUE: u32 = 0;
}
