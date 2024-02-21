#[doc = "Register `PRESET` reader"]
pub type R = crate::R<PRESET_SPEC>;
#[doc = "Register `PRESET` writer"]
pub type W = crate::W<PRESET_SPEC>;
#[doc = "Field `PRESET` reader - Preset Value."]
pub type PRESET_R = crate::FieldReader<u32>;
#[doc = "Field `PRESET` writer - Preset Value."]
pub type PRESET_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Preset Value."]
    #[inline(always)]
    pub fn preset(&self) -> PRESET_R {
        PRESET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Preset Value."]
    #[inline(always)]
    #[must_use]
    pub fn preset(&mut self) -> PRESET_W<PRESET_SPEC> {
        PRESET_W::new(self, 0)
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
#[doc = "Preset register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`preset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`preset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRESET_SPEC;
impl crate::RegisterSpec for PRESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`preset::R`](R) reader structure"]
impl crate::Readable for PRESET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`preset::W`](W) writer structure"]
impl crate::Writable for PRESET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRESET to value 0"]
impl crate::Resettable for PRESET_SPEC {
    const RESET_VALUE: u32 = 0;
}
