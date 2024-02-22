#[doc = "Register `TXPEEK` reader"]
pub type R = crate::R<TXPEEK_SPEC>;
#[doc = "Register `TXPEEK` writer"]
pub type W = crate::W<TXPEEK_SPEC>;
#[doc = "Field `DATA` reader - Read TX FIFO next data. Reading from this field does not affect the contents of TX FIFO. Note that the parity bit is available from this field."]
pub type DATA_R = crate::FieldReader;
#[doc = "Field `DATA` writer - Read TX FIFO next data. Reading from this field does not affect the contents of TX FIFO. Note that the parity bit is available from this field."]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Read TX FIFO next data. Reading from this field does not affect the contents of TX FIFO. Note that the parity bit is available from this field."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read TX FIFO next data. Reading from this field does not affect the contents of TX FIFO. Note that the parity bit is available from this field."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<TXPEEK_SPEC> {
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
#[doc = "TX FIFO Output Peek register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txpeek::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txpeek::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXPEEK_SPEC;
impl crate::RegisterSpec for TXPEEK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txpeek::R`](R) reader structure"]
impl crate::Readable for TXPEEK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txpeek::W`](W) writer structure"]
impl crate::Writable for TXPEEK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXPEEK to value 0"]
impl crate::Resettable for TXPEEK_SPEC {
    const RESET_VALUE: u32 = 0;
}
