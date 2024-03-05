#[doc = "Register `MEMCTRL` reader"]
pub type R = crate::R<MEMCTRL_SPEC>;
#[doc = "Register `MEMCTRL` writer"]
pub type W = crate::W<MEMCTRL_SPEC>;
#[doc = "Field `FWS` reader - Flash Wait State. These bits define the number of wait-state cycles per Flash data read access. Minimum wait state is 2."]
pub type FWS_R = crate::FieldReader;
#[doc = "Field `FWS` writer - Flash Wait State. These bits define the number of wait-state cycles per Flash data read access. Minimum wait state is 2."]
pub type FWS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SYSRAM0ECC` reader - SYSRAM0 ECC Select."]
pub type SYSRAM0ECC_R = crate::BitReader;
#[doc = "Field `SYSRAM0ECC` writer - SYSRAM0 ECC Select."]
pub type SYSRAM0ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Flash Wait State. These bits define the number of wait-state cycles per Flash data read access. Minimum wait state is 2."]
    #[inline(always)]
    pub fn fws(&self) -> FWS_R {
        FWS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 16 - SYSRAM0 ECC Select."]
    #[inline(always)]
    pub fn sysram0ecc(&self) -> SYSRAM0ECC_R {
        SYSRAM0ECC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Flash Wait State. These bits define the number of wait-state cycles per Flash data read access. Minimum wait state is 2."]
    #[inline(always)]
    #[must_use]
    pub fn fws(&mut self) -> FWS_W<MEMCTRL_SPEC> {
        FWS_W::new(self, 0)
    }
    #[doc = "Bit 16 - SYSRAM0 ECC Select."]
    #[inline(always)]
    #[must_use]
    pub fn sysram0ecc(&mut self) -> SYSRAM0ECC_W<MEMCTRL_SPEC> {
        SYSRAM0ECC_W::new(self, 16)
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
#[doc = "Memory Clock Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEMCTRL_SPEC;
impl crate::RegisterSpec for MEMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memctrl::R`](R) reader structure"]
impl crate::Readable for MEMCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`memctrl::W`](W) writer structure"]
impl crate::Writable for MEMCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMCTRL to value 0"]
impl crate::Resettable for MEMCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
