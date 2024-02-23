#[doc = "Register `TXCTRL1` reader"]
pub type R = crate::R<TXCTRL1_SPEC>;
#[doc = "Register `TXCTRL1` writer"]
pub type W = crate::W<TXCTRL1_SPEC>;
#[doc = "Field `PRELOAD_RDY` reader - Transmit FIFO Preload Ready."]
pub type PRELOAD_RDY_R = crate::BitReader;
#[doc = "Field `PRELOAD_RDY` writer - Transmit FIFO Preload Ready."]
pub type PRELOAD_RDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVL` reader - Transmit FIFO Count. These bits reflect the number of bytes in the TX_FIFO."]
pub type LVL_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Transmit FIFO Preload Ready."]
    #[inline(always)]
    pub fn preload_rdy(&self) -> PRELOAD_RDY_R {
        PRELOAD_RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Transmit FIFO Count. These bits reflect the number of bytes in the TX_FIFO."]
    #[inline(always)]
    pub fn lvl(&self) -> LVL_R {
        LVL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO Preload Ready."]
    #[inline(always)]
    #[must_use]
    pub fn preload_rdy(&mut self) -> PRELOAD_RDY_W<TXCTRL1_SPEC> {
        PRELOAD_RDY_W::new(self, 0)
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
#[doc = "Transmit Control Register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXCTRL1_SPEC;
impl crate::RegisterSpec for TXCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txctrl1::R`](R) reader structure"]
impl crate::Readable for TXCTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txctrl1::W`](W) writer structure"]
impl crate::Writable for TXCTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXCTRL1 to value 0"]
impl crate::Resettable for TXCTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
