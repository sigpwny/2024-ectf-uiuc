#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `MON_ENA` reader - Enable the DVS monitoring circuit"]
pub type MON_ENA_R = crate::BitReader;
#[doc = "Field `MON_ENA` writer - Enable the DVS monitoring circuit"]
pub type MON_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADJ_ENA` reader - Enable the power supply adjustment based on measurements"]
pub type ADJ_ENA_R = crate::BitReader;
#[doc = "Field `ADJ_ENA` writer - Enable the power supply adjustment based on measurements"]
pub type ADJ_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS_FB_DIS` reader - Power Supply Feedback Disable"]
pub type PS_FB_DIS_R = crate::BitReader;
#[doc = "Field `PS_FB_DIS` writer - Power Supply Feedback Disable"]
pub type PS_FB_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_TAP_ENA` reader - Use the TAP Select for automatic adjustment or monitoring"]
pub type CTRL_TAP_ENA_R = crate::BitReader;
#[doc = "Field `CTRL_TAP_ENA` writer - Use the TAP Select for automatic adjustment or monitoring"]
pub type CTRL_TAP_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROP_DLY` reader - Additional delay to monitor lines"]
pub type PROP_DLY_R = crate::FieldReader;
#[doc = "Field `PROP_DLY` writer - Additional delay to monitor lines"]
pub type PROP_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MON_ONESHOT` reader - Measure delay once"]
pub type MON_ONESHOT_R = crate::BitReader;
#[doc = "Field `MON_ONESHOT` writer - Measure delay once"]
pub type MON_ONESHOT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GO_DIRECT` reader - Operate in automatic mode or move directly"]
pub type GO_DIRECT_R = crate::BitReader;
#[doc = "Field `GO_DIRECT` writer - Operate in automatic mode or move directly"]
pub type GO_DIRECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRECT_REG` reader - Step incrementally to target voltage"]
pub type DIRECT_REG_R = crate::BitReader;
#[doc = "Field `DIRECT_REG` writer - Step incrementally to target voltage"]
pub type DIRECT_REG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIME_ENA` reader - Include a delay line priming signal before monitoring"]
pub type PRIME_ENA_R = crate::BitReader;
#[doc = "Field `PRIME_ENA` writer - Include a delay line priming signal before monitoring"]
pub type PRIME_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LIMIT_IE` reader - Enable Limit Error Interrupt"]
pub type LIMIT_IE_R = crate::BitReader;
#[doc = "Field `LIMIT_IE` writer - Enable Limit Error Interrupt"]
pub type LIMIT_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RANGE_IE` reader - Enable Range Error Interrupt"]
pub type RANGE_IE_R = crate::BitReader;
#[doc = "Field `RANGE_IE` writer - Enable Range Error Interrupt"]
pub type RANGE_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADJ_IE` reader - Enable Adjustment Error Interrupt"]
pub type ADJ_IE_R = crate::BitReader;
#[doc = "Field `ADJ_IE` writer - Enable Adjustment Error Interrupt"]
pub type ADJ_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_SEL` reader - Select TAP used for voltage adjustment"]
pub type REF_SEL_R = crate::FieldReader;
#[doc = "Field `REF_SEL` writer - Select TAP used for voltage adjustment"]
pub type REF_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INC_VAL` reader - Step size to increment voltage when in automatic mode"]
pub type INC_VAL_R = crate::FieldReader;
#[doc = "Field `INC_VAL` writer - Step size to increment voltage when in automatic mode"]
pub type INC_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DVS_PS_APB_DIS` reader - Prevent the application code from adjusting Vcore"]
pub type DVS_PS_APB_DIS_R = crate::BitReader;
#[doc = "Field `DVS_PS_APB_DIS` writer - Prevent the application code from adjusting Vcore"]
pub type DVS_PS_APB_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DVS_HI_RANGE_ANY` reader - Any high range signal from a delay line will cause a voltage adjustment"]
pub type DVS_HI_RANGE_ANY_R = crate::BitReader;
#[doc = "Field `DVS_HI_RANGE_ANY` writer - Any high range signal from a delay line will cause a voltage adjustment"]
pub type DVS_HI_RANGE_ANY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FB_TO_IE` reader - Enable Voltage Adjustment Timeout Interrupt"]
pub type FB_TO_IE_R = crate::BitReader;
#[doc = "Field `FB_TO_IE` writer - Enable Voltage Adjustment Timeout Interrupt"]
pub type FB_TO_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FC_LV_IE` reader - Enable Low Voltage Interrupt"]
pub type FC_LV_IE_R = crate::BitReader;
#[doc = "Field `FC_LV_IE` writer - Enable Low Voltage Interrupt"]
pub type FC_LV_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD_ACK_ENA` reader - Prevent DVS from ack'ing a request to enter a low power mode until in the idle state"]
pub type PD_ACK_ENA_R = crate::BitReader;
#[doc = "Field `PD_ACK_ENA` writer - Prevent DVS from ack'ing a request to enter a low power mode until in the idle state"]
pub type PD_ACK_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADJ_ABORT` reader - Causes the DVS to enter the idle state immediately on a request to enter a low power mode"]
pub type ADJ_ABORT_R = crate::BitReader;
#[doc = "Field `ADJ_ABORT` writer - Causes the DVS to enter the idle state immediately on a request to enter a low power mode"]
pub type ADJ_ABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable the DVS monitoring circuit"]
    #[inline(always)]
    pub fn mon_ena(&self) -> MON_ENA_R {
        MON_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the power supply adjustment based on measurements"]
    #[inline(always)]
    pub fn adj_ena(&self) -> ADJ_ENA_R {
        ADJ_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power Supply Feedback Disable"]
    #[inline(always)]
    pub fn ps_fb_dis(&self) -> PS_FB_DIS_R {
        PS_FB_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Use the TAP Select for automatic adjustment or monitoring"]
    #[inline(always)]
    pub fn ctrl_tap_ena(&self) -> CTRL_TAP_ENA_R {
        CTRL_TAP_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Additional delay to monitor lines"]
    #[inline(always)]
    pub fn prop_dly(&self) -> PROP_DLY_R {
        PROP_DLY_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Measure delay once"]
    #[inline(always)]
    pub fn mon_oneshot(&self) -> MON_ONESHOT_R {
        MON_ONESHOT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Operate in automatic mode or move directly"]
    #[inline(always)]
    pub fn go_direct(&self) -> GO_DIRECT_R {
        GO_DIRECT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Step incrementally to target voltage"]
    #[inline(always)]
    pub fn direct_reg(&self) -> DIRECT_REG_R {
        DIRECT_REG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Include a delay line priming signal before monitoring"]
    #[inline(always)]
    pub fn prime_ena(&self) -> PRIME_ENA_R {
        PRIME_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Limit Error Interrupt"]
    #[inline(always)]
    pub fn limit_ie(&self) -> LIMIT_IE_R {
        LIMIT_IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Range Error Interrupt"]
    #[inline(always)]
    pub fn range_ie(&self) -> RANGE_IE_R {
        RANGE_IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Adjustment Error Interrupt"]
    #[inline(always)]
    pub fn adj_ie(&self) -> ADJ_IE_R {
        ADJ_IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - Select TAP used for voltage adjustment"]
    #[inline(always)]
    pub fn ref_sel(&self) -> REF_SEL_R {
        REF_SEL_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:19 - Step size to increment voltage when in automatic mode"]
    #[inline(always)]
    pub fn inc_val(&self) -> INC_VAL_R {
        INC_VAL_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - Prevent the application code from adjusting Vcore"]
    #[inline(always)]
    pub fn dvs_ps_apb_dis(&self) -> DVS_PS_APB_DIS_R {
        DVS_PS_APB_DIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Any high range signal from a delay line will cause a voltage adjustment"]
    #[inline(always)]
    pub fn dvs_hi_range_any(&self) -> DVS_HI_RANGE_ANY_R {
        DVS_HI_RANGE_ANY_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Voltage Adjustment Timeout Interrupt"]
    #[inline(always)]
    pub fn fb_to_ie(&self) -> FB_TO_IE_R {
        FB_TO_IE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Low Voltage Interrupt"]
    #[inline(always)]
    pub fn fc_lv_ie(&self) -> FC_LV_IE_R {
        FC_LV_IE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Prevent DVS from ack'ing a request to enter a low power mode until in the idle state"]
    #[inline(always)]
    pub fn pd_ack_ena(&self) -> PD_ACK_ENA_R {
        PD_ACK_ENA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Causes the DVS to enter the idle state immediately on a request to enter a low power mode"]
    #[inline(always)]
    pub fn adj_abort(&self) -> ADJ_ABORT_R {
        ADJ_ABORT_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the DVS monitoring circuit"]
    #[inline(always)]
    #[must_use]
    pub fn mon_ena(&mut self) -> MON_ENA_W<CTL_SPEC> {
        MON_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable the power supply adjustment based on measurements"]
    #[inline(always)]
    #[must_use]
    pub fn adj_ena(&mut self) -> ADJ_ENA_W<CTL_SPEC> {
        ADJ_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Power Supply Feedback Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ps_fb_dis(&mut self) -> PS_FB_DIS_W<CTL_SPEC> {
        PS_FB_DIS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Use the TAP Select for automatic adjustment or monitoring"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_tap_ena(&mut self) -> CTRL_TAP_ENA_W<CTL_SPEC> {
        CTRL_TAP_ENA_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Additional delay to monitor lines"]
    #[inline(always)]
    #[must_use]
    pub fn prop_dly(&mut self) -> PROP_DLY_W<CTL_SPEC> {
        PROP_DLY_W::new(self, 4)
    }
    #[doc = "Bit 6 - Measure delay once"]
    #[inline(always)]
    #[must_use]
    pub fn mon_oneshot(&mut self) -> MON_ONESHOT_W<CTL_SPEC> {
        MON_ONESHOT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Operate in automatic mode or move directly"]
    #[inline(always)]
    #[must_use]
    pub fn go_direct(&mut self) -> GO_DIRECT_W<CTL_SPEC> {
        GO_DIRECT_W::new(self, 7)
    }
    #[doc = "Bit 8 - Step incrementally to target voltage"]
    #[inline(always)]
    #[must_use]
    pub fn direct_reg(&mut self) -> DIRECT_REG_W<CTL_SPEC> {
        DIRECT_REG_W::new(self, 8)
    }
    #[doc = "Bit 9 - Include a delay line priming signal before monitoring"]
    #[inline(always)]
    #[must_use]
    pub fn prime_ena(&mut self) -> PRIME_ENA_W<CTL_SPEC> {
        PRIME_ENA_W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Limit Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn limit_ie(&mut self) -> LIMIT_IE_W<CTL_SPEC> {
        LIMIT_IE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Range Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn range_ie(&mut self) -> RANGE_IE_W<CTL_SPEC> {
        RANGE_IE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Adjustment Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn adj_ie(&mut self) -> ADJ_IE_W<CTL_SPEC> {
        ADJ_IE_W::new(self, 12)
    }
    #[doc = "Bits 13:16 - Select TAP used for voltage adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn ref_sel(&mut self) -> REF_SEL_W<CTL_SPEC> {
        REF_SEL_W::new(self, 13)
    }
    #[doc = "Bits 17:19 - Step size to increment voltage when in automatic mode"]
    #[inline(always)]
    #[must_use]
    pub fn inc_val(&mut self) -> INC_VAL_W<CTL_SPEC> {
        INC_VAL_W::new(self, 17)
    }
    #[doc = "Bit 20 - Prevent the application code from adjusting Vcore"]
    #[inline(always)]
    #[must_use]
    pub fn dvs_ps_apb_dis(&mut self) -> DVS_PS_APB_DIS_W<CTL_SPEC> {
        DVS_PS_APB_DIS_W::new(self, 20)
    }
    #[doc = "Bit 21 - Any high range signal from a delay line will cause a voltage adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn dvs_hi_range_any(&mut self) -> DVS_HI_RANGE_ANY_W<CTL_SPEC> {
        DVS_HI_RANGE_ANY_W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Voltage Adjustment Timeout Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fb_to_ie(&mut self) -> FB_TO_IE_W<CTL_SPEC> {
        FB_TO_IE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Low Voltage Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fc_lv_ie(&mut self) -> FC_LV_IE_W<CTL_SPEC> {
        FC_LV_IE_W::new(self, 23)
    }
    #[doc = "Bit 24 - Prevent DVS from ack'ing a request to enter a low power mode until in the idle state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_ack_ena(&mut self) -> PD_ACK_ENA_W<CTL_SPEC> {
        PD_ACK_ENA_W::new(self, 24)
    }
    #[doc = "Bit 25 - Causes the DVS to enter the idle state immediately on a request to enter a low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn adj_abort(&mut self) -> ADJ_ABORT_W<CTL_SPEC> {
        ADJ_ABORT_W::new(self, 25)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
