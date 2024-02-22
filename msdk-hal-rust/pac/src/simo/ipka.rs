#[doc = "Register `IPKA` reader"]
pub type R = crate::R<IPKA_SPEC>;
#[doc = "Register `IPKA` writer"]
pub type W = crate::W<IPKA_SPEC>;
#[doc = "Field `IPKSETA` reader - Voltage Regulator Peak Current Setting"]
pub type IPKSETA_R = crate::FieldReader;
#[doc = "Field `IPKSETA` writer - Voltage Regulator Peak Current Setting"]
pub type IPKSETA_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IPKSETB` reader - Voltage Regulator Peak Current Setting"]
pub type IPKSETB_R = crate::FieldReader;
#[doc = "Field `IPKSETB` writer - Voltage Regulator Peak Current Setting"]
pub type IPKSETB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    pub fn ipkseta(&self) -> IPKSETA_R {
        IPKSETA_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    pub fn ipksetb(&self) -> IPKSETB_R {
        IPKSETB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ipkseta(&mut self) -> IPKSETA_W<IPKA_SPEC> {
        IPKSETA_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ipksetb(&mut self) -> IPKSETB_W<IPKA_SPEC> {
        IPKSETB_W::new(self, 4)
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
#[doc = "High Side FET Peak Current VREGO_A/VREGO_B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipka::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipka::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPKA_SPEC;
impl crate::RegisterSpec for IPKA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipka::R`](R) reader structure"]
impl crate::Readable for IPKA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ipka::W`](W) writer structure"]
impl crate::Writable for IPKA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPKA to value 0"]
impl crate::Resettable for IPKA_SPEC {
    const RESET_VALUE: u32 = 0;
}
