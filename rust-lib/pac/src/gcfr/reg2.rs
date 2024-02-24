#[doc = "Register `REG2` reader"]
pub type R = crate::R<REG2_SPEC>;
#[doc = "Register `REG2` writer"]
pub type W = crate::W<REG2_SPEC>;
#[doc = "Field `cnnx16_0_iso` reader - CNNx16_0 Power Domain Isolation"]
pub type CNNX16_0_ISO_R = crate::BitReader;
#[doc = "Field `cnnx16_0_iso` writer - CNNx16_0 Power Domain Isolation"]
pub type CNNX16_0_ISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_1_iso` reader - CNNx16_1 Power Domain Isolation"]
pub type CNNX16_1_ISO_R = crate::BitReader;
#[doc = "Field `cnnx16_1_iso` writer - CNNx16_1 Power Domain Isolation"]
pub type CNNX16_1_ISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_2_iso` reader - CNNx16_2 Power Domain Isolation"]
pub type CNNX16_2_ISO_R = crate::BitReader;
#[doc = "Field `cnnx16_2_iso` writer - CNNx16_2 Power Domain Isolation"]
pub type CNNX16_2_ISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_3_iso` reader - CNNx16_3 Power Domain Isolation"]
pub type CNNX16_3_ISO_R = crate::BitReader;
#[doc = "Field `cnnx16_3_iso` writer - CNNx16_3 Power Domain Isolation"]
pub type CNNX16_3_ISO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CNNx16_0 Power Domain Isolation"]
    #[inline(always)]
    pub fn cnnx16_0_iso(&self) -> CNNX16_0_ISO_R {
        CNNX16_0_ISO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CNNx16_1 Power Domain Isolation"]
    #[inline(always)]
    pub fn cnnx16_1_iso(&self) -> CNNX16_1_ISO_R {
        CNNX16_1_ISO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CNNx16_2 Power Domain Isolation"]
    #[inline(always)]
    pub fn cnnx16_2_iso(&self) -> CNNX16_2_ISO_R {
        CNNX16_2_ISO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CNNx16_3 Power Domain Isolation"]
    #[inline(always)]
    pub fn cnnx16_3_iso(&self) -> CNNX16_3_ISO_R {
        CNNX16_3_ISO_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CNNx16_0 Power Domain Isolation"]
    #[inline(always)]
    #[must_use]
    pub fn cnnx16_0_iso(&mut self) -> CNNX16_0_ISO_W<REG2_SPEC> {
        CNNX16_0_ISO_W::new(self, 0)
    }
    #[doc = "Bit 1 - CNNx16_1 Power Domain Isolation"]
    #[inline(always)]
    #[must_use]
    pub fn cnnx16_1_iso(&mut self) -> CNNX16_1_ISO_W<REG2_SPEC> {
        CNNX16_1_ISO_W::new(self, 1)
    }
    #[doc = "Bit 2 - CNNx16_2 Power Domain Isolation"]
    #[inline(always)]
    #[must_use]
    pub fn cnnx16_2_iso(&mut self) -> CNNX16_2_ISO_W<REG2_SPEC> {
        CNNX16_2_ISO_W::new(self, 2)
    }
    #[doc = "Bit 3 - CNNx16_3 Power Domain Isolation"]
    #[inline(always)]
    #[must_use]
    pub fn cnnx16_3_iso(&mut self) -> CNNX16_3_ISO_W<REG2_SPEC> {
        CNNX16_3_ISO_W::new(self, 3)
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
#[doc = "Register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG2_SPEC;
impl crate::RegisterSpec for REG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg2::R`](R) reader structure"]
impl crate::Readable for REG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg2::W`](W) writer structure"]
impl crate::Writable for REG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG2 to value 0"]
impl crate::Resettable for REG2_SPEC {
    const RESET_VALUE: u32 = 0;
}
