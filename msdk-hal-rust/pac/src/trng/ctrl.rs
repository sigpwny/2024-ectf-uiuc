#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `RND_IE` reader - To enable IRQ generation when a new 32-bit Random number is ready."]
pub type RND_IE_R = crate::BitReader<RND_IE_A>;
#[doc = "To enable IRQ generation when a new 32-bit Random number is ready.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RND_IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<RND_IE_A> for bool {
    #[inline(always)]
    fn from(variant: RND_IE_A) -> Self {
        variant as u8 != 0
    }
}
impl RND_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RND_IE_A {
        match self.bits {
            false => RND_IE_A::DISABLE,
            true => RND_IE_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RND_IE_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RND_IE_A::ENABLE
    }
}
#[doc = "Field `RND_IE` writer - To enable IRQ generation when a new 32-bit Random number is ready."]
pub type RND_IE_W<'a, REG> = crate::BitWriter<'a, REG, RND_IE_A>;
impl<'a, REG> RND_IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RND_IE_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RND_IE_A::ENABLE)
    }
}
#[doc = "Field `KEYGEN` reader - AES Key Generate. When enabled, the key for securing NVSRAM is generated and transferred to the secure key register automatically without user visibility or intervention. This bit is cleared by hardware once the key has been transferred to the secure key register."]
pub type KEYGEN_R = crate::BitReader;
#[doc = "Field `KEYGEN` writer - AES Key Generate. When enabled, the key for securing NVSRAM is generated and transferred to the secure key register automatically without user visibility or intervention. This bit is cleared by hardware once the key has been transferred to the secure key register."]
pub type KEYGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYWIPE` reader - To wipe the Battery Backed key."]
pub type KEYWIPE_R = crate::BitReader;
#[doc = "Field `KEYWIPE` writer - To wipe the Battery Backed key."]
pub type KEYWIPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - To enable IRQ generation when a new 32-bit Random number is ready."]
    #[inline(always)]
    pub fn rnd_ie(&self) -> RND_IE_R {
        RND_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - AES Key Generate. When enabled, the key for securing NVSRAM is generated and transferred to the secure key register automatically without user visibility or intervention. This bit is cleared by hardware once the key has been transferred to the secure key register."]
    #[inline(always)]
    pub fn keygen(&self) -> KEYGEN_R {
        KEYGEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 15 - To wipe the Battery Backed key."]
    #[inline(always)]
    pub fn keywipe(&self) -> KEYWIPE_R {
        KEYWIPE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - To enable IRQ generation when a new 32-bit Random number is ready."]
    #[inline(always)]
    #[must_use]
    pub fn rnd_ie(&mut self) -> RND_IE_W<CTRL_SPEC> {
        RND_IE_W::new(self, 1)
    }
    #[doc = "Bit 3 - AES Key Generate. When enabled, the key for securing NVSRAM is generated and transferred to the secure key register automatically without user visibility or intervention. This bit is cleared by hardware once the key has been transferred to the secure key register."]
    #[inline(always)]
    #[must_use]
    pub fn keygen(&mut self) -> KEYGEN_W<CTRL_SPEC> {
        KEYGEN_W::new(self, 3)
    }
    #[doc = "Bit 15 - To wipe the Battery Backed key."]
    #[inline(always)]
    #[must_use]
    pub fn keywipe(&mut self) -> KEYWIPE_W<CTRL_SPEC> {
        KEYWIPE_W::new(self, 15)
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
#[doc = "TRNG Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x03"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
