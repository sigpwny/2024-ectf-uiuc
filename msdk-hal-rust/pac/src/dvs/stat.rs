#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `DVS_STATE` reader - State machine state"]
pub type DVS_STATE_R = crate::FieldReader;
#[doc = "Field `DVS_STATE` writer - State machine state"]
pub type DVS_STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADJ_UP_ENA` reader - DVS Raising voltage"]
pub type ADJ_UP_ENA_R = crate::BitReader;
#[doc = "Field `ADJ_UP_ENA` writer - DVS Raising voltage"]
pub type ADJ_UP_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADJ_DWN_ENA` reader - DVS Lowering voltage"]
pub type ADJ_DWN_ENA_R = crate::BitReader;
#[doc = "Field `ADJ_DWN_ENA` writer - DVS Lowering voltage"]
pub type ADJ_DWN_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADJ_ACTIVE` reader - Adjustment to a Direct Voltage"]
pub type ADJ_ACTIVE_R = crate::BitReader;
#[doc = "Field `ADJ_ACTIVE` writer - Adjustment to a Direct Voltage"]
pub type ADJ_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR_TAP_OK` reader - Tap Enabled and the Tap is withing Hi/Low limits"]
pub type CTR_TAP_OK_R = crate::BitReader;
#[doc = "Field `CTR_TAP_OK` writer - Tap Enabled and the Tap is withing Hi/Low limits"]
pub type CTR_TAP_OK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR_TAP_SEL` reader - Status of selected center tap delay line detect output"]
pub type CTR_TAP_SEL_R = crate::BitReader;
#[doc = "Field `CTR_TAP_SEL` writer - Status of selected center tap delay line detect output"]
pub type CTR_TAP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOW_TRIP_DET` reader - Provides the current combined status of all selected Low Range delay lines"]
pub type SLOW_TRIP_DET_R = crate::BitReader;
#[doc = "Field `SLOW_TRIP_DET` writer - Provides the current combined status of all selected Low Range delay lines"]
pub type SLOW_TRIP_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAST_TRIP_DET` reader - Provides the current combined status of all selected High Range delay lines"]
pub type FAST_TRIP_DET_R = crate::BitReader;
#[doc = "Field `FAST_TRIP_DET` writer - Provides the current combined status of all selected High Range delay lines"]
pub type FAST_TRIP_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS_IN_RANGE` reader - Indicates if the power supply is in range"]
pub type PS_IN_RANGE_R = crate::BitReader;
#[doc = "Field `PS_IN_RANGE` writer - Indicates if the power supply is in range"]
pub type PS_IN_RANGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS_VCNTR` reader - Voltage Count value sent to the power supply"]
pub type PS_VCNTR_R = crate::FieldReader;
#[doc = "Field `PS_VCNTR` writer - Voltage Count value sent to the power supply"]
pub type PS_VCNTR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MON_DLY_OK` reader - Indicates the monitor delay count is at 0"]
pub type MON_DLY_OK_R = crate::BitReader;
#[doc = "Field `MON_DLY_OK` writer - Indicates the monitor delay count is at 0"]
pub type MON_DLY_OK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADJ_DLY_OK` reader - Indicates the adjustment delay count is at 0"]
pub type ADJ_DLY_OK_R = crate::BitReader;
#[doc = "Field `ADJ_DLY_OK` writer - Indicates the adjustment delay count is at 0"]
pub type ADJ_DLY_OK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LO_LIMIT_DET` reader - Power supply voltage counter is at low limit"]
pub type LO_LIMIT_DET_R = crate::BitReader;
#[doc = "Field `LO_LIMIT_DET` writer - Power supply voltage counter is at low limit"]
pub type LO_LIMIT_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HI_LIMIT_DET` reader - Power supply voltage counter is at high limit"]
pub type HI_LIMIT_DET_R = crate::BitReader;
#[doc = "Field `HI_LIMIT_DET` writer - Power supply voltage counter is at high limit"]
pub type HI_LIMIT_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VALID_TAP` reader - At least one delay line has been enabled"]
pub type VALID_TAP_R = crate::BitReader;
#[doc = "Field `VALID_TAP` writer - At least one delay line has been enabled"]
pub type VALID_TAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LIMIT_ERR` reader - Interrupt flag that indicates a voltage count is at/beyond manufacturer limits"]
pub type LIMIT_ERR_R = crate::BitReader;
#[doc = "Field `LIMIT_ERR` writer - Interrupt flag that indicates a voltage count is at/beyond manufacturer limits"]
pub type LIMIT_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RANGE_ERR` reader - Interrupt flag that indicates a tap has an invalid value"]
pub type RANGE_ERR_R = crate::BitReader;
#[doc = "Field `RANGE_ERR` writer - Interrupt flag that indicates a tap has an invalid value"]
pub type RANGE_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADJ_ERR` reader - Interrupt flag that indicates up and down adjustment requested simultaneously"]
pub type ADJ_ERR_R = crate::BitReader;
#[doc = "Field `ADJ_ERR` writer - Interrupt flag that indicates up and down adjustment requested simultaneously"]
pub type ADJ_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_SEL_ERR` reader - Indicates the ref select register bit is out of range"]
pub type REF_SEL_ERR_R = crate::BitReader;
#[doc = "Field `REF_SEL_ERR` writer - Indicates the ref select register bit is out of range"]
pub type REF_SEL_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FB_TO_ERR` reader - Interrupt flag that indicates a timeout while adjusting the voltage"]
pub type FB_TO_ERR_R = crate::BitReader;
#[doc = "Field `FB_TO_ERR` writer - Interrupt flag that indicates a timeout while adjusting the voltage"]
pub type FB_TO_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FB_TO_ERR_S` reader - Interrupt flag that mirror FB_TO_ERR and is write one clear"]
pub type FB_TO_ERR_S_R = crate::BitReader;
#[doc = "Field `FB_TO_ERR_S` writer - Interrupt flag that mirror FB_TO_ERR and is write one clear"]
pub type FB_TO_ERR_S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FC_LV_DET_INT` reader - Interrupt flag that indicates the power supply voltage requested is below the low threshold"]
pub type FC_LV_DET_INT_R = crate::BitReader;
#[doc = "Field `FC_LV_DET_INT` writer - Interrupt flag that indicates the power supply voltage requested is below the low threshold"]
pub type FC_LV_DET_INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FC_LV_DET_S` reader - Interrupt flag that mirrors FC_LV_DET_INT"]
pub type FC_LV_DET_S_R = crate::BitReader;
#[doc = "Field `FC_LV_DET_S` writer - Interrupt flag that mirrors FC_LV_DET_INT"]
pub type FC_LV_DET_S_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - State machine state"]
    #[inline(always)]
    pub fn dvs_state(&self) -> DVS_STATE_R {
        DVS_STATE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - DVS Raising voltage"]
    #[inline(always)]
    pub fn adj_up_ena(&self) -> ADJ_UP_ENA_R {
        ADJ_UP_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DVS Lowering voltage"]
    #[inline(always)]
    pub fn adj_dwn_ena(&self) -> ADJ_DWN_ENA_R {
        ADJ_DWN_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Adjustment to a Direct Voltage"]
    #[inline(always)]
    pub fn adj_active(&self) -> ADJ_ACTIVE_R {
        ADJ_ACTIVE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tap Enabled and the Tap is withing Hi/Low limits"]
    #[inline(always)]
    pub fn ctr_tap_ok(&self) -> CTR_TAP_OK_R {
        CTR_TAP_OK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Status of selected center tap delay line detect output"]
    #[inline(always)]
    pub fn ctr_tap_sel(&self) -> CTR_TAP_SEL_R {
        CTR_TAP_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Provides the current combined status of all selected Low Range delay lines"]
    #[inline(always)]
    pub fn slow_trip_det(&self) -> SLOW_TRIP_DET_R {
        SLOW_TRIP_DET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Provides the current combined status of all selected High Range delay lines"]
    #[inline(always)]
    pub fn fast_trip_det(&self) -> FAST_TRIP_DET_R {
        FAST_TRIP_DET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Indicates if the power supply is in range"]
    #[inline(always)]
    pub fn ps_in_range(&self) -> PS_IN_RANGE_R {
        PS_IN_RANGE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:18 - Voltage Count value sent to the power supply"]
    #[inline(always)]
    pub fn ps_vcntr(&self) -> PS_VCNTR_R {
        PS_VCNTR_R::new(((self.bits >> 12) & 0x7f) as u8)
    }
    #[doc = "Bit 19 - Indicates the monitor delay count is at 0"]
    #[inline(always)]
    pub fn mon_dly_ok(&self) -> MON_DLY_OK_R {
        MON_DLY_OK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Indicates the adjustment delay count is at 0"]
    #[inline(always)]
    pub fn adj_dly_ok(&self) -> ADJ_DLY_OK_R {
        ADJ_DLY_OK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Power supply voltage counter is at low limit"]
    #[inline(always)]
    pub fn lo_limit_det(&self) -> LO_LIMIT_DET_R {
        LO_LIMIT_DET_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Power supply voltage counter is at high limit"]
    #[inline(always)]
    pub fn hi_limit_det(&self) -> HI_LIMIT_DET_R {
        HI_LIMIT_DET_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - At least one delay line has been enabled"]
    #[inline(always)]
    pub fn valid_tap(&self) -> VALID_TAP_R {
        VALID_TAP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt flag that indicates a voltage count is at/beyond manufacturer limits"]
    #[inline(always)]
    pub fn limit_err(&self) -> LIMIT_ERR_R {
        LIMIT_ERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt flag that indicates a tap has an invalid value"]
    #[inline(always)]
    pub fn range_err(&self) -> RANGE_ERR_R {
        RANGE_ERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt flag that indicates up and down adjustment requested simultaneously"]
    #[inline(always)]
    pub fn adj_err(&self) -> ADJ_ERR_R {
        ADJ_ERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Indicates the ref select register bit is out of range"]
    #[inline(always)]
    pub fn ref_sel_err(&self) -> REF_SEL_ERR_R {
        REF_SEL_ERR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt flag that indicates a timeout while adjusting the voltage"]
    #[inline(always)]
    pub fn fb_to_err(&self) -> FB_TO_ERR_R {
        FB_TO_ERR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt flag that mirror FB_TO_ERR and is write one clear"]
    #[inline(always)]
    pub fn fb_to_err_s(&self) -> FB_TO_ERR_S_R {
        FB_TO_ERR_S_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt flag that indicates the power supply voltage requested is below the low threshold"]
    #[inline(always)]
    pub fn fc_lv_det_int(&self) -> FC_LV_DET_INT_R {
        FC_LV_DET_INT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt flag that mirrors FC_LV_DET_INT"]
    #[inline(always)]
    pub fn fc_lv_det_s(&self) -> FC_LV_DET_S_R {
        FC_LV_DET_S_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - State machine state"]
    #[inline(always)]
    #[must_use]
    pub fn dvs_state(&mut self) -> DVS_STATE_W<STAT_SPEC> {
        DVS_STATE_W::new(self, 0)
    }
    #[doc = "Bit 4 - DVS Raising voltage"]
    #[inline(always)]
    #[must_use]
    pub fn adj_up_ena(&mut self) -> ADJ_UP_ENA_W<STAT_SPEC> {
        ADJ_UP_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - DVS Lowering voltage"]
    #[inline(always)]
    #[must_use]
    pub fn adj_dwn_ena(&mut self) -> ADJ_DWN_ENA_W<STAT_SPEC> {
        ADJ_DWN_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - Adjustment to a Direct Voltage"]
    #[inline(always)]
    #[must_use]
    pub fn adj_active(&mut self) -> ADJ_ACTIVE_W<STAT_SPEC> {
        ADJ_ACTIVE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Tap Enabled and the Tap is withing Hi/Low limits"]
    #[inline(always)]
    #[must_use]
    pub fn ctr_tap_ok(&mut self) -> CTR_TAP_OK_W<STAT_SPEC> {
        CTR_TAP_OK_W::new(self, 7)
    }
    #[doc = "Bit 8 - Status of selected center tap delay line detect output"]
    #[inline(always)]
    #[must_use]
    pub fn ctr_tap_sel(&mut self) -> CTR_TAP_SEL_W<STAT_SPEC> {
        CTR_TAP_SEL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Provides the current combined status of all selected Low Range delay lines"]
    #[inline(always)]
    #[must_use]
    pub fn slow_trip_det(&mut self) -> SLOW_TRIP_DET_W<STAT_SPEC> {
        SLOW_TRIP_DET_W::new(self, 9)
    }
    #[doc = "Bit 10 - Provides the current combined status of all selected High Range delay lines"]
    #[inline(always)]
    #[must_use]
    pub fn fast_trip_det(&mut self) -> FAST_TRIP_DET_W<STAT_SPEC> {
        FAST_TRIP_DET_W::new(self, 10)
    }
    #[doc = "Bit 11 - Indicates if the power supply is in range"]
    #[inline(always)]
    #[must_use]
    pub fn ps_in_range(&mut self) -> PS_IN_RANGE_W<STAT_SPEC> {
        PS_IN_RANGE_W::new(self, 11)
    }
    #[doc = "Bits 12:18 - Voltage Count value sent to the power supply"]
    #[inline(always)]
    #[must_use]
    pub fn ps_vcntr(&mut self) -> PS_VCNTR_W<STAT_SPEC> {
        PS_VCNTR_W::new(self, 12)
    }
    #[doc = "Bit 19 - Indicates the monitor delay count is at 0"]
    #[inline(always)]
    #[must_use]
    pub fn mon_dly_ok(&mut self) -> MON_DLY_OK_W<STAT_SPEC> {
        MON_DLY_OK_W::new(self, 19)
    }
    #[doc = "Bit 20 - Indicates the adjustment delay count is at 0"]
    #[inline(always)]
    #[must_use]
    pub fn adj_dly_ok(&mut self) -> ADJ_DLY_OK_W<STAT_SPEC> {
        ADJ_DLY_OK_W::new(self, 20)
    }
    #[doc = "Bit 21 - Power supply voltage counter is at low limit"]
    #[inline(always)]
    #[must_use]
    pub fn lo_limit_det(&mut self) -> LO_LIMIT_DET_W<STAT_SPEC> {
        LO_LIMIT_DET_W::new(self, 21)
    }
    #[doc = "Bit 22 - Power supply voltage counter is at high limit"]
    #[inline(always)]
    #[must_use]
    pub fn hi_limit_det(&mut self) -> HI_LIMIT_DET_W<STAT_SPEC> {
        HI_LIMIT_DET_W::new(self, 22)
    }
    #[doc = "Bit 23 - At least one delay line has been enabled"]
    #[inline(always)]
    #[must_use]
    pub fn valid_tap(&mut self) -> VALID_TAP_W<STAT_SPEC> {
        VALID_TAP_W::new(self, 23)
    }
    #[doc = "Bit 24 - Interrupt flag that indicates a voltage count is at/beyond manufacturer limits"]
    #[inline(always)]
    #[must_use]
    pub fn limit_err(&mut self) -> LIMIT_ERR_W<STAT_SPEC> {
        LIMIT_ERR_W::new(self, 24)
    }
    #[doc = "Bit 25 - Interrupt flag that indicates a tap has an invalid value"]
    #[inline(always)]
    #[must_use]
    pub fn range_err(&mut self) -> RANGE_ERR_W<STAT_SPEC> {
        RANGE_ERR_W::new(self, 25)
    }
    #[doc = "Bit 26 - Interrupt flag that indicates up and down adjustment requested simultaneously"]
    #[inline(always)]
    #[must_use]
    pub fn adj_err(&mut self) -> ADJ_ERR_W<STAT_SPEC> {
        ADJ_ERR_W::new(self, 26)
    }
    #[doc = "Bit 27 - Indicates the ref select register bit is out of range"]
    #[inline(always)]
    #[must_use]
    pub fn ref_sel_err(&mut self) -> REF_SEL_ERR_W<STAT_SPEC> {
        REF_SEL_ERR_W::new(self, 27)
    }
    #[doc = "Bit 28 - Interrupt flag that indicates a timeout while adjusting the voltage"]
    #[inline(always)]
    #[must_use]
    pub fn fb_to_err(&mut self) -> FB_TO_ERR_W<STAT_SPEC> {
        FB_TO_ERR_W::new(self, 28)
    }
    #[doc = "Bit 29 - Interrupt flag that mirror FB_TO_ERR and is write one clear"]
    #[inline(always)]
    #[must_use]
    pub fn fb_to_err_s(&mut self) -> FB_TO_ERR_S_W<STAT_SPEC> {
        FB_TO_ERR_S_W::new(self, 29)
    }
    #[doc = "Bit 30 - Interrupt flag that indicates the power supply voltage requested is below the low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn fc_lv_det_int(&mut self) -> FC_LV_DET_INT_W<STAT_SPEC> {
        FC_LV_DET_INT_W::new(self, 30)
    }
    #[doc = "Bit 31 - Interrupt flag that mirrors FC_LV_DET_INT"]
    #[inline(always)]
    #[must_use]
    pub fn fc_lv_det_s(&mut self) -> FC_LV_DET_S_W<STAT_SPEC> {
        FC_LV_DET_S_W::new(self, 31)
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
#[doc = "Status Fields\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
