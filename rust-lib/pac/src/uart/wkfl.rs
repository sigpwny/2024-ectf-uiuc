#[doc = "Register `WKFL` reader"]
pub type R = crate::R<WKFL_SPEC>;
#[doc = "Register `WKFL` writer"]
pub type W = crate::W<WKFL_SPEC>;
#[doc = "Field `RX_NE` reader - Wake-Up Flag for RX FIFO Not Empty"]
pub type RX_NE_R = crate::BitReader;
#[doc = "Field `RX_NE` writer - Wake-Up Flag for RX FIFO Not Empty"]
pub type RX_NE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FULL` reader - Wake-Up Flag for RX FIFO Full"]
pub type RX_FULL_R = crate::BitReader;
#[doc = "Field `RX_FULL` writer - Wake-Up Flag for RX FIFO Full"]
pub type RX_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_THD` reader - Wake-Up Flag for RX FIFO Threshold Met"]
pub type RX_THD_R = crate::BitReader;
#[doc = "Field `RX_THD` writer - Wake-Up Flag for RX FIFO Threshold Met"]
pub type RX_THD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wake-Up Flag for RX FIFO Not Empty"]
    #[inline(always)]
    pub fn rx_ne(&self) -> RX_NE_R {
        RX_NE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake-Up Flag for RX FIFO Full"]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-Up Flag for RX FIFO Threshold Met"]
    #[inline(always)]
    pub fn rx_thd(&self) -> RX_THD_R {
        RX_THD_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-Up Flag for RX FIFO Not Empty"]
    #[inline(always)]
    #[must_use]
    pub fn rx_ne(&mut self) -> RX_NE_W<WKFL_SPEC> {
        RX_NE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Wake-Up Flag for RX FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn rx_full(&mut self) -> RX_FULL_W<WKFL_SPEC> {
        RX_FULL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Wake-Up Flag for RX FIFO Threshold Met"]
    #[inline(always)]
    #[must_use]
    pub fn rx_thd(&mut self) -> RX_THD_W<WKFL_SPEC> {
        RX_THD_W::new(self, 2)
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
#[doc = "Wake up Flags register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wkfl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wkfl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WKFL_SPEC;
impl crate::RegisterSpec for WKFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wkfl::R`](R) reader structure"]
impl crate::Readable for WKFL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wkfl::W`](W) writer structure"]
impl crate::Writable for WKFL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WKFL to value 0"]
impl crate::Resettable for WKFL_SPEC {
    const RESET_VALUE: u32 = 0;
}
