#[doc = "Register `SLAVE_MULTI[%s]` reader"]
pub type R = crate::R<SLAVE_MULTI_SPEC>;
#[doc = "Register `SLAVE_MULTI[%s]` writer"]
pub type W = crate::W<SLAVE_MULTI_SPEC>;
#[doc = "Field `ADDR` reader - Slave Address."]
pub type ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Slave Address."]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DIS` reader - Slave Disable."]
pub type DIS_R = crate::BitReader;
#[doc = "Field `DIS` writer - Slave Disable."]
pub type DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_ADDR_EN` reader - Extended Address Select."]
pub type EXT_ADDR_EN_R = crate::BitReader<EXT_ADDR_EN_A>;
#[doc = "Extended Address Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXT_ADDR_EN_A {
    #[doc = "0: 7-bit address."]
    _7_BITS_ADDRESS = 0,
    #[doc = "1: 10-bit address."]
    _10_BITS_ADDRESS = 1,
}
impl From<EXT_ADDR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EXT_ADDR_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EXT_ADDR_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXT_ADDR_EN_A {
        match self.bits {
            false => EXT_ADDR_EN_A::_7_BITS_ADDRESS,
            true => EXT_ADDR_EN_A::_10_BITS_ADDRESS,
        }
    }
    #[doc = "7-bit address."]
    #[inline(always)]
    pub fn is_7_bits_address(&self) -> bool {
        *self == EXT_ADDR_EN_A::_7_BITS_ADDRESS
    }
    #[doc = "10-bit address."]
    #[inline(always)]
    pub fn is_10_bits_address(&self) -> bool {
        *self == EXT_ADDR_EN_A::_10_BITS_ADDRESS
    }
}
#[doc = "Field `EXT_ADDR_EN` writer - Extended Address Select."]
pub type EXT_ADDR_EN_W<'a, REG> = crate::BitWriter<'a, REG, EXT_ADDR_EN_A>;
impl<'a, REG> EXT_ADDR_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "7-bit address."]
    #[inline(always)]
    pub fn _7_bits_address(self) -> &'a mut crate::W<REG> {
        self.variant(EXT_ADDR_EN_A::_7_BITS_ADDRESS)
    }
    #[doc = "10-bit address."]
    #[inline(always)]
    pub fn _10_bits_address(self) -> &'a mut crate::W<REG> {
        self.variant(EXT_ADDR_EN_A::_10_BITS_ADDRESS)
    }
}
impl R {
    #[doc = "Bits 0:9 - Slave Address."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Slave Disable."]
    #[inline(always)]
    pub fn dis(&self) -> DIS_R {
        DIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Extended Address Select."]
    #[inline(always)]
    pub fn ext_addr_en(&self) -> EXT_ADDR_EN_R {
        EXT_ADDR_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave Address."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<SLAVE_MULTI_SPEC> {
        ADDR_W::new(self, 0)
    }
    #[doc = "Bit 10 - Slave Disable."]
    #[inline(always)]
    #[must_use]
    pub fn dis(&mut self) -> DIS_W<SLAVE_MULTI_SPEC> {
        DIS_W::new(self, 10)
    }
    #[doc = "Bit 15 - Extended Address Select."]
    #[inline(always)]
    #[must_use]
    pub fn ext_addr_en(&mut self) -> EXT_ADDR_EN_W<SLAVE_MULTI_SPEC> {
        EXT_ADDR_EN_W::new(self, 15)
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
#[doc = "Slave Address Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave_multi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave_multi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLAVE_MULTI_SPEC;
impl crate::RegisterSpec for SLAVE_MULTI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slave_multi::R`](R) reader structure"]
impl crate::Readable for SLAVE_MULTI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slave_multi::W`](W) writer structure"]
impl crate::Writable for SLAVE_MULTI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLAVE_MULTI[%s]
to value 0"]
impl crate::Resettable for SLAVE_MULTI_SPEC {
    const RESET_VALUE: u32 = 0;
}
