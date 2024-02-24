#[doc = "Register `RXCTRL0` reader"]
pub type R = crate::R<RXCTRL0_SPEC>;
#[doc = "Register `RXCTRL0` writer"]
pub type W = crate::W<RXCTRL0_SPEC>;
#[doc = "Field `DNR` reader - Do Not Respond."]
pub type DNR_R = crate::BitReader<DNR_A>;
#[doc = "Do Not Respond.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DNR_A {
    #[doc = "0: Always respond to address match."]
    RESPOND = 0,
    #[doc = "1: Do not respond to address match when RX_FIFO is not empty."]
    NOT_RESPOND_RX_FIFO_EMPTY = 1,
}
impl From<DNR_A> for bool {
    #[inline(always)]
    fn from(variant: DNR_A) -> Self {
        variant as u8 != 0
    }
}
impl DNR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DNR_A {
        match self.bits {
            false => DNR_A::RESPOND,
            true => DNR_A::NOT_RESPOND_RX_FIFO_EMPTY,
        }
    }
    #[doc = "Always respond to address match."]
    #[inline(always)]
    pub fn is_respond(&self) -> bool {
        *self == DNR_A::RESPOND
    }
    #[doc = "Do not respond to address match when RX_FIFO is not empty."]
    #[inline(always)]
    pub fn is_not_respond_rx_fifo_empty(&self) -> bool {
        *self == DNR_A::NOT_RESPOND_RX_FIFO_EMPTY
    }
}
#[doc = "Field `DNR` writer - Do Not Respond."]
pub type DNR_W<'a, REG> = crate::BitWriter<'a, REG, DNR_A>;
impl<'a, REG> DNR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Always respond to address match."]
    #[inline(always)]
    pub fn respond(self) -> &'a mut crate::W<REG> {
        self.variant(DNR_A::RESPOND)
    }
    #[doc = "Do not respond to address match when RX_FIFO is not empty."]
    #[inline(always)]
    pub fn not_respond_rx_fifo_empty(self) -> &'a mut crate::W<REG> {
        self.variant(DNR_A::NOT_RESPOND_RX_FIFO_EMPTY)
    }
}
#[doc = "Field `FLUSH` reader - Receive FIFO Flush. This bit is automatically cleared to 0 after the operation. Setting this bit to 1 will affect RX_FIFO status."]
pub type FLUSH_R = crate::BitReader<FLUSH_A>;
#[doc = "Receive FIFO Flush. This bit is automatically cleared to 0 after the operation. Setting this bit to 1 will affect RX_FIFO status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLUSH_A {
    #[doc = "0: FIFO not flushed."]
    NOT_FLUSHED = 0,
    #[doc = "1: Flush RX_FIFO."]
    FLUSH = 1,
}
impl From<FLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: FLUSH_A) -> Self {
        variant as u8 != 0
    }
}
impl FLUSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLUSH_A {
        match self.bits {
            false => FLUSH_A::NOT_FLUSHED,
            true => FLUSH_A::FLUSH,
        }
    }
    #[doc = "FIFO not flushed."]
    #[inline(always)]
    pub fn is_not_flushed(&self) -> bool {
        *self == FLUSH_A::NOT_FLUSHED
    }
    #[doc = "Flush RX_FIFO."]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == FLUSH_A::FLUSH
    }
}
#[doc = "Field `FLUSH` writer - Receive FIFO Flush. This bit is automatically cleared to 0 after the operation. Setting this bit to 1 will affect RX_FIFO status."]
pub type FLUSH_W<'a, REG> = crate::BitWriter<'a, REG, FLUSH_A>;
impl<'a, REG> FLUSH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFO not flushed."]
    #[inline(always)]
    pub fn not_flushed(self) -> &'a mut crate::W<REG> {
        self.variant(FLUSH_A::NOT_FLUSHED)
    }
    #[doc = "Flush RX_FIFO."]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(FLUSH_A::FLUSH)
    }
}
#[doc = "Field `THD_LVL` reader - Receive FIFO Threshold. These bits define the RX_FIFO interrupt threshold."]
pub type THD_LVL_R = crate::FieldReader;
#[doc = "Field `THD_LVL` writer - Receive FIFO Threshold. These bits define the RX_FIFO interrupt threshold."]
pub type THD_LVL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Do Not Respond."]
    #[inline(always)]
    pub fn dnr(&self) -> DNR_R {
        DNR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Receive FIFO Flush. This bit is automatically cleared to 0 after the operation. Setting this bit to 1 will affect RX_FIFO status."]
    #[inline(always)]
    pub fn flush(&self) -> FLUSH_R {
        FLUSH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Receive FIFO Threshold. These bits define the RX_FIFO interrupt threshold."]
    #[inline(always)]
    pub fn thd_lvl(&self) -> THD_LVL_R {
        THD_LVL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Do Not Respond."]
    #[inline(always)]
    #[must_use]
    pub fn dnr(&mut self) -> DNR_W<RXCTRL0_SPEC> {
        DNR_W::new(self, 0)
    }
    #[doc = "Bit 7 - Receive FIFO Flush. This bit is automatically cleared to 0 after the operation. Setting this bit to 1 will affect RX_FIFO status."]
    #[inline(always)]
    #[must_use]
    pub fn flush(&mut self) -> FLUSH_W<RXCTRL0_SPEC> {
        FLUSH_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Receive FIFO Threshold. These bits define the RX_FIFO interrupt threshold."]
    #[inline(always)]
    #[must_use]
    pub fn thd_lvl(&mut self) -> THD_LVL_W<RXCTRL0_SPEC> {
        THD_LVL_W::new(self, 8)
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
#[doc = "Receive Control Register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCTRL0_SPEC;
impl crate::RegisterSpec for RXCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxctrl0::R`](R) reader structure"]
impl crate::Readable for RXCTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxctrl0::W`](W) writer structure"]
impl crate::Writable for RXCTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXCTRL0 to value 0"]
impl crate::Resettable for RXCTRL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
