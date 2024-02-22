#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `DONE` reader - AES Done Interrupt Enable"]
pub type DONE_R = crate::BitReader;
#[doc = "Field `DONE` writer - AES Done Interrupt Enable"]
pub type DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_CHANGE` reader - External AES Key Changed Interrupt Enable"]
pub type KEY_CHANGE_R = crate::BitReader;
#[doc = "Field `KEY_CHANGE` writer - External AES Key Changed Interrupt Enable"]
pub type KEY_CHANGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_ZERO` reader - External AES Key Zero Interrupt Enable"]
pub type KEY_ZERO_R = crate::BitReader;
#[doc = "Field `KEY_ZERO` writer - External AES Key Zero Interrupt Enable"]
pub type KEY_ZERO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OV` reader - Data Output FIFO Overrun Interrupt Enable"]
pub type OV_R = crate::BitReader;
#[doc = "Field `OV` writer - Data Output FIFO Overrun Interrupt Enable"]
pub type OV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_ONE` reader - KEY_ONE"]
pub type KEY_ONE_R = crate::BitReader;
#[doc = "Field `KEY_ONE` writer - KEY_ONE"]
pub type KEY_ONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AES Done Interrupt Enable"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External AES Key Changed Interrupt Enable"]
    #[inline(always)]
    pub fn key_change(&self) -> KEY_CHANGE_R {
        KEY_CHANGE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External AES Key Zero Interrupt Enable"]
    #[inline(always)]
    pub fn key_zero(&self) -> KEY_ZERO_R {
        KEY_ZERO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Output FIFO Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ov(&self) -> OV_R {
        OV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - KEY_ONE"]
    #[inline(always)]
    pub fn key_one(&self) -> KEY_ONE_R {
        KEY_ONE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES Done Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<INTEN_SPEC> {
        DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - External AES Key Changed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn key_change(&mut self) -> KEY_CHANGE_W<INTEN_SPEC> {
        KEY_CHANGE_W::new(self, 1)
    }
    #[doc = "Bit 2 - External AES Key Zero Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn key_zero(&mut self) -> KEY_ZERO_W<INTEN_SPEC> {
        KEY_ZERO_W::new(self, 2)
    }
    #[doc = "Bit 3 - Data Output FIFO Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ov(&mut self) -> OV_W<INTEN_SPEC> {
        OV_W::new(self, 3)
    }
    #[doc = "Bit 4 - KEY_ONE"]
    #[inline(always)]
    #[must_use]
    pub fn key_one(&mut self) -> KEY_ONE_W<INTEN_SPEC> {
        KEY_ONE_W::new(self, 4)
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
#[doc = "AES Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
