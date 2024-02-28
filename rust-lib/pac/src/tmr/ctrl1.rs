#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `CLKSEL_A` reader - Timer Clock Select for Timer A"]
pub type CLKSEL_A_R = crate::FieldReader;
#[doc = "Field `CLKSEL_A` writer - Timer Clock Select for Timer A"]
pub type CLKSEL_A_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLKEN_A` reader - Timer A Enable Status"]
pub type CLKEN_A_R = crate::BitReader;
#[doc = "Field `CLKEN_A` writer - Timer A Enable Status"]
pub type CLKEN_A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKRDY_A` reader - CLK_TMR Ready Flag for Timer A"]
pub type CLKRDY_A_R = crate::BitReader;
#[doc = "Field `CLKRDY_A` writer - CLK_TMR Ready Flag for Timer A"]
pub type CLKRDY_A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENT_SEL_A` reader - Event Select for Timer A"]
pub type EVENT_SEL_A_R = crate::FieldReader;
#[doc = "Field `EVENT_SEL_A` writer - Event Select for Timer A"]
pub type EVENT_SEL_A_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NEGTRIG_A` reader - Negative Edge Trigger for Event for Timer A"]
pub type NEGTRIG_A_R = crate::BitReader;
#[doc = "Field `NEGTRIG_A` writer - Negative Edge Trigger for Event for Timer A"]
pub type NEGTRIG_A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE_A` reader - Interrupt Enable for Timer A"]
pub type IE_A_R = crate::BitReader;
#[doc = "Field `IE_A` writer - Interrupt Enable for Timer A"]
pub type IE_A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPEVENT_SEL_A` reader - Capture Event Select for Timer A"]
pub type CAPEVENT_SEL_A_R = crate::FieldReader;
#[doc = "Field `CAPEVENT_SEL_A` writer - Capture Event Select for Timer A"]
pub type CAPEVENT_SEL_A_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SW_CAPEVENT_A` reader - Software Capture Event for Timer A"]
pub type SW_CAPEVENT_A_R = crate::BitReader;
#[doc = "Field `SW_CAPEVENT_A` writer - Software Capture Event for Timer A"]
pub type SW_CAPEVENT_A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WE_A` reader - Wake-Up Enable for Timer A"]
pub type WE_A_R = crate::BitReader;
#[doc = "Field `WE_A` writer - Wake-Up Enable for Timer A"]
pub type WE_A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEN_A` reader - OUT_OE_O Enable for Modes 0, 1,and 5 for Timer A"]
pub type OUTEN_A_R = crate::BitReader;
#[doc = "Field `OUTEN_A` writer - OUT_OE_O Enable for Modes 0, 1,and 5 for Timer A"]
pub type OUTEN_A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTBEN_A` reader - PWM_CKB_EN_O Enable for Modes other than Mode 3 for Timer A"]
pub type OUTBEN_A_R = crate::BitReader;
#[doc = "Field `OUTBEN_A` writer - PWM_CKB_EN_O Enable for Modes other than Mode 3 for Timer A"]
pub type OUTBEN_A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSEL_B` reader - Timer Clock Select for Timer B"]
pub type CLKSEL_B_R = crate::FieldReader;
#[doc = "Field `CLKSEL_B` writer - Timer Clock Select for Timer B"]
pub type CLKSEL_B_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLKEN_B` reader - Timer B Enable Status"]
pub type CLKEN_B_R = crate::BitReader;
#[doc = "Field `CLKEN_B` writer - Timer B Enable Status"]
pub type CLKEN_B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKRDY_B` reader - CLK_TMR Ready Flag for Timer B"]
pub type CLKRDY_B_R = crate::BitReader;
#[doc = "Field `CLKRDY_B` writer - CLK_TMR Ready Flag for Timer B"]
pub type CLKRDY_B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENT_SEL_B` reader - Event Select for Timer B"]
pub type EVENT_SEL_B_R = crate::FieldReader;
#[doc = "Field `EVENT_SEL_B` writer - Event Select for Timer B"]
pub type EVENT_SEL_B_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NEGTRIG_B` reader - Negative Edge Trigger for Event for Timer B"]
pub type NEGTRIG_B_R = crate::BitReader;
#[doc = "Field `NEGTRIG_B` writer - Negative Edge Trigger for Event for Timer B"]
pub type NEGTRIG_B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE_B` reader - Interrupt Enable for Timer B"]
pub type IE_B_R = crate::BitReader;
#[doc = "Field `IE_B` writer - Interrupt Enable for Timer B"]
pub type IE_B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPEVENT_SEL_B` reader - Capture Event Select for Timer B"]
pub type CAPEVENT_SEL_B_R = crate::FieldReader;
#[doc = "Field `CAPEVENT_SEL_B` writer - Capture Event Select for Timer B"]
pub type CAPEVENT_SEL_B_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SW_CAPEVENT_B` reader - Software Capture Event for Timer B"]
pub type SW_CAPEVENT_B_R = crate::BitReader;
#[doc = "Field `SW_CAPEVENT_B` writer - Software Capture Event for Timer B"]
pub type SW_CAPEVENT_B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WE_B` reader - Wake-Up Enable for Timer B"]
pub type WE_B_R = crate::BitReader;
#[doc = "Field `WE_B` writer - Wake-Up Enable for Timer B"]
pub type WE_B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CASCADE` reader - Cascade two 16-bit timers into one 32-bit timer. Only available when C_TMR16=0 adn C_DUALTMR16=1."]
pub type CASCADE_R = crate::BitReader;
#[doc = "Field `CASCADE` writer - Cascade two 16-bit timers into one 32-bit timer. Only available when C_TMR16=0 adn C_DUALTMR16=1."]
pub type CASCADE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Timer Clock Select for Timer A"]
    #[inline(always)]
    pub fn clksel_a(&self) -> CLKSEL_A_R {
        CLKSEL_A_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Timer A Enable Status"]
    #[inline(always)]
    pub fn clken_a(&self) -> CLKEN_A_R {
        CLKEN_A_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CLK_TMR Ready Flag for Timer A"]
    #[inline(always)]
    pub fn clkrdy_a(&self) -> CLKRDY_A_R {
        CLKRDY_A_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Event Select for Timer A"]
    #[inline(always)]
    pub fn event_sel_a(&self) -> EVENT_SEL_A_R {
        EVENT_SEL_A_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Negative Edge Trigger for Event for Timer A"]
    #[inline(always)]
    pub fn negtrig_a(&self) -> NEGTRIG_A_R {
        NEGTRIG_A_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Enable for Timer A"]
    #[inline(always)]
    pub fn ie_a(&self) -> IE_A_R {
        IE_A_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Capture Event Select for Timer A"]
    #[inline(always)]
    pub fn capevent_sel_a(&self) -> CAPEVENT_SEL_A_R {
        CAPEVENT_SEL_A_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Software Capture Event for Timer A"]
    #[inline(always)]
    pub fn sw_capevent_a(&self) -> SW_CAPEVENT_A_R {
        SW_CAPEVENT_A_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wake-Up Enable for Timer A"]
    #[inline(always)]
    pub fn we_a(&self) -> WE_A_R {
        WE_A_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OUT_OE_O Enable for Modes 0, 1,and 5 for Timer A"]
    #[inline(always)]
    pub fn outen_a(&self) -> OUTEN_A_R {
        OUTEN_A_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PWM_CKB_EN_O Enable for Modes other than Mode 3 for Timer A"]
    #[inline(always)]
    pub fn outben_a(&self) -> OUTBEN_A_R {
        OUTBEN_A_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Timer Clock Select for Timer B"]
    #[inline(always)]
    pub fn clksel_b(&self) -> CLKSEL_B_R {
        CLKSEL_B_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Timer B Enable Status"]
    #[inline(always)]
    pub fn clken_b(&self) -> CLKEN_B_R {
        CLKEN_B_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CLK_TMR Ready Flag for Timer B"]
    #[inline(always)]
    pub fn clkrdy_b(&self) -> CLKRDY_B_R {
        CLKRDY_B_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Event Select for Timer B"]
    #[inline(always)]
    pub fn event_sel_b(&self) -> EVENT_SEL_B_R {
        EVENT_SEL_B_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Negative Edge Trigger for Event for Timer B"]
    #[inline(always)]
    pub fn negtrig_b(&self) -> NEGTRIG_B_R {
        NEGTRIG_B_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Enable for Timer B"]
    #[inline(always)]
    pub fn ie_b(&self) -> IE_B_R {
        IE_B_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Capture Event Select for Timer B"]
    #[inline(always)]
    pub fn capevent_sel_b(&self) -> CAPEVENT_SEL_B_R {
        CAPEVENT_SEL_B_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Software Capture Event for Timer B"]
    #[inline(always)]
    pub fn sw_capevent_b(&self) -> SW_CAPEVENT_B_R {
        SW_CAPEVENT_B_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wake-Up Enable for Timer B"]
    #[inline(always)]
    pub fn we_b(&self) -> WE_B_R {
        WE_B_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Cascade two 16-bit timers into one 32-bit timer. Only available when C_TMR16=0 adn C_DUALTMR16=1."]
    #[inline(always)]
    pub fn cascade(&self) -> CASCADE_R {
        CASCADE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer Clock Select for Timer A"]
    #[inline(always)]
    #[must_use]
    pub fn clksel_a(&mut self) -> CLKSEL_A_W<CTRL1_SPEC> {
        CLKSEL_A_W::new(self, 0)
    }
    #[doc = "Bit 2 - Timer A Enable Status"]
    #[inline(always)]
    #[must_use]
    pub fn clken_a(&mut self) -> CLKEN_A_W<CTRL1_SPEC> {
        CLKEN_A_W::new(self, 2)
    }
    #[doc = "Bit 3 - CLK_TMR Ready Flag for Timer A"]
    #[inline(always)]
    #[must_use]
    pub fn clkrdy_a(&mut self) -> CLKRDY_A_W<CTRL1_SPEC> {
        CLKRDY_A_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Event Select for Timer A"]
    #[inline(always)]
    #[must_use]
    pub fn event_sel_a(&mut self) -> EVENT_SEL_A_W<CTRL1_SPEC> {
        EVENT_SEL_A_W::new(self, 4)
    }
    #[doc = "Bit 7 - Negative Edge Trigger for Event for Timer A"]
    #[inline(always)]
    #[must_use]
    pub fn negtrig_a(&mut self) -> NEGTRIG_A_W<CTRL1_SPEC> {
        NEGTRIG_A_W::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt Enable for Timer A"]
    #[inline(always)]
    #[must_use]
    pub fn ie_a(&mut self) -> IE_A_W<CTRL1_SPEC> {
        IE_A_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - Capture Event Select for Timer A"]
    #[inline(always)]
    #[must_use]
    pub fn capevent_sel_a(&mut self) -> CAPEVENT_SEL_A_W<CTRL1_SPEC> {
        CAPEVENT_SEL_A_W::new(self, 9)
    }
    #[doc = "Bit 11 - Software Capture Event for Timer A"]
    #[inline(always)]
    #[must_use]
    pub fn sw_capevent_a(&mut self) -> SW_CAPEVENT_A_W<CTRL1_SPEC> {
        SW_CAPEVENT_A_W::new(self, 11)
    }
    #[doc = "Bit 12 - Wake-Up Enable for Timer A"]
    #[inline(always)]
    #[must_use]
    pub fn we_a(&mut self) -> WE_A_W<CTRL1_SPEC> {
        WE_A_W::new(self, 12)
    }
    #[doc = "Bit 13 - OUT_OE_O Enable for Modes 0, 1,and 5 for Timer A"]
    #[inline(always)]
    #[must_use]
    pub fn outen_a(&mut self) -> OUTEN_A_W<CTRL1_SPEC> {
        OUTEN_A_W::new(self, 13)
    }
    #[doc = "Bit 14 - PWM_CKB_EN_O Enable for Modes other than Mode 3 for Timer A"]
    #[inline(always)]
    #[must_use]
    pub fn outben_a(&mut self) -> OUTBEN_A_W<CTRL1_SPEC> {
        OUTBEN_A_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Timer Clock Select for Timer B"]
    #[inline(always)]
    #[must_use]
    pub fn clksel_b(&mut self) -> CLKSEL_B_W<CTRL1_SPEC> {
        CLKSEL_B_W::new(self, 16)
    }
    #[doc = "Bit 18 - Timer B Enable Status"]
    #[inline(always)]
    #[must_use]
    pub fn clken_b(&mut self) -> CLKEN_B_W<CTRL1_SPEC> {
        CLKEN_B_W::new(self, 18)
    }
    #[doc = "Bit 19 - CLK_TMR Ready Flag for Timer B"]
    #[inline(always)]
    #[must_use]
    pub fn clkrdy_b(&mut self) -> CLKRDY_B_W<CTRL1_SPEC> {
        CLKRDY_B_W::new(self, 19)
    }
    #[doc = "Bits 20:22 - Event Select for Timer B"]
    #[inline(always)]
    #[must_use]
    pub fn event_sel_b(&mut self) -> EVENT_SEL_B_W<CTRL1_SPEC> {
        EVENT_SEL_B_W::new(self, 20)
    }
    #[doc = "Bit 23 - Negative Edge Trigger for Event for Timer B"]
    #[inline(always)]
    #[must_use]
    pub fn negtrig_b(&mut self) -> NEGTRIG_B_W<CTRL1_SPEC> {
        NEGTRIG_B_W::new(self, 23)
    }
    #[doc = "Bit 24 - Interrupt Enable for Timer B"]
    #[inline(always)]
    #[must_use]
    pub fn ie_b(&mut self) -> IE_B_W<CTRL1_SPEC> {
        IE_B_W::new(self, 24)
    }
    #[doc = "Bits 25:26 - Capture Event Select for Timer B"]
    #[inline(always)]
    #[must_use]
    pub fn capevent_sel_b(&mut self) -> CAPEVENT_SEL_B_W<CTRL1_SPEC> {
        CAPEVENT_SEL_B_W::new(self, 25)
    }
    #[doc = "Bit 27 - Software Capture Event for Timer B"]
    #[inline(always)]
    #[must_use]
    pub fn sw_capevent_b(&mut self) -> SW_CAPEVENT_B_W<CTRL1_SPEC> {
        SW_CAPEVENT_B_W::new(self, 27)
    }
    #[doc = "Bit 28 - Wake-Up Enable for Timer B"]
    #[inline(always)]
    #[must_use]
    pub fn we_b(&mut self) -> WE_B_W<CTRL1_SPEC> {
        WE_B_W::new(self, 28)
    }
    #[doc = "Bit 31 - Cascade two 16-bit timers into one 32-bit timer. Only available when C_TMR16=0 adn C_DUALTMR16=1."]
    #[inline(always)]
    #[must_use]
    pub fn cascade(&mut self) -> CASCADE_W<CTRL1_SPEC> {
        CASCADE_W::new(self, 31)
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
#[doc = "Timer Configuration Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
