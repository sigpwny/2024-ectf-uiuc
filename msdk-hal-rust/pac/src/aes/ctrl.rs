#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `EN` reader - AES Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - AES Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_RX_EN` reader - DMA Request To Read Data Output FIFO"]
pub type DMA_RX_EN_R = crate::BitReader;
#[doc = "Field `DMA_RX_EN` writer - DMA Request To Read Data Output FIFO"]
pub type DMA_RX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_TX_EN` reader - DMA Request To Write Data Input FIFO"]
pub type DMA_TX_EN_R = crate::BitReader;
#[doc = "Field `DMA_TX_EN` writer - DMA Request To Write Data Input FIFO"]
pub type DMA_TX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Start AES Calculation"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Start AES Calculation"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUT_FLUSH` reader - Flush the data input FIFO"]
pub type INPUT_FLUSH_R = crate::BitReader;
#[doc = "Field `INPUT_FLUSH` writer - Flush the data input FIFO"]
pub type INPUT_FLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPUT_FLUSH` reader - Flush the data output FIFO"]
pub type OUTPUT_FLUSH_R = crate::BitReader;
#[doc = "Field `OUTPUT_FLUSH` writer - Flush the data output FIFO"]
pub type OUTPUT_FLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_SIZE` reader - Encryption Key Size"]
pub type KEY_SIZE_R = crate::FieldReader<KEY_SIZE_A>;
#[doc = "Encryption Key Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_SIZE_A {
    #[doc = "0: 128 Bits."]
    AES128 = 0,
    #[doc = "1: 192 Bits."]
    AES192 = 1,
    #[doc = "2: 256 Bits."]
    AES256 = 2,
}
impl From<KEY_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_SIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEY_SIZE_A {
    type Ux = u8;
}
impl KEY_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<KEY_SIZE_A> {
        match self.bits {
            0 => Some(KEY_SIZE_A::AES128),
            1 => Some(KEY_SIZE_A::AES192),
            2 => Some(KEY_SIZE_A::AES256),
            _ => None,
        }
    }
    #[doc = "128 Bits."]
    #[inline(always)]
    pub fn is_aes128(&self) -> bool {
        *self == KEY_SIZE_A::AES128
    }
    #[doc = "192 Bits."]
    #[inline(always)]
    pub fn is_aes192(&self) -> bool {
        *self == KEY_SIZE_A::AES192
    }
    #[doc = "256 Bits."]
    #[inline(always)]
    pub fn is_aes256(&self) -> bool {
        *self == KEY_SIZE_A::AES256
    }
}
#[doc = "Field `KEY_SIZE` writer - Encryption Key Size"]
pub type KEY_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, KEY_SIZE_A>;
impl<'a, REG> KEY_SIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128 Bits."]
    #[inline(always)]
    pub fn aes128(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_SIZE_A::AES128)
    }
    #[doc = "192 Bits."]
    #[inline(always)]
    pub fn aes192(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_SIZE_A::AES192)
    }
    #[doc = "256 Bits."]
    #[inline(always)]
    pub fn aes256(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_SIZE_A::AES256)
    }
}
#[doc = "Field `TYPE` reader - Encryption Type Selection"]
pub type TYPE_R = crate::FieldReader;
#[doc = "Field `TYPE` writer - Encryption Type Selection"]
pub type TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - AES Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Request To Read Data Output FIFO"]
    #[inline(always)]
    pub fn dma_rx_en(&self) -> DMA_RX_EN_R {
        DMA_RX_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Request To Write Data Input FIFO"]
    #[inline(always)]
    pub fn dma_tx_en(&self) -> DMA_TX_EN_R {
        DMA_TX_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start AES Calculation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flush the data input FIFO"]
    #[inline(always)]
    pub fn input_flush(&self) -> INPUT_FLUSH_R {
        INPUT_FLUSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flush the data output FIFO"]
    #[inline(always)]
    pub fn output_flush(&self) -> OUTPUT_FLUSH_R {
        OUTPUT_FLUSH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Encryption Key Size"]
    #[inline(always)]
    pub fn key_size(&self) -> KEY_SIZE_R {
        KEY_SIZE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Encryption Type Selection"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AES Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CTRL_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Request To Read Data Output FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_en(&mut self) -> DMA_RX_EN_W<CTRL_SPEC> {
        DMA_RX_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA Request To Write Data Input FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn dma_tx_en(&mut self) -> DMA_TX_EN_W<CTRL_SPEC> {
        DMA_TX_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Start AES Calculation"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CTRL_SPEC> {
        START_W::new(self, 3)
    }
    #[doc = "Bit 4 - Flush the data input FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn input_flush(&mut self) -> INPUT_FLUSH_W<CTRL_SPEC> {
        INPUT_FLUSH_W::new(self, 4)
    }
    #[doc = "Bit 5 - Flush the data output FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn output_flush(&mut self) -> OUTPUT_FLUSH_W<CTRL_SPEC> {
        OUTPUT_FLUSH_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Encryption Key Size"]
    #[inline(always)]
    #[must_use]
    pub fn key_size(&mut self) -> KEY_SIZE_W<CTRL_SPEC> {
        KEY_SIZE_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Encryption Type Selection"]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TYPE_W<CTRL_SPEC> {
        TYPE_W::new(self, 8)
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
#[doc = "AES Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
