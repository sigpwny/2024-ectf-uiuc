#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `EN` reader - CRC Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - CRC Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_EN` reader - DMA Request Enable"]
pub type DMA_EN_R = crate::BitReader;
#[doc = "Field `DMA_EN` writer - DMA Request Enable"]
pub type DMA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSB` reader - MSB Select"]
pub type MSB_R = crate::BitReader;
#[doc = "Field `MSB` writer - MSB Select"]
pub type MSB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTE_SWAP_IN` reader - Byte Swap CRC Data Input"]
pub type BYTE_SWAP_IN_R = crate::BitReader;
#[doc = "Field `BYTE_SWAP_IN` writer - Byte Swap CRC Data Input"]
pub type BYTE_SWAP_IN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTE_SWAP_OUT` reader - Byte Swap CRC Value Output"]
pub type BYTE_SWAP_OUT_R = crate::BitReader;
#[doc = "Field `BYTE_SWAP_OUT` writer - Byte Swap CRC Value Output"]
pub type BYTE_SWAP_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - CRC Busy"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `BUSY` writer - CRC Busy"]
pub type BUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CRC Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Request Enable"]
    #[inline(always)]
    pub fn dma_en(&self) -> DMA_EN_R {
        DMA_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSB Select"]
    #[inline(always)]
    pub fn msb(&self) -> MSB_R {
        MSB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Byte Swap CRC Data Input"]
    #[inline(always)]
    pub fn byte_swap_in(&self) -> BYTE_SWAP_IN_R {
        BYTE_SWAP_IN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Byte Swap CRC Value Output"]
    #[inline(always)]
    pub fn byte_swap_out(&self) -> BYTE_SWAP_OUT_R {
        BYTE_SWAP_OUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - CRC Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CTRL_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_en(&mut self) -> DMA_EN_W<CTRL_SPEC> {
        DMA_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - MSB Select"]
    #[inline(always)]
    #[must_use]
    pub fn msb(&mut self) -> MSB_W<CTRL_SPEC> {
        MSB_W::new(self, 2)
    }
    #[doc = "Bit 3 - Byte Swap CRC Data Input"]
    #[inline(always)]
    #[must_use]
    pub fn byte_swap_in(&mut self) -> BYTE_SWAP_IN_W<CTRL_SPEC> {
        BYTE_SWAP_IN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Byte Swap CRC Value Output"]
    #[inline(always)]
    #[must_use]
    pub fn byte_swap_out(&mut self) -> BYTE_SWAP_OUT_W<CTRL_SPEC> {
        BYTE_SWAP_OUT_W::new(self, 4)
    }
    #[doc = "Bit 16 - CRC Busy"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<CTRL_SPEC> {
        BUSY_W::new(self, 16)
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
#[doc = "CRC Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
