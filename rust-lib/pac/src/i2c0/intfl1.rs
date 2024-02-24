#[doc = "Register `INTFL1` reader"]
pub type R = crate::R<INTFL1_SPEC>;
#[doc = "Register `INTFL1` writer"]
pub type W = crate::W<INTFL1_SPEC>;
#[doc = "Field `RX_OV` reader - Receiver Overflow Interrupt. When operating as a slave receiver, this bit is set when you reach the first data bit and the RX FIFO and shift register are both full."]
pub type RX_OV_R = crate::BitReader<RX_OV_A>;
#[doc = "Receiver Overflow Interrupt. When operating as a slave receiver, this bit is set when you reach the first data bit and the RX FIFO and shift register are both full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_OV_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<RX_OV_A> for bool {
    #[inline(always)]
    fn from(variant: RX_OV_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_OV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_OV_A {
        match self.bits {
            false => RX_OV_A::INACTIVE,
            true => RX_OV_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == RX_OV_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RX_OV_A::PENDING
    }
}
#[doc = "Field `RX_OV` writer - Receiver Overflow Interrupt. When operating as a slave receiver, this bit is set when you reach the first data bit and the RX FIFO and shift register are both full."]
pub type RX_OV_W<'a, REG> = crate::BitWriter<'a, REG, RX_OV_A>;
impl<'a, REG> RX_OV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(RX_OV_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(RX_OV_A::PENDING)
    }
}
#[doc = "Field `TX_UN` reader - Transmit Underflow Interrupt. When operating as a slave transmitter, this bit is set when you reach the first data bit and the TX FIFO is empty and the master is still asking for more data (i.e the master hasn't sent a NACK yet)."]
pub type TX_UN_R = crate::BitReader<TX_UN_A>;
#[doc = "Transmit Underflow Interrupt. When operating as a slave transmitter, this bit is set when you reach the first data bit and the TX FIFO is empty and the master is still asking for more data (i.e the master hasn't sent a NACK yet).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_UN_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<TX_UN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_UN_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_UN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_UN_A {
        match self.bits {
            false => TX_UN_A::INACTIVE,
            true => TX_UN_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == TX_UN_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TX_UN_A::PENDING
    }
}
#[doc = "Field `TX_UN` writer - Transmit Underflow Interrupt. When operating as a slave transmitter, this bit is set when you reach the first data bit and the TX FIFO is empty and the master is still asking for more data (i.e the master hasn't sent a NACK yet)."]
pub type TX_UN_W<'a, REG> = crate::BitWriter<'a, REG, TX_UN_A>;
impl<'a, REG> TX_UN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(TX_UN_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(TX_UN_A::PENDING)
    }
}
#[doc = "Field `START` reader - START Condition Status Flag."]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - START Condition Status Flag."]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receiver Overflow Interrupt. When operating as a slave receiver, this bit is set when you reach the first data bit and the RX FIFO and shift register are both full."]
    #[inline(always)]
    pub fn rx_ov(&self) -> RX_OV_R {
        RX_OV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Underflow Interrupt. When operating as a slave transmitter, this bit is set when you reach the first data bit and the TX FIFO is empty and the master is still asking for more data (i.e the master hasn't sent a NACK yet)."]
    #[inline(always)]
    pub fn tx_un(&self) -> TX_UN_R {
        TX_UN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - START Condition Status Flag."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Overflow Interrupt. When operating as a slave receiver, this bit is set when you reach the first data bit and the RX FIFO and shift register are both full."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ov(&mut self) -> RX_OV_W<INTFL1_SPEC> {
        RX_OV_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Underflow Interrupt. When operating as a slave transmitter, this bit is set when you reach the first data bit and the TX FIFO is empty and the master is still asking for more data (i.e the master hasn't sent a NACK yet)."]
    #[inline(always)]
    #[must_use]
    pub fn tx_un(&mut self) -> TX_UN_W<INTFL1_SPEC> {
        TX_UN_W::new(self, 1)
    }
    #[doc = "Bit 2 - START Condition Status Flag."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<INTFL1_SPEC> {
        START_W::new(self, 2)
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
#[doc = "Interrupt Status Register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFL1_SPEC;
impl crate::RegisterSpec for INTFL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intfl1::R`](R) reader structure"]
impl crate::Readable for INTFL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intfl1::W`](W) writer structure"]
impl crate::Writable for INTFL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTFL1 to value 0"]
impl crate::Resettable for INTFL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
