#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `BUSY` reader - AES Busy Status"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `BUSY` writer - AES Busy Status"]
pub type BUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUT_EM` reader - Data input FIFO empty status"]
pub type INPUT_EM_R = crate::BitReader;
#[doc = "Field `INPUT_EM` writer - Data input FIFO empty status"]
pub type INPUT_EM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUT_FULL` reader - Data input FIFO full status"]
pub type INPUT_FULL_R = crate::BitReader;
#[doc = "Field `INPUT_FULL` writer - Data input FIFO full status"]
pub type INPUT_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPUT_EM` reader - Data output FIFO empty status"]
pub type OUTPUT_EM_R = crate::BitReader;
#[doc = "Field `OUTPUT_EM` writer - Data output FIFO empty status"]
pub type OUTPUT_EM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPUT_FULL` reader - Data output FIFO full status"]
pub type OUTPUT_FULL_R = crate::BitReader;
#[doc = "Field `OUTPUT_FULL` writer - Data output FIFO full status"]
pub type OUTPUT_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AES Busy Status"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data input FIFO empty status"]
    #[inline(always)]
    pub fn input_em(&self) -> INPUT_EM_R {
        INPUT_EM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data input FIFO full status"]
    #[inline(always)]
    pub fn input_full(&self) -> INPUT_FULL_R {
        INPUT_FULL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data output FIFO empty status"]
    #[inline(always)]
    pub fn output_em(&self) -> OUTPUT_EM_R {
        OUTPUT_EM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data output FIFO full status"]
    #[inline(always)]
    pub fn output_full(&self) -> OUTPUT_FULL_R {
        OUTPUT_FULL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES Busy Status"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<STATUS_SPEC> {
        BUSY_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data input FIFO empty status"]
    #[inline(always)]
    #[must_use]
    pub fn input_em(&mut self) -> INPUT_EM_W<STATUS_SPEC> {
        INPUT_EM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Data input FIFO full status"]
    #[inline(always)]
    #[must_use]
    pub fn input_full(&mut self) -> INPUT_FULL_W<STATUS_SPEC> {
        INPUT_FULL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Data output FIFO empty status"]
    #[inline(always)]
    #[must_use]
    pub fn output_em(&mut self) -> OUTPUT_EM_W<STATUS_SPEC> {
        OUTPUT_EM_W::new(self, 3)
    }
    #[doc = "Bit 4 - Data output FIFO full status"]
    #[inline(always)]
    #[must_use]
    pub fn output_full(&mut self) -> OUTPUT_FULL_W<STATUS_SPEC> {
        OUTPUT_FULL_W::new(self, 4)
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
#[doc = "AES Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
