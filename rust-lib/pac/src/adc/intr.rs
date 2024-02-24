#[doc = "Register `INTR` reader"]
pub type R = crate::R<INTR_SPEC>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<INTR_SPEC>;
#[doc = "Field `done_ie` reader - ADC Done Interrupt Enable"]
pub type DONE_IE_R = crate::BitReader;
#[doc = "Field `done_ie` writer - ADC Done Interrupt Enable"]
pub type DONE_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ref_ready_ie` reader - ADC Reference Ready Interrupt Enable"]
pub type REF_READY_IE_R = crate::BitReader;
#[doc = "Field `ref_ready_ie` writer - ADC Reference Ready Interrupt Enable"]
pub type REF_READY_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hi_limit_ie` reader - ADC Hi Limit Monitor Interrupt Enable"]
pub type HI_LIMIT_IE_R = crate::BitReader;
#[doc = "Field `hi_limit_ie` writer - ADC Hi Limit Monitor Interrupt Enable"]
pub type HI_LIMIT_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_limit_ie` reader - ADC Lo Limit Monitor Interrupt Enable"]
pub type LO_LIMIT_IE_R = crate::BitReader;
#[doc = "Field `lo_limit_ie` writer - ADC Lo Limit Monitor Interrupt Enable"]
pub type LO_LIMIT_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `overflow_ie` reader - ADC Overflow Interrupt Enable"]
pub type OVERFLOW_IE_R = crate::BitReader;
#[doc = "Field `overflow_ie` writer - ADC Overflow Interrupt Enable"]
pub type OVERFLOW_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `done_if` reader - ADC Done Interrupt Flag"]
pub type DONE_IF_R = crate::BitReader;
#[doc = "Field `done_if` writer - ADC Done Interrupt Flag"]
pub type DONE_IF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ref_ready_if` reader - ADC Reference Ready Interrupt Flag"]
pub type REF_READY_IF_R = crate::BitReader;
#[doc = "Field `ref_ready_if` writer - ADC Reference Ready Interrupt Flag"]
pub type REF_READY_IF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `hi_limit_if` reader - ADC Hi Limit Monitor Interrupt Flag"]
pub type HI_LIMIT_IF_R = crate::BitReader;
#[doc = "Field `hi_limit_if` writer - ADC Hi Limit Monitor Interrupt Flag"]
pub type HI_LIMIT_IF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `lo_limit_if` reader - ADC Lo Limit Monitor Interrupt Flag"]
pub type LO_LIMIT_IF_R = crate::BitReader;
#[doc = "Field `lo_limit_if` writer - ADC Lo Limit Monitor Interrupt Flag"]
pub type LO_LIMIT_IF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `overflow_if` reader - ADC Overflow Interrupt Flag"]
pub type OVERFLOW_IF_R = crate::BitReader;
#[doc = "Field `overflow_if` writer - ADC Overflow Interrupt Flag"]
pub type OVERFLOW_IF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `pending` reader - ADC Interrupt Pending Status"]
pub type PENDING_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ADC Done Interrupt Enable"]
    #[inline(always)]
    pub fn done_ie(&self) -> DONE_IE_R {
        DONE_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Reference Ready Interrupt Enable"]
    #[inline(always)]
    pub fn ref_ready_ie(&self) -> REF_READY_IE_R {
        REF_READY_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC Hi Limit Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn hi_limit_ie(&self) -> HI_LIMIT_IE_R {
        HI_LIMIT_IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Lo Limit Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn lo_limit_ie(&self) -> LO_LIMIT_IE_R {
        LO_LIMIT_IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn overflow_ie(&self) -> OVERFLOW_IE_R {
        OVERFLOW_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC Done Interrupt Flag"]
    #[inline(always)]
    pub fn done_if(&self) -> DONE_IF_R {
        DONE_IF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC Reference Ready Interrupt Flag"]
    #[inline(always)]
    pub fn ref_ready_if(&self) -> REF_READY_IF_R {
        REF_READY_IF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC Hi Limit Monitor Interrupt Flag"]
    #[inline(always)]
    pub fn hi_limit_if(&self) -> HI_LIMIT_IF_R {
        HI_LIMIT_IF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC Lo Limit Monitor Interrupt Flag"]
    #[inline(always)]
    pub fn lo_limit_if(&self) -> LO_LIMIT_IF_R {
        LO_LIMIT_IF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn overflow_if(&self) -> OVERFLOW_IF_R {
        OVERFLOW_IF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - ADC Interrupt Pending Status"]
    #[inline(always)]
    pub fn pending(&self) -> PENDING_R {
        PENDING_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Done Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn done_ie(&mut self) -> DONE_IE_W<INTR_SPEC> {
        DONE_IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC Reference Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ref_ready_ie(&mut self) -> REF_READY_IE_W<INTR_SPEC> {
        REF_READY_IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC Hi Limit Monitor Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hi_limit_ie(&mut self) -> HI_LIMIT_IE_W<INTR_SPEC> {
        HI_LIMIT_IE_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC Lo Limit Monitor Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lo_limit_ie(&mut self) -> LO_LIMIT_IE_W<INTR_SPEC> {
        LO_LIMIT_IE_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn overflow_ie(&mut self) -> OVERFLOW_IE_W<INTR_SPEC> {
        OVERFLOW_IE_W::new(self, 4)
    }
    #[doc = "Bit 16 - ADC Done Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn done_if(&mut self) -> DONE_IF_W<INTR_SPEC> {
        DONE_IF_W::new(self, 16)
    }
    #[doc = "Bit 17 - ADC Reference Ready Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ref_ready_if(&mut self) -> REF_READY_IF_W<INTR_SPEC> {
        REF_READY_IF_W::new(self, 17)
    }
    #[doc = "Bit 18 - ADC Hi Limit Monitor Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hi_limit_if(&mut self) -> HI_LIMIT_IF_W<INTR_SPEC> {
        HI_LIMIT_IF_W::new(self, 18)
    }
    #[doc = "Bit 19 - ADC Lo Limit Monitor Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lo_limit_if(&mut self) -> LO_LIMIT_IF_W<INTR_SPEC> {
        LO_LIMIT_IF_W::new(self, 19)
    }
    #[doc = "Bit 20 - ADC Overflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn overflow_if(&mut self) -> OVERFLOW_IF_W<INTR_SPEC> {
        OVERFLOW_IF_W::new(self, 20)
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
#[doc = "ADC Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for INTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x001f_0000;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
