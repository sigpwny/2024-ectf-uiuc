#[doc = "Register `SNAPSHOT` reader"]
pub type R = crate::R<SNAPSHOT_SPEC>;
#[doc = "Register `SNAPSHOT` writer"]
pub type W = crate::W<SNAPSHOT_SPEC>;
#[doc = "Field `SNAPSHOT` reader - Snapshot Value."]
pub type SNAPSHOT_R = crate::FieldReader<u32>;
#[doc = "Field `SNAPSHOT` writer - Snapshot Value."]
pub type SNAPSHOT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Snapshot Value."]
    #[inline(always)]
    pub fn snapshot(&self) -> SNAPSHOT_R {
        SNAPSHOT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Snapshot Value."]
    #[inline(always)]
    #[must_use]
    pub fn snapshot(&mut self) -> SNAPSHOT_W<SNAPSHOT_SPEC> {
        SNAPSHOT_W::new(self, 0)
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
#[doc = "Snapshot register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snapshot::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snapshot::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SNAPSHOT_SPEC;
impl crate::RegisterSpec for SNAPSHOT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`snapshot::R`](R) reader structure"]
impl crate::Readable for SNAPSHOT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`snapshot::W`](W) writer structure"]
impl crate::Writable for SNAPSHOT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SNAPSHOT to value 0"]
impl crate::Resettable for SNAPSHOT_SPEC {
    const RESET_VALUE: u32 = 0;
}
