#[doc = "Register `LPPWST` reader"]
pub type R = crate::R<LPPWST_SPEC>;
#[doc = "Register `LPPWST` writer"]
pub type W = crate::W<LPPWST_SPEC>;
#[doc = "Field `AINCOMP0` reader - Analog Input Comparator Wakeup Flag."]
pub type AINCOMP0_R = crate::BitReader;
#[doc = "Field `AINCOMP0` writer - Analog Input Comparator Wakeup Flag."]
pub type AINCOMP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BACKUP` reader - Backup Mode Wakeup Flag."]
pub type BACKUP_R = crate::BitReader;
#[doc = "Field `BACKUP` writer - Backup Mode Wakeup Flag."]
pub type BACKUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - Reset Detected Wakeup Flag."]
pub type RESET_R = crate::BitReader;
#[doc = "Field `RESET` writer - Reset Detected Wakeup Flag."]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Analog Input Comparator Wakeup Flag."]
    #[inline(always)]
    pub fn aincomp0(&self) -> AINCOMP0_R {
        AINCOMP0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - Backup Mode Wakeup Flag."]
    #[inline(always)]
    pub fn backup(&self) -> BACKUP_R {
        BACKUP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset Detected Wakeup Flag."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Analog Input Comparator Wakeup Flag."]
    #[inline(always)]
    #[must_use]
    pub fn aincomp0(&mut self) -> AINCOMP0_W<LPPWST_SPEC> {
        AINCOMP0_W::new(self, 4)
    }
    #[doc = "Bit 16 - Backup Mode Wakeup Flag."]
    #[inline(always)]
    #[must_use]
    pub fn backup(&mut self) -> BACKUP_W<LPPWST_SPEC> {
        BACKUP_W::new(self, 16)
    }
    #[doc = "Bit 17 - Reset Detected Wakeup Flag."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<LPPWST_SPEC> {
        RESET_W::new(self, 17)
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
#[doc = "Low Power Peripheral Wakeup Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lppwst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lppwst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPPWST_SPEC;
impl crate::RegisterSpec for LPPWST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lppwst::R`](R) reader structure"]
impl crate::Readable for LPPWST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lppwst::W`](W) writer structure"]
impl crate::Writable for LPPWST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPPWST to value 0"]
impl crate::Resettable for LPPWST_SPEC {
    const RESET_VALUE: u32 = 0;
}
