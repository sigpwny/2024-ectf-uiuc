#[doc = "Register `DMA` reader"]
pub type R = crate::R<DMA_SPEC>;
#[doc = "Register `DMA` writer"]
pub type W = crate::W<DMA_SPEC>;
#[doc = "Field `TX_EN` reader - TX channel enable."]
pub type TX_EN_R = crate::BitReader<TX_EN_A>;
#[doc = "TX channel enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_EN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<TX_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_EN_A {
        match self.bits {
            false => TX_EN_A::DIS,
            true => TX_EN_A::EN,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TX_EN_A::DIS
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TX_EN_A::EN
    }
}
#[doc = "Field `TX_EN` writer - TX channel enable."]
pub type TX_EN_W<'a, REG> = crate::BitWriter<'a, REG, TX_EN_A>;
impl<'a, REG> TX_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TX_EN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TX_EN_A::EN)
    }
}
#[doc = "Field `RX_EN` reader - RX channel enable."]
pub type RX_EN_R = crate::BitReader<RX_EN_A>;
#[doc = "RX channel enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_EN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<RX_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_EN_A {
        match self.bits {
            false => RX_EN_A::DIS,
            true => RX_EN_A::EN,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RX_EN_A::DIS
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RX_EN_A::EN
    }
}
#[doc = "Field `RX_EN` writer - RX channel enable."]
pub type RX_EN_W<'a, REG> = crate::BitWriter<'a, REG, RX_EN_A>;
impl<'a, REG> RX_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RX_EN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RX_EN_A::EN)
    }
}
impl R {
    #[doc = "Bit 0 - TX channel enable."]
    #[inline(always)]
    pub fn tx_en(&self) -> TX_EN_R {
        TX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX channel enable."]
    #[inline(always)]
    pub fn rx_en(&self) -> RX_EN_R {
        RX_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX channel enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_en(&mut self) -> TX_EN_W<DMA_SPEC> {
        TX_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - RX channel enable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_en(&mut self) -> RX_EN_W<DMA_SPEC> {
        RX_EN_W::new(self, 1)
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
#[doc = "DMA Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_SPEC;
impl crate::RegisterSpec for DMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma::R`](R) reader structure"]
impl crate::Readable for DMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma::W`](W) writer structure"]
impl crate::Writable for DMA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA to value 0"]
impl crate::Resettable for DMA_SPEC {
    const RESET_VALUE: u32 = 0;
}
