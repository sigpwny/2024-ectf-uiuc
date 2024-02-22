#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `READ_MODE` reader - Read Mode."]
pub type READ_MODE_R = crate::FieldReader<READ_MODE_A>;
#[doc = "Read Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum READ_MODE_A {
    #[doc = "0: Camera Interface Disabled."]
    DIS = 0,
    #[doc = "1: Single Image Capture."]
    SINGLE_IMG = 1,
    #[doc = "2: Continuous Image Capture."]
    CONTINUOUS = 2,
}
impl From<READ_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: READ_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for READ_MODE_A {
    type Ux = u8;
}
impl READ_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<READ_MODE_A> {
        match self.bits {
            0 => Some(READ_MODE_A::DIS),
            1 => Some(READ_MODE_A::SINGLE_IMG),
            2 => Some(READ_MODE_A::CONTINUOUS),
            _ => None,
        }
    }
    #[doc = "Camera Interface Disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == READ_MODE_A::DIS
    }
    #[doc = "Single Image Capture."]
    #[inline(always)]
    pub fn is_single_img(&self) -> bool {
        *self == READ_MODE_A::SINGLE_IMG
    }
    #[doc = "Continuous Image Capture."]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == READ_MODE_A::CONTINUOUS
    }
}
#[doc = "Field `READ_MODE` writer - Read Mode."]
pub type READ_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, READ_MODE_A>;
impl<'a, REG> READ_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Camera Interface Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(READ_MODE_A::DIS)
    }
    #[doc = "Single Image Capture."]
    #[inline(always)]
    pub fn single_img(self) -> &'a mut crate::W<REG> {
        self.variant(READ_MODE_A::SINGLE_IMG)
    }
    #[doc = "Continuous Image Capture."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(READ_MODE_A::CONTINUOUS)
    }
}
#[doc = "Field `DATA_WIDTH` reader - Data Width."]
pub type DATA_WIDTH_R = crate::FieldReader<DATA_WIDTH_A>;
#[doc = "Data Width.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATA_WIDTH_A {
    #[doc = "0: 8 bit."]
    _8BIT = 0,
    #[doc = "1: 10 bit."]
    _10BIT = 1,
    #[doc = "2: 12 bit."]
    _12BIT = 2,
}
impl From<DATA_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DATA_WIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATA_WIDTH_A {
    type Ux = u8;
}
impl DATA_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DATA_WIDTH_A> {
        match self.bits {
            0 => Some(DATA_WIDTH_A::_8BIT),
            1 => Some(DATA_WIDTH_A::_10BIT),
            2 => Some(DATA_WIDTH_A::_12BIT),
            _ => None,
        }
    }
    #[doc = "8 bit."]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == DATA_WIDTH_A::_8BIT
    }
    #[doc = "10 bit."]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == DATA_WIDTH_A::_10BIT
    }
    #[doc = "12 bit."]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == DATA_WIDTH_A::_12BIT
    }
}
#[doc = "Field `DATA_WIDTH` writer - Data Width."]
pub type DATA_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DATA_WIDTH_A>;
impl<'a, REG> DATA_WIDTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bit."]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_WIDTH_A::_8BIT)
    }
    #[doc = "10 bit."]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_WIDTH_A::_10BIT)
    }
    #[doc = "12 bit."]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_WIDTH_A::_12BIT)
    }
}
#[doc = "Field `DS_TIMING_EN` reader - DS Timing Enable."]
pub type DS_TIMING_EN_R = crate::BitReader<DS_TIMING_EN_A>;
#[doc = "DS Timing Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DS_TIMING_EN_A {
    #[doc = "0: Timing from VSYNC and HSYNC."]
    DIS = 0,
    #[doc = "1: Timing embedded in data using SAV and EAV codes."]
    EN = 1,
}
impl From<DS_TIMING_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DS_TIMING_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DS_TIMING_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DS_TIMING_EN_A {
        match self.bits {
            false => DS_TIMING_EN_A::DIS,
            true => DS_TIMING_EN_A::EN,
        }
    }
    #[doc = "Timing from VSYNC and HSYNC."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DS_TIMING_EN_A::DIS
    }
    #[doc = "Timing embedded in data using SAV and EAV codes."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DS_TIMING_EN_A::EN
    }
}
#[doc = "Field `DS_TIMING_EN` writer - DS Timing Enable."]
pub type DS_TIMING_EN_W<'a, REG> = crate::BitWriter<'a, REG, DS_TIMING_EN_A>;
impl<'a, REG> DS_TIMING_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timing from VSYNC and HSYNC."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(DS_TIMING_EN_A::DIS)
    }
    #[doc = "Timing embedded in data using SAV and EAV codes."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(DS_TIMING_EN_A::EN)
    }
}
#[doc = "Field `FIFO_THRSH` reader - Data FIFO Threshold."]
pub type FIFO_THRSH_R = crate::FieldReader;
#[doc = "Field `FIFO_THRSH` writer - Data FIFO Threshold."]
pub type FIFO_THRSH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RX_DMA` reader - DMA Enable."]
pub type RX_DMA_R = crate::BitReader<RX_DMA_A>;
#[doc = "DMA Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_DMA_A {
    #[doc = "0: DMA disabled."]
    DIS = 0,
    #[doc = "1: DMA enabled."]
    EN = 1,
}
impl From<RX_DMA_A> for bool {
    #[inline(always)]
    fn from(variant: RX_DMA_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_DMA_A {
        match self.bits {
            false => RX_DMA_A::DIS,
            true => RX_DMA_A::EN,
        }
    }
    #[doc = "DMA disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RX_DMA_A::DIS
    }
    #[doc = "DMA enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RX_DMA_A::EN
    }
}
#[doc = "Field `RX_DMA` writer - DMA Enable."]
pub type RX_DMA_W<'a, REG> = crate::BitWriter<'a, REG, RX_DMA_A>;
impl<'a, REG> RX_DMA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RX_DMA_A::DIS)
    }
    #[doc = "DMA enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RX_DMA_A::EN)
    }
}
#[doc = "Field `RX_DMA_THRSH` reader - DMA Threshold."]
pub type RX_DMA_THRSH_R = crate::FieldReader;
#[doc = "Field `RX_DMA_THRSH` writer - DMA Threshold."]
pub type RX_DMA_THRSH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `THREE_CH_EN` reader - Three-channel mode enable."]
pub type THREE_CH_EN_R = crate::BitReader;
#[doc = "Field `THREE_CH_EN` writer - Three-channel mode enable."]
pub type THREE_CH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCIF_SYS` reader - PCIF Control."]
pub type PCIF_SYS_R = crate::BitReader<PCIF_SYS_A>;
#[doc = "PCIF Control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCIF_SYS_A {
    #[doc = "0: PCIF disabled."]
    DIS = 0,
    #[doc = "1: PCIF enabled."]
    EN = 1,
}
impl From<PCIF_SYS_A> for bool {
    #[inline(always)]
    fn from(variant: PCIF_SYS_A) -> Self {
        variant as u8 != 0
    }
}
impl PCIF_SYS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCIF_SYS_A {
        match self.bits {
            false => PCIF_SYS_A::DIS,
            true => PCIF_SYS_A::EN,
        }
    }
    #[doc = "PCIF disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PCIF_SYS_A::DIS
    }
    #[doc = "PCIF enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PCIF_SYS_A::EN
    }
}
#[doc = "Field `PCIF_SYS` writer - PCIF Control."]
pub type PCIF_SYS_W<'a, REG> = crate::BitWriter<'a, REG, PCIF_SYS_A>;
impl<'a, REG> PCIF_SYS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PCIF disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(PCIF_SYS_A::DIS)
    }
    #[doc = "PCIF enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(PCIF_SYS_A::EN)
    }
}
impl R {
    #[doc = "Bits 0:1 - Read Mode."]
    #[inline(always)]
    pub fn read_mode(&self) -> READ_MODE_R {
        READ_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Data Width."]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - DS Timing Enable."]
    #[inline(always)]
    pub fn ds_timing_en(&self) -> DS_TIMING_EN_R {
        DS_TIMING_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:9 - Data FIFO Threshold."]
    #[inline(always)]
    pub fn fifo_thrsh(&self) -> FIFO_THRSH_R {
        FIFO_THRSH_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - DMA Enable."]
    #[inline(always)]
    pub fn rx_dma(&self) -> RX_DMA_R {
        RX_DMA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:20 - DMA Threshold."]
    #[inline(always)]
    pub fn rx_dma_thrsh(&self) -> RX_DMA_THRSH_R {
        RX_DMA_THRSH_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Three-channel mode enable."]
    #[inline(always)]
    pub fn three_ch_en(&self) -> THREE_CH_EN_R {
        THREE_CH_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PCIF Control."]
    #[inline(always)]
    pub fn pcif_sys(&self) -> PCIF_SYS_R {
        PCIF_SYS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Read Mode."]
    #[inline(always)]
    #[must_use]
    pub fn read_mode(&mut self) -> READ_MODE_W<CTRL_SPEC> {
        READ_MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Data Width."]
    #[inline(always)]
    #[must_use]
    pub fn data_width(&mut self) -> DATA_WIDTH_W<CTRL_SPEC> {
        DATA_WIDTH_W::new(self, 2)
    }
    #[doc = "Bit 4 - DS Timing Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ds_timing_en(&mut self) -> DS_TIMING_EN_W<CTRL_SPEC> {
        DS_TIMING_EN_W::new(self, 4)
    }
    #[doc = "Bits 5:9 - Data FIFO Threshold."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_thrsh(&mut self) -> FIFO_THRSH_W<CTRL_SPEC> {
        FIFO_THRSH_W::new(self, 5)
    }
    #[doc = "Bit 16 - DMA Enable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_dma(&mut self) -> RX_DMA_W<CTRL_SPEC> {
        RX_DMA_W::new(self, 16)
    }
    #[doc = "Bits 17:20 - DMA Threshold."]
    #[inline(always)]
    #[must_use]
    pub fn rx_dma_thrsh(&mut self) -> RX_DMA_THRSH_W<CTRL_SPEC> {
        RX_DMA_THRSH_W::new(self, 17)
    }
    #[doc = "Bit 30 - Three-channel mode enable."]
    #[inline(always)]
    #[must_use]
    pub fn three_ch_en(&mut self) -> THREE_CH_EN_W<CTRL_SPEC> {
        THREE_CH_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - PCIF Control."]
    #[inline(always)]
    #[must_use]
    pub fn pcif_sys(&mut self) -> PCIF_SYS_W<CTRL_SPEC> {
        PCIF_SYS_W::new(self, 31)
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
#[doc = "Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
