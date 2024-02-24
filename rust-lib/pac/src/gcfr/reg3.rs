#[doc = "Register `REG3` reader"]
pub type R = crate::R<REG3_SPEC>;
#[doc = "Register `REG3` writer"]
pub type W = crate::W<REG3_SPEC>;
#[doc = "Field `cnnx16_0_rst` reader - CNNx16_0 Power Domain Reset"]
pub type CNNX16_0_RST_R = crate::BitReader;
#[doc = "Field `cnnx16_0_rst` writer - CNNx16_0 Power Domain Reset"]
pub type CNNX16_0_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_1_rst` reader - CNNx16_1 Power Domain Reset"]
pub type CNNX16_1_RST_R = crate::BitReader;
#[doc = "Field `cnnx16_1_rst` writer - CNNx16_1 Power Domain Reset"]
pub type CNNX16_1_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_2_rst` reader - CNNx16_2 Power Domain Reset"]
pub type CNNX16_2_RST_R = crate::BitReader;
#[doc = "Field `cnnx16_2_rst` writer - CNNx16_2 Power Domain Reset"]
pub type CNNX16_2_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_3_rst` reader - CNNx16_3 Power Domain Reset"]
pub type CNNX16_3_RST_R = crate::BitReader;
#[doc = "Field `cnnx16_3_rst` writer - CNNx16_3 Power Domain Reset"]
pub type CNNX16_3_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CNNx16_0 Power Domain Reset"]
    #[inline(always)]
    pub fn cnnx16_0_rst(&self) -> CNNX16_0_RST_R {
        CNNX16_0_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CNNx16_1 Power Domain Reset"]
    #[inline(always)]
    pub fn cnnx16_1_rst(&self) -> CNNX16_1_RST_R {
        CNNX16_1_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CNNx16_2 Power Domain Reset"]
    #[inline(always)]
    pub fn cnnx16_2_rst(&self) -> CNNX16_2_RST_R {
        CNNX16_2_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CNNx16_3 Power Domain Reset"]
    #[inline(always)]
    pub fn cnnx16_3_rst(&self) -> CNNX16_3_RST_R {
        CNNX16_3_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CNNx16_0 Power Domain Reset"]
    #[inline(always)]
    #[must_use]
    pub fn cnnx16_0_rst(&mut self) -> CNNX16_0_RST_W<REG3_SPEC> {
        CNNX16_0_RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - CNNx16_1 Power Domain Reset"]
    #[inline(always)]
    #[must_use]
    pub fn cnnx16_1_rst(&mut self) -> CNNX16_1_RST_W<REG3_SPEC> {
        CNNX16_1_RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - CNNx16_2 Power Domain Reset"]
    #[inline(always)]
    #[must_use]
    pub fn cnnx16_2_rst(&mut self) -> CNNX16_2_RST_W<REG3_SPEC> {
        CNNX16_2_RST_W::new(self, 2)
    }
    #[doc = "Bit 3 - CNNx16_3 Power Domain Reset"]
    #[inline(always)]
    #[must_use]
    pub fn cnnx16_3_rst(&mut self) -> CNNX16_3_RST_W<REG3_SPEC> {
        CNNX16_3_RST_W::new(self, 3)
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
#[doc = "Register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG3_SPEC;
impl crate::RegisterSpec for REG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg3::R`](R) reader structure"]
impl crate::Readable for REG3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg3::W`](W) writer structure"]
impl crate::Writable for REG3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG3 to value 0"]
impl crate::Resettable for REG3_SPEC {
    const RESET_VALUE: u32 = 0;
}
