#[doc = "Register `DIRECT` reader"]
pub type R = crate::R<DIRECT_SPEC>;
#[doc = "Register `DIRECT` writer"]
pub type W = crate::W<DIRECT_SPEC>;
#[doc = "Field `VOLTAGE` reader - Sets the target power supply value"]
pub type VOLTAGE_R = crate::FieldReader;
#[doc = "Field `VOLTAGE` writer - Sets the target power supply value"]
pub type VOLTAGE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Sets the target power supply value"]
    #[inline(always)]
    pub fn voltage(&self) -> VOLTAGE_R {
        VOLTAGE_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Sets the target power supply value"]
    #[inline(always)]
    #[must_use]
    pub fn voltage(&mut self) -> VOLTAGE_W<DIRECT_SPEC> {
        VOLTAGE_W::new(self, 0)
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
#[doc = "Direct control of target voltage\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`direct::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`direct::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIRECT_SPEC;
impl crate::RegisterSpec for DIRECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`direct::R`](R) reader structure"]
impl crate::Readable for DIRECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`direct::W`](W) writer structure"]
impl crate::Writable for DIRECT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIRECT to value 0"]
impl crate::Resettable for DIRECT_SPEC {
    const RESET_VALUE: u32 = 0;
}
