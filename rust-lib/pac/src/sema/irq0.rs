#[doc = "Register `irq0` reader"]
pub type R = crate::R<IRQ0_SPEC>;
#[doc = "Register `irq0` writer"]
pub type W = crate::W<IRQ0_SPEC>;
#[doc = "Field `en` reader - "]
pub type EN_R = crate::BitReader;
#[doc = "Field `en` writer - "]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cm4_irq` reader - "]
pub type CM4_IRQ_R = crate::BitReader;
#[doc = "Field `cm4_irq` writer - "]
pub type CM4_IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cm4_irq(&self) -> CM4_IRQ_R {
        CM4_IRQ_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<IRQ0_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cm4_irq(&mut self) -> CM4_IRQ_W<IRQ0_SPEC> {
        CM4_IRQ_W::new(self, 16)
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
#[doc = "Semaphore IRQ0 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ0_SPEC;
impl crate::RegisterSpec for IRQ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq0::R`](R) reader structure"]
impl crate::Readable for IRQ0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irq0::W`](W) writer structure"]
impl crate::Writable for IRQ0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets irq0 to value 0"]
impl crate::Resettable for IRQ0_SPEC {
    const RESET_VALUE: u32 = 0;
}
