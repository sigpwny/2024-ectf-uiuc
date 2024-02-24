#[doc = "Register `FIFO_DATA` reader"]
pub type R = crate::R<FIFO_DATA_SPEC>;
#[doc = "Register `FIFO_DATA` writer"]
pub type W = crate::W<FIFO_DATA_SPEC>;
#[doc = "Field `DATA` reader - Data from FIFO to be read by DMA."]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data from FIFO to be read by DMA."]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data from FIFO to be read by DMA."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data from FIFO to be read by DMA."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<FIFO_DATA_SPEC> {
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
#[doc = "FIFO DATA Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_DATA_SPEC;
impl crate::RegisterSpec for FIFO_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_data::R`](R) reader structure"]
impl crate::Readable for FIFO_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo_data::W`](W) writer structure"]
impl crate::Writable for FIFO_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO_DATA to value 0"]
impl crate::Resettable for FIFO_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
