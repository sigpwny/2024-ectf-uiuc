#[doc = "Register `FIFOCH0` reader"]
pub type R = crate::R<FIFOCH0_SPEC>;
#[doc = "Register `FIFOCH0` writer"]
pub type W = crate::W<FIFOCH0_SPEC>;
#[doc = "Field `DATA` reader - Load/unload location for TX and RX FIFO buffers."]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Load/unload location for TX and RX FIFO buffers."]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Load/unload location for TX and RX FIFO buffers."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Load/unload location for TX and RX FIFO buffers."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<FIFOCH0_SPEC> {
        DATA_W::new(self, 0)
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
#[doc = "I2S Fifo.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifoch0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifoch0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFOCH0_SPEC;
impl crate::RegisterSpec for FIFOCH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifoch0::R`](R) reader structure"]
impl crate::Readable for FIFOCH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifoch0::W`](W) writer structure"]
impl crate::Writable for FIFOCH0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOCH0 to value 0"]
impl crate::Resettable for FIFOCH0_SPEC {
    const RESET_VALUE: u32 = 0;
}
