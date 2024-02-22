#[doc = "Register `INTFL` reader"]
pub type R = crate::R<INTFL_SPEC>;
#[doc = "Register `INTFL` writer"]
pub type W = crate::W<INTFL_SPEC>;
#[doc = "Field `RX_OV_CH0` reader - Status for RX FIFO Overrun interrupt."]
pub type RX_OV_CH0_R = crate::BitReader;
#[doc = "Field `RX_OV_CH0` writer - Status for RX FIFO Overrun interrupt."]
pub type RX_OV_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_THD_CH0` reader - Status for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
pub type RX_THD_CH0_R = crate::BitReader;
#[doc = "Field `RX_THD_CH0` writer - Status for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
pub type RX_THD_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_OB_CH0` reader - Status for interrupt when TX FIFO has only one byte remaining."]
pub type TX_OB_CH0_R = crate::BitReader;
#[doc = "Field `TX_OB_CH0` writer - Status for interrupt when TX FIFO has only one byte remaining."]
pub type TX_OB_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_HE_CH0` reader - Status for interrupt when TX FIFO is half empty."]
pub type TX_HE_CH0_R = crate::BitReader;
#[doc = "Field `TX_HE_CH0` writer - Status for interrupt when TX FIFO is half empty."]
pub type TX_HE_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Status for RX FIFO Overrun interrupt."]
    #[inline(always)]
    pub fn rx_ov_ch0(&self) -> RX_OV_CH0_R {
        RX_OV_CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
    #[inline(always)]
    pub fn rx_thd_ch0(&self) -> RX_THD_CH0_R {
        RX_THD_CH0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status for interrupt when TX FIFO has only one byte remaining."]
    #[inline(always)]
    pub fn tx_ob_ch0(&self) -> TX_OB_CH0_R {
        TX_OB_CH0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status for interrupt when TX FIFO is half empty."]
    #[inline(always)]
    pub fn tx_he_ch0(&self) -> TX_HE_CH0_R {
        TX_HE_CH0_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Status for RX FIFO Overrun interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ov_ch0(&mut self) -> RX_OV_CH0_W<INTFL_SPEC> {
        RX_OV_CH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Status for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
    #[inline(always)]
    #[must_use]
    pub fn rx_thd_ch0(&mut self) -> RX_THD_CH0_W<INTFL_SPEC> {
        RX_THD_CH0_W::new(self, 1)
    }
    #[doc = "Bit 2 - Status for interrupt when TX FIFO has only one byte remaining."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ob_ch0(&mut self) -> TX_OB_CH0_W<INTFL_SPEC> {
        TX_OB_CH0_W::new(self, 2)
    }
    #[doc = "Bit 3 - Status for interrupt when TX FIFO is half empty."]
    #[inline(always)]
    #[must_use]
    pub fn tx_he_ch0(&mut self) -> TX_HE_CH0_W<INTFL_SPEC> {
        TX_HE_CH0_W::new(self, 3)
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
#[doc = "ISR Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
