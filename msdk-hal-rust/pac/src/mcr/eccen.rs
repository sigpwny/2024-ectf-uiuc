#[doc = "Register `ECCEN` reader"]
pub type R = crate::R<ECCEN_SPEC>;
#[doc = "Register `ECCEN` writer"]
pub type W = crate::W<ECCEN_SPEC>;
#[doc = "Field `RAM0` reader - ECC System RAM0 Enable."]
pub type RAM0_R = crate::BitReader<RAM0_A>;
#[doc = "ECC System RAM0 Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM0_A {
    #[doc = "0: disabled."]
    DIS = 0,
    #[doc = "1: enabled."]
    EN = 1,
}
impl From<RAM0_A> for bool {
    #[inline(always)]
    fn from(variant: RAM0_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RAM0_A {
        match self.bits {
            false => RAM0_A::DIS,
            true => RAM0_A::EN,
        }
    }
    #[doc = "disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RAM0_A::DIS
    }
    #[doc = "enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RAM0_A::EN
    }
}
#[doc = "Field `RAM0` writer - ECC System RAM0 Enable."]
pub type RAM0_W<'a, REG> = crate::BitWriter<'a, REG, RAM0_A>;
impl<'a, REG> RAM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RAM0_A::DIS)
    }
    #[doc = "enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RAM0_A::EN)
    }
}
impl R {
    #[doc = "Bit 0 - ECC System RAM0 Enable."]
    #[inline(always)]
    pub fn ram0(&self) -> RAM0_R {
        RAM0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC System RAM0 Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ram0(&mut self) -> RAM0_W<ECCEN_SPEC> {
        RAM0_W::new(self, 0)
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
#[doc = "ECC Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCEN_SPEC;
impl crate::RegisterSpec for ECCEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccen::R`](R) reader structure"]
impl crate::Readable for ECCEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eccen::W`](W) writer structure"]
impl crate::Writable for ECCEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCEN to value 0"]
impl crate::Resettable for ECCEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
