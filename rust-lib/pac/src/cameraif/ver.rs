#[doc = "Register `VER` reader"]
pub type R = crate::R<VER_SPEC>;
#[doc = "Register `VER` writer"]
pub type W = crate::W<VER_SPEC>;
#[doc = "Field `minor` reader - Minor Version Number."]
pub type MINOR_R = crate::FieldReader;
#[doc = "Field `minor` writer - Minor Version Number."]
pub type MINOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `major` reader - Major Version Number."]
pub type MAJOR_R = crate::FieldReader;
#[doc = "Field `major` writer - Major Version Number."]
pub type MAJOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Minor Version Number."]
    #[inline(always)]
    pub fn minor(&self) -> MINOR_R {
        MINOR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Major Version Number."]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Minor Version Number."]
    #[inline(always)]
    #[must_use]
    pub fn minor(&mut self) -> MINOR_W<VER_SPEC> {
        MINOR_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Major Version Number."]
    #[inline(always)]
    #[must_use]
    pub fn major(&mut self) -> MAJOR_W<VER_SPEC> {
        MAJOR_W::new(self, 8)
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
#[doc = "Hardware Version.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ver::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ver::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VER_SPEC;
impl crate::RegisterSpec for VER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ver::R`](R) reader structure"]
impl crate::Readable for VER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ver::W`](W) writer structure"]
impl crate::Writable for VER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VER to value 0"]
impl crate::Resettable for VER_SPEC {
    const RESET_VALUE: u32 = 0;
}
