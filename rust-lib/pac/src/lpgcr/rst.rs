#[doc = "Register `RST` reader"]
pub type R = crate::R<RST_SPEC>;
#[doc = "Register `RST` writer"]
pub type W = crate::W<RST_SPEC>;
#[doc = "Field `GPIO2` reader - Low Power GPIO 2 Reset."]
pub type GPIO2_R = crate::BitReader<RESET_A>;
#[doc = "Low Power GPIO 2 Reset.\n\nValue on reset: 0"]
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
impl GPIO2_R {
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
#[doc = "Field `GPIO2` writer - Low Power GPIO 2 Reset."]
pub type GPIO2_W<'a, REG> = crate::BitWriter<'a, REG, RESET_A>;
impl<'a, REG> GPIO2_W<'a, REG>
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
#[doc = "Field `WDT1` reader - Low Power Watchdog Timer 1 Reset."]
pub use GPIO2_R as WDT1_R;
#[doc = "Field `TMR4` reader - Low Power Timer 4 Reset."]
pub use GPIO2_R as TMR4_R;
#[doc = "Field `TMR5` reader - Low Power Timer 5 Reset."]
pub use GPIO2_R as TMR5_R;
#[doc = "Field `UART3` reader - Low Power UART 3 Reset."]
pub use GPIO2_R as UART3_R;
#[doc = "Field `LPCOMP` reader - Low Power Comparator Reset."]
pub use GPIO2_R as LPCOMP_R;
#[doc = "Field `WDT1` writer - Low Power Watchdog Timer 1 Reset."]
pub use GPIO2_W as WDT1_W;
#[doc = "Field `TMR4` writer - Low Power Timer 4 Reset."]
pub use GPIO2_W as TMR4_W;
#[doc = "Field `TMR5` writer - Low Power Timer 5 Reset."]
pub use GPIO2_W as TMR5_W;
#[doc = "Field `UART3` writer - Low Power UART 3 Reset."]
pub use GPIO2_W as UART3_W;
#[doc = "Field `LPCOMP` writer - Low Power Comparator Reset."]
pub use GPIO2_W as LPCOMP_W;
impl R {
    #[doc = "Bit 0 - Low Power GPIO 2 Reset."]
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO2_R {
        GPIO2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Power Watchdog Timer 1 Reset."]
    #[inline(always)]
    pub fn wdt1(&self) -> WDT1_R {
        WDT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Power Timer 4 Reset."]
    #[inline(always)]
    pub fn tmr4(&self) -> TMR4_R {
        TMR4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Low Power Timer 5 Reset."]
    #[inline(always)]
    pub fn tmr5(&self) -> TMR5_R {
        TMR5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Power UART 3 Reset."]
    #[inline(always)]
    pub fn uart3(&self) -> UART3_R {
        UART3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Low Power Comparator Reset."]
    #[inline(always)]
    pub fn lpcomp(&self) -> LPCOMP_R {
        LPCOMP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Power GPIO 2 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn gpio2(&mut self) -> GPIO2_W<RST_SPEC> {
        GPIO2_W::new(self, 0)
    }
    #[doc = "Bit 1 - Low Power Watchdog Timer 1 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn wdt1(&mut self) -> WDT1_W<RST_SPEC> {
        WDT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Low Power Timer 4 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn tmr4(&mut self) -> TMR4_W<RST_SPEC> {
        TMR4_W::new(self, 2)
    }
    #[doc = "Bit 3 - Low Power Timer 5 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn tmr5(&mut self) -> TMR5_W<RST_SPEC> {
        TMR5_W::new(self, 3)
    }
    #[doc = "Bit 4 - Low Power UART 3 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn uart3(&mut self) -> UART3_W<RST_SPEC> {
        UART3_W::new(self, 4)
    }
    #[doc = "Bit 6 - Low Power Comparator Reset."]
    #[inline(always)]
    #[must_use]
    pub fn lpcomp(&mut self) -> LPCOMP_W<RST_SPEC> {
        LPCOMP_W::new(self, 6)
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
#[doc = "Low Power Reset Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RST_SPEC;
impl crate::RegisterSpec for RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst::R`](R) reader structure"]
impl crate::Readable for RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rst::W`](W) writer structure"]
impl crate::Writable for RST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RST to value 0"]
impl crate::Resettable for RST_SPEC {
    const RESET_VALUE: u32 = 0;
}
