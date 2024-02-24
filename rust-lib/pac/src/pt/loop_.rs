#[doc = "Register `LOOP` reader"]
pub type R = crate::R<LOOP_SPEC>;
#[doc = "Register `LOOP` writer"]
pub type W = crate::W<LOOP_SPEC>;
#[doc = "Field `count` reader - Number of loops for this pulse train to repeat."]
pub type COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `count` writer - Number of loops for this pulse train to repeat."]
pub type COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `delay` reader - Delay between loops of the Pulse Train in PT Peripheral Clock cycles"]
pub type DELAY_R = crate::FieldReader<u16>;
#[doc = "Field `delay` writer - Delay between loops of the Pulse Train in PT Peripheral Clock cycles"]
pub type DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of loops for this pulse train to repeat."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:27 - Delay between loops of the Pulse Train in PT Peripheral Clock cycles"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of loops for this pulse train to repeat."]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<LOOP_SPEC> {
        COUNT_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Delay between loops of the Pulse Train in PT Peripheral Clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<LOOP_SPEC> {
        DELAY_W::new(self, 16)
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
#[doc = "Pulse Train Loop Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`loop_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`loop_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOOP_SPEC;
impl crate::RegisterSpec for LOOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`loop_::R`](R) reader structure"]
impl crate::Readable for LOOP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`loop_::W`](W) writer structure"]
impl crate::Writable for LOOP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOOP to value 0"]
impl crate::Resettable for LOOP_SPEC {
    const RESET_VALUE: u32 = 0;
}
