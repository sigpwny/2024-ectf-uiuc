#[doc = "Register `status` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `status` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `status0` reader - "]
pub type STATUS0_R = crate::BitReader;
#[doc = "Field `status0` writer - "]
pub type STATUS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `status1` reader - "]
pub type STATUS1_R = crate::BitReader;
#[doc = "Field `status1` writer - "]
pub type STATUS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `status2` reader - "]
pub type STATUS2_R = crate::BitReader;
#[doc = "Field `status2` writer - "]
pub type STATUS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `status3` reader - "]
pub type STATUS3_R = crate::BitReader;
#[doc = "Field `status3` writer - "]
pub type STATUS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `status4` reader - "]
pub type STATUS4_R = crate::BitReader;
#[doc = "Field `status4` writer - "]
pub type STATUS4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `status5` reader - "]
pub type STATUS5_R = crate::BitReader;
#[doc = "Field `status5` writer - "]
pub type STATUS5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `status6` reader - "]
pub type STATUS6_R = crate::BitReader;
#[doc = "Field `status6` writer - "]
pub type STATUS6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `status7` reader - "]
pub type STATUS7_R = crate::BitReader;
#[doc = "Field `status7` writer - "]
pub type STATUS7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn status0(&self) -> STATUS0_R {
        STATUS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn status1(&self) -> STATUS1_R {
        STATUS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn status2(&self) -> STATUS2_R {
        STATUS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn status3(&self) -> STATUS3_R {
        STATUS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn status4(&self) -> STATUS4_R {
        STATUS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn status5(&self) -> STATUS5_R {
        STATUS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn status6(&self) -> STATUS6_R {
        STATUS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn status7(&self) -> STATUS7_R {
        STATUS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn status0(&mut self) -> STATUS0_W<STATUS_SPEC> {
        STATUS0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn status1(&mut self) -> STATUS1_W<STATUS_SPEC> {
        STATUS1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn status2(&mut self) -> STATUS2_W<STATUS_SPEC> {
        STATUS2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn status3(&mut self) -> STATUS3_W<STATUS_SPEC> {
        STATUS3_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn status4(&mut self) -> STATUS4_W<STATUS_SPEC> {
        STATUS4_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn status5(&mut self) -> STATUS5_W<STATUS_SPEC> {
        STATUS5_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn status6(&mut self) -> STATUS6_W<STATUS_SPEC> {
        STATUS6_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn status7(&mut self) -> STATUS7_W<STATUS_SPEC> {
        STATUS7_W::new(self, 7)
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
#[doc = "Semaphore status bits. 0 indicates the semaphore is free, 1 indicates taken.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
