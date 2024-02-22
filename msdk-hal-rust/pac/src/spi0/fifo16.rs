#[doc = "Register `FIFO16[%s]` reader"]
pub type R = crate::R<FIFO16_SPEC>;
#[doc = "Register `FIFO16[%s]` writer"]
pub type W = crate::W<FIFO16_SPEC>;
#[doc = "Field `DATA` reader - Read to pull from RX FIFO, write to put into TX FIFO."]
pub type DATA_R = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - Read to pull from RX FIFO, write to put into TX FIFO."]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Read to pull from RX FIFO, write to put into TX FIFO."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Read to pull from RX FIFO, write to put into TX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<FIFO16_SPEC> {
        DATA_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Register for reading and writing the FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO16_SPEC;
impl crate::RegisterSpec for FIFO16_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fifo16::R`](R) reader structure"]
impl crate::Readable for FIFO16_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo16::W`](W) writer structure"]
impl crate::Writable for FIFO16_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FIFO16[%s]
to value 0"]
impl crate::Resettable for FIFO16_SPEC {
    const RESET_VALUE: u16 = 0;
}
