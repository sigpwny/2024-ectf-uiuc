#[doc = "Register `RXCTRL1` reader"]
pub type R = crate::R<RXCTRL1_SPEC>;
#[doc = "Register `RXCTRL1` writer"]
pub type W = crate::W<RXCTRL1_SPEC>;
#[doc = "Field `CNT` reader - Receive Count Bits. These bits define the number of bytes to be received in a transaction, except for the case RXCNT = 0. RXCNT = 0 means 256 bytes to be received in a transaction."]
pub type CNT_R = crate::FieldReader;
#[doc = "Field `CNT` writer - Receive Count Bits. These bits define the number of bytes to be received in a transaction, except for the case RXCNT = 0. RXCNT = 0 means 256 bytes to be received in a transaction."]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LVL` reader - Receive FIFO Count. These bits reflect the number of byte in the RX_FIFO. These bits are flushed when I2CEN = 0."]
pub type LVL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive Count Bits. These bits define the number of bytes to be received in a transaction, except for the case RXCNT = 0. RXCNT = 0 means 256 bytes to be received in a transaction."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Receive FIFO Count. These bits reflect the number of byte in the RX_FIFO. These bits are flushed when I2CEN = 0."]
    #[inline(always)]
    pub fn lvl(&self) -> LVL_R {
        LVL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Count Bits. These bits define the number of bytes to be received in a transaction, except for the case RXCNT = 0. RXCNT = 0 means 256 bytes to be received in a transaction."]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<RXCTRL1_SPEC> {
        CNT_W::new(self, 0)
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
#[doc = "Receive Control Register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCTRL1_SPEC;
impl crate::RegisterSpec for RXCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxctrl1::R`](R) reader structure"]
impl crate::Readable for RXCTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxctrl1::W`](W) writer structure"]
impl crate::Writable for RXCTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXCTRL1 to value 0"]
impl crate::Resettable for RXCTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
