#[doc = "Register `INTFL` reader"]
pub type R = crate::R<INTFL_SPEC>;
#[doc = "Register `INTFL` writer"]
pub type W = crate::W<INTFL_SPEC>;
#[doc = "Field `ow_reset_done` reader - OW Reset Sequence Completed."]
pub type OW_RESET_DONE_R = crate::BitReader;
#[doc = "Field `ow_reset_done` writer - OW Reset Sequence Completed."]
pub type OW_RESET_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_data_empty` reader - TX Data Empty Interrupt Flag."]
pub type TX_DATA_EMPTY_R = crate::BitReader;
#[doc = "Field `tx_data_empty` writer - TX Data Empty Interrupt Flag."]
pub type TX_DATA_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_data_ready` reader - RX Data Ready Interrupt Flag"]
pub type RX_DATA_READY_R = crate::BitReader;
#[doc = "Field `rx_data_ready` writer - RX Data Ready Interrupt Flag"]
pub type RX_DATA_READY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `line_short` reader - OW Line Short Detected Interrupt Flag."]
pub type LINE_SHORT_R = crate::BitReader;
#[doc = "Field `line_short` writer - OW Line Short Detected Interrupt Flag."]
pub type LINE_SHORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `line_low` reader - OW Line Low Detected Interrupt Flag."]
pub type LINE_LOW_R = crate::BitReader;
#[doc = "Field `line_low` writer - OW Line Low Detected Interrupt Flag."]
pub type LINE_LOW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OW Reset Sequence Completed."]
    #[inline(always)]
    pub fn ow_reset_done(&self) -> OW_RESET_DONE_R {
        OW_RESET_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Data Empty Interrupt Flag."]
    #[inline(always)]
    pub fn tx_data_empty(&self) -> TX_DATA_EMPTY_R {
        TX_DATA_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Data Ready Interrupt Flag"]
    #[inline(always)]
    pub fn rx_data_ready(&self) -> RX_DATA_READY_R {
        RX_DATA_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OW Line Short Detected Interrupt Flag."]
    #[inline(always)]
    pub fn line_short(&self) -> LINE_SHORT_R {
        LINE_SHORT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OW Line Low Detected Interrupt Flag."]
    #[inline(always)]
    pub fn line_low(&self) -> LINE_LOW_R {
        LINE_LOW_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OW Reset Sequence Completed."]
    #[inline(always)]
    #[must_use]
    pub fn ow_reset_done(&mut self) -> OW_RESET_DONE_W<INTFL_SPEC> {
        OW_RESET_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - TX Data Empty Interrupt Flag."]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_empty(&mut self) -> TX_DATA_EMPTY_W<INTFL_SPEC> {
        TX_DATA_EMPTY_W::new(self, 1)
    }
    #[doc = "Bit 2 - RX Data Ready Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_ready(&mut self) -> RX_DATA_READY_W<INTFL_SPEC> {
        RX_DATA_READY_W::new(self, 2)
    }
    #[doc = "Bit 3 - OW Line Short Detected Interrupt Flag."]
    #[inline(always)]
    #[must_use]
    pub fn line_short(&mut self) -> LINE_SHORT_W<INTFL_SPEC> {
        LINE_SHORT_W::new(self, 3)
    }
    #[doc = "Bit 4 - OW Line Low Detected Interrupt Flag."]
    #[inline(always)]
    #[must_use]
    pub fn line_low(&mut self) -> LINE_LOW_W<INTFL_SPEC> {
        LINE_LOW_W::new(self, 4)
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
#[doc = "1-Wire Master Interrupt Flags.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFL_SPEC;
impl crate::RegisterSpec for INTFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intfl::R`](R) reader structure"]
impl crate::Readable for INTFL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intfl::W`](W) writer structure"]
impl crate::Writable for INTFL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTFL to value 0"]
impl crate::Resettable for INTFL_SPEC {
    const RESET_VALUE: u32 = 0;
}
