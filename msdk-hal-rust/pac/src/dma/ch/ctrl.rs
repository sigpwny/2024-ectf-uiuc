#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `EN` reader - Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0."]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DIS,
            true => EN_A::EN,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN_A::DIS
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN_A::EN
    }
}
#[doc = "Field `EN` writer - Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN_A>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::EN)
    }
}
#[doc = "Field `RLDEN` reader - Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed."]
pub type RLDEN_R = crate::BitReader<RLDEN_A>;
#[doc = "Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RLDEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<RLDEN_A> for bool {
    #[inline(always)]
    fn from(variant: RLDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RLDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RLDEN_A {
        match self.bits {
            false => RLDEN_A::DIS,
            true => RLDEN_A::EN,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RLDEN_A::DIS
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RLDEN_A::EN
    }
}
#[doc = "Field `RLDEN` writer - Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed."]
pub type RLDEN_W<'a, REG> = crate::BitWriter<'a, REG, RLDEN_A>;
impl<'a, REG> RLDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RLDEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RLDEN_A::EN)
    }
}
#[doc = "Field `PRI` reader - DMA Priority."]
pub type PRI_R = crate::FieldReader<PRI_A>;
#[doc = "DMA Priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRI_A {
    #[doc = "0: Highest Priority."]
    HIGH = 0,
    #[doc = "1: Medium High Priority."]
    MED_HIGH = 1,
    #[doc = "2: Medium Low Priority."]
    MED_LOW = 2,
    #[doc = "3: Lowest Priority."]
    LOW = 3,
}
impl From<PRI_A> for u8 {
    #[inline(always)]
    fn from(variant: PRI_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRI_A {
    type Ux = u8;
}
impl PRI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRI_A {
        match self.bits {
            0 => PRI_A::HIGH,
            1 => PRI_A::MED_HIGH,
            2 => PRI_A::MED_LOW,
            3 => PRI_A::LOW,
            _ => unreachable!(),
        }
    }
    #[doc = "Highest Priority."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PRI_A::HIGH
    }
    #[doc = "Medium High Priority."]
    #[inline(always)]
    pub fn is_med_high(&self) -> bool {
        *self == PRI_A::MED_HIGH
    }
    #[doc = "Medium Low Priority."]
    #[inline(always)]
    pub fn is_med_low(&self) -> bool {
        *self == PRI_A::MED_LOW
    }
    #[doc = "Lowest Priority."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PRI_A::LOW
    }
}
#[doc = "Field `PRI` writer - DMA Priority."]
pub type PRI_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PRI_A>;
impl<'a, REG> PRI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Highest Priority."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(PRI_A::HIGH)
    }
    #[doc = "Medium High Priority."]
    #[inline(always)]
    pub fn med_high(self) -> &'a mut crate::W<REG> {
        self.variant(PRI_A::MED_HIGH)
    }
    #[doc = "Medium Low Priority."]
    #[inline(always)]
    pub fn med_low(self) -> &'a mut crate::W<REG> {
        self.variant(PRI_A::MED_LOW)
    }
    #[doc = "Lowest Priority."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(PRI_A::LOW)
    }
}
#[doc = "Field `REQUEST` reader - Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active."]
pub type REQUEST_R = crate::FieldReader<REQUEST_A>;
#[doc = "Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REQUEST_A {
    #[doc = "0: Memory To Memory"]
    MEMTOMEM = 0,
    #[doc = "1: SPI1 RX"]
    SPI1RX = 1,
    #[doc = "4: UART0 RX"]
    UART0RX = 4,
    #[doc = "5: UART1 RX"]
    UART1RX = 5,
    #[doc = "7: I2C0 RX"]
    I2C0RX = 7,
    #[doc = "8: I2C1 RX"]
    I2C1RX = 8,
    #[doc = "9: ADC"]
    ADC = 9,
    #[doc = "10: I2C2 RX"]
    I2C2RX = 10,
    #[doc = "14: UART2 RX"]
    UART2RX = 14,
    #[doc = "15: SPI0 RX"]
    SPI0RX = 15,
    #[doc = "16: AES RX"]
    AESRX = 16,
    #[doc = "28: UART3 RX"]
    UART3RX = 28,
    #[doc = "30: I2S RX"]
    I2SRX = 30,
    #[doc = "33: SPI1 TX"]
    SPI1TX = 33,
    #[doc = "36: UART0 TX"]
    UART0TX = 36,
    #[doc = "37: UART1 TX"]
    UART1TX = 37,
    #[doc = "39: I2C0 TX"]
    I2C0TX = 39,
    #[doc = "40: I2C1 TX"]
    I2C1TX = 40,
    #[doc = "42: I2C2 TX"]
    I2C2TX = 42,
    #[doc = "44: CRC TX"]
    CRCTX = 44,
    #[doc = "45: PCIF TX"]
    PCIFTX = 45,
    #[doc = "46: UART2 TX"]
    UART2TX = 46,
    #[doc = "47: SPI0 TX"]
    SPI0TX = 47,
    #[doc = "48: AES TX"]
    AESTX = 48,
    #[doc = "60: UART3 TX"]
    UART3TX = 60,
    #[doc = "62: I2S TX"]
    I2STX = 62,
}
impl From<REQUEST_A> for u8 {
    #[inline(always)]
    fn from(variant: REQUEST_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REQUEST_A {
    type Ux = u8;
}
impl REQUEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<REQUEST_A> {
        match self.bits {
            0 => Some(REQUEST_A::MEMTOMEM),
            1 => Some(REQUEST_A::SPI1RX),
            4 => Some(REQUEST_A::UART0RX),
            5 => Some(REQUEST_A::UART1RX),
            7 => Some(REQUEST_A::I2C0RX),
            8 => Some(REQUEST_A::I2C1RX),
            9 => Some(REQUEST_A::ADC),
            10 => Some(REQUEST_A::I2C2RX),
            14 => Some(REQUEST_A::UART2RX),
            15 => Some(REQUEST_A::SPI0RX),
            16 => Some(REQUEST_A::AESRX),
            28 => Some(REQUEST_A::UART3RX),
            30 => Some(REQUEST_A::I2SRX),
            33 => Some(REQUEST_A::SPI1TX),
            36 => Some(REQUEST_A::UART0TX),
            37 => Some(REQUEST_A::UART1TX),
            39 => Some(REQUEST_A::I2C0TX),
            40 => Some(REQUEST_A::I2C1TX),
            42 => Some(REQUEST_A::I2C2TX),
            44 => Some(REQUEST_A::CRCTX),
            45 => Some(REQUEST_A::PCIFTX),
            46 => Some(REQUEST_A::UART2TX),
            47 => Some(REQUEST_A::SPI0TX),
            48 => Some(REQUEST_A::AESTX),
            60 => Some(REQUEST_A::UART3TX),
            62 => Some(REQUEST_A::I2STX),
            _ => None,
        }
    }
    #[doc = "Memory To Memory"]
    #[inline(always)]
    pub fn is_memtomem(&self) -> bool {
        *self == REQUEST_A::MEMTOMEM
    }
    #[doc = "SPI1 RX"]
    #[inline(always)]
    pub fn is_spi1rx(&self) -> bool {
        *self == REQUEST_A::SPI1RX
    }
    #[doc = "UART0 RX"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        *self == REQUEST_A::UART0RX
    }
    #[doc = "UART1 RX"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        *self == REQUEST_A::UART1RX
    }
    #[doc = "I2C0 RX"]
    #[inline(always)]
    pub fn is_i2c0rx(&self) -> bool {
        *self == REQUEST_A::I2C0RX
    }
    #[doc = "I2C1 RX"]
    #[inline(always)]
    pub fn is_i2c1rx(&self) -> bool {
        *self == REQUEST_A::I2C1RX
    }
    #[doc = "ADC"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == REQUEST_A::ADC
    }
    #[doc = "I2C2 RX"]
    #[inline(always)]
    pub fn is_i2c2rx(&self) -> bool {
        *self == REQUEST_A::I2C2RX
    }
    #[doc = "UART2 RX"]
    #[inline(always)]
    pub fn is_uart2rx(&self) -> bool {
        *self == REQUEST_A::UART2RX
    }
    #[doc = "SPI0 RX"]
    #[inline(always)]
    pub fn is_spi0rx(&self) -> bool {
        *self == REQUEST_A::SPI0RX
    }
    #[doc = "AES RX"]
    #[inline(always)]
    pub fn is_aesrx(&self) -> bool {
        *self == REQUEST_A::AESRX
    }
    #[doc = "UART3 RX"]
    #[inline(always)]
    pub fn is_uart3rx(&self) -> bool {
        *self == REQUEST_A::UART3RX
    }
    #[doc = "I2S RX"]
    #[inline(always)]
    pub fn is_i2srx(&self) -> bool {
        *self == REQUEST_A::I2SRX
    }
    #[doc = "SPI1 TX"]
    #[inline(always)]
    pub fn is_spi1tx(&self) -> bool {
        *self == REQUEST_A::SPI1TX
    }
    #[doc = "UART0 TX"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        *self == REQUEST_A::UART0TX
    }
    #[doc = "UART1 TX"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        *self == REQUEST_A::UART1TX
    }
    #[doc = "I2C0 TX"]
    #[inline(always)]
    pub fn is_i2c0tx(&self) -> bool {
        *self == REQUEST_A::I2C0TX
    }
    #[doc = "I2C1 TX"]
    #[inline(always)]
    pub fn is_i2c1tx(&self) -> bool {
        *self == REQUEST_A::I2C1TX
    }
    #[doc = "I2C2 TX"]
    #[inline(always)]
    pub fn is_i2c2tx(&self) -> bool {
        *self == REQUEST_A::I2C2TX
    }
    #[doc = "CRC TX"]
    #[inline(always)]
    pub fn is_crctx(&self) -> bool {
        *self == REQUEST_A::CRCTX
    }
    #[doc = "PCIF TX"]
    #[inline(always)]
    pub fn is_pciftx(&self) -> bool {
        *self == REQUEST_A::PCIFTX
    }
    #[doc = "UART2 TX"]
    #[inline(always)]
    pub fn is_uart2tx(&self) -> bool {
        *self == REQUEST_A::UART2TX
    }
    #[doc = "SPI0 TX"]
    #[inline(always)]
    pub fn is_spi0tx(&self) -> bool {
        *self == REQUEST_A::SPI0TX
    }
    #[doc = "AES TX"]
    #[inline(always)]
    pub fn is_aestx(&self) -> bool {
        *self == REQUEST_A::AESTX
    }
    #[doc = "UART3 TX"]
    #[inline(always)]
    pub fn is_uart3tx(&self) -> bool {
        *self == REQUEST_A::UART3TX
    }
    #[doc = "I2S TX"]
    #[inline(always)]
    pub fn is_i2stx(&self) -> bool {
        *self == REQUEST_A::I2STX
    }
}
#[doc = "Field `REQUEST` writer - Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active."]
pub type REQUEST_W<'a, REG> = crate::FieldWriter<'a, REG, 6, REQUEST_A>;
impl<'a, REG> REQUEST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Memory To Memory"]
    #[inline(always)]
    pub fn memtomem(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::MEMTOMEM)
    }
    #[doc = "SPI1 RX"]
    #[inline(always)]
    pub fn spi1rx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::SPI1RX)
    }
    #[doc = "UART0 RX"]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::UART0RX)
    }
    #[doc = "UART1 RX"]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::UART1RX)
    }
    #[doc = "I2C0 RX"]
    #[inline(always)]
    pub fn i2c0rx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::I2C0RX)
    }
    #[doc = "I2C1 RX"]
    #[inline(always)]
    pub fn i2c1rx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::I2C1RX)
    }
    #[doc = "ADC"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::ADC)
    }
    #[doc = "I2C2 RX"]
    #[inline(always)]
    pub fn i2c2rx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::I2C2RX)
    }
    #[doc = "UART2 RX"]
    #[inline(always)]
    pub fn uart2rx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::UART2RX)
    }
    #[doc = "SPI0 RX"]
    #[inline(always)]
    pub fn spi0rx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::SPI0RX)
    }
    #[doc = "AES RX"]
    #[inline(always)]
    pub fn aesrx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::AESRX)
    }
    #[doc = "UART3 RX"]
    #[inline(always)]
    pub fn uart3rx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::UART3RX)
    }
    #[doc = "I2S RX"]
    #[inline(always)]
    pub fn i2srx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::I2SRX)
    }
    #[doc = "SPI1 TX"]
    #[inline(always)]
    pub fn spi1tx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::SPI1TX)
    }
    #[doc = "UART0 TX"]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::UART0TX)
    }
    #[doc = "UART1 TX"]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::UART1TX)
    }
    #[doc = "I2C0 TX"]
    #[inline(always)]
    pub fn i2c0tx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::I2C0TX)
    }
    #[doc = "I2C1 TX"]
    #[inline(always)]
    pub fn i2c1tx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::I2C1TX)
    }
    #[doc = "I2C2 TX"]
    #[inline(always)]
    pub fn i2c2tx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::I2C2TX)
    }
    #[doc = "CRC TX"]
    #[inline(always)]
    pub fn crctx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::CRCTX)
    }
    #[doc = "PCIF TX"]
    #[inline(always)]
    pub fn pciftx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::PCIFTX)
    }
    #[doc = "UART2 TX"]
    #[inline(always)]
    pub fn uart2tx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::UART2TX)
    }
    #[doc = "SPI0 TX"]
    #[inline(always)]
    pub fn spi0tx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::SPI0TX)
    }
    #[doc = "AES TX"]
    #[inline(always)]
    pub fn aestx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::AESTX)
    }
    #[doc = "UART3 TX"]
    #[inline(always)]
    pub fn uart3tx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::UART3TX)
    }
    #[doc = "I2S TX"]
    #[inline(always)]
    pub fn i2stx(self) -> &'a mut crate::W<REG> {
        self.variant(REQUEST_A::I2STX)
    }
}
#[doc = "Field `TO_WAIT` reader - Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive."]
pub type TO_WAIT_R = crate::BitReader<TO_WAIT_A>;
#[doc = "Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TO_WAIT_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<TO_WAIT_A> for bool {
    #[inline(always)]
    fn from(variant: TO_WAIT_A) -> Self {
        variant as u8 != 0
    }
}
impl TO_WAIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TO_WAIT_A {
        match self.bits {
            false => TO_WAIT_A::DIS,
            true => TO_WAIT_A::EN,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TO_WAIT_A::DIS
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TO_WAIT_A::EN
    }
}
#[doc = "Field `TO_WAIT` writer - Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive."]
pub type TO_WAIT_W<'a, REG> = crate::BitWriter<'a, REG, TO_WAIT_A>;
impl<'a, REG> TO_WAIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TO_WAIT_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TO_WAIT_A::EN)
    }
}
#[doc = "Field `TO_PER` reader - Timeout Period Select."]
pub type TO_PER_R = crate::FieldReader<TO_PER_A>;
#[doc = "Timeout Period Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TO_PER_A {
    #[doc = "0: Timeout of 3 to 4 prescale clocks."]
    TO4 = 0,
    #[doc = "1: Timeout of 7 to 8 prescale clocks."]
    TO8 = 1,
    #[doc = "2: Timeout of 15 to 16 prescale clocks."]
    TO16 = 2,
    #[doc = "3: Timeout of 31 to 32 prescale clocks."]
    TO32 = 3,
    #[doc = "4: Timeout of 63 to 64 prescale clocks."]
    TO64 = 4,
    #[doc = "5: Timeout of 127 to 128 prescale clocks."]
    TO128 = 5,
    #[doc = "6: Timeout of 255 to 256 prescale clocks."]
    TO256 = 6,
    #[doc = "7: Timeout of 511 to 512 prescale clocks."]
    TO512 = 7,
}
impl From<TO_PER_A> for u8 {
    #[inline(always)]
    fn from(variant: TO_PER_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TO_PER_A {
    type Ux = u8;
}
impl TO_PER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TO_PER_A {
        match self.bits {
            0 => TO_PER_A::TO4,
            1 => TO_PER_A::TO8,
            2 => TO_PER_A::TO16,
            3 => TO_PER_A::TO32,
            4 => TO_PER_A::TO64,
            5 => TO_PER_A::TO128,
            6 => TO_PER_A::TO256,
            7 => TO_PER_A::TO512,
            _ => unreachable!(),
        }
    }
    #[doc = "Timeout of 3 to 4 prescale clocks."]
    #[inline(always)]
    pub fn is_to4(&self) -> bool {
        *self == TO_PER_A::TO4
    }
    #[doc = "Timeout of 7 to 8 prescale clocks."]
    #[inline(always)]
    pub fn is_to8(&self) -> bool {
        *self == TO_PER_A::TO8
    }
    #[doc = "Timeout of 15 to 16 prescale clocks."]
    #[inline(always)]
    pub fn is_to16(&self) -> bool {
        *self == TO_PER_A::TO16
    }
    #[doc = "Timeout of 31 to 32 prescale clocks."]
    #[inline(always)]
    pub fn is_to32(&self) -> bool {
        *self == TO_PER_A::TO32
    }
    #[doc = "Timeout of 63 to 64 prescale clocks."]
    #[inline(always)]
    pub fn is_to64(&self) -> bool {
        *self == TO_PER_A::TO64
    }
    #[doc = "Timeout of 127 to 128 prescale clocks."]
    #[inline(always)]
    pub fn is_to128(&self) -> bool {
        *self == TO_PER_A::TO128
    }
    #[doc = "Timeout of 255 to 256 prescale clocks."]
    #[inline(always)]
    pub fn is_to256(&self) -> bool {
        *self == TO_PER_A::TO256
    }
    #[doc = "Timeout of 511 to 512 prescale clocks."]
    #[inline(always)]
    pub fn is_to512(&self) -> bool {
        *self == TO_PER_A::TO512
    }
}
#[doc = "Field `TO_PER` writer - Timeout Period Select."]
pub type TO_PER_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, TO_PER_A>;
impl<'a, REG> TO_PER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout of 3 to 4 prescale clocks."]
    #[inline(always)]
    pub fn to4(self) -> &'a mut crate::W<REG> {
        self.variant(TO_PER_A::TO4)
    }
    #[doc = "Timeout of 7 to 8 prescale clocks."]
    #[inline(always)]
    pub fn to8(self) -> &'a mut crate::W<REG> {
        self.variant(TO_PER_A::TO8)
    }
    #[doc = "Timeout of 15 to 16 prescale clocks."]
    #[inline(always)]
    pub fn to16(self) -> &'a mut crate::W<REG> {
        self.variant(TO_PER_A::TO16)
    }
    #[doc = "Timeout of 31 to 32 prescale clocks."]
    #[inline(always)]
    pub fn to32(self) -> &'a mut crate::W<REG> {
        self.variant(TO_PER_A::TO32)
    }
    #[doc = "Timeout of 63 to 64 prescale clocks."]
    #[inline(always)]
    pub fn to64(self) -> &'a mut crate::W<REG> {
        self.variant(TO_PER_A::TO64)
    }
    #[doc = "Timeout of 127 to 128 prescale clocks."]
    #[inline(always)]
    pub fn to128(self) -> &'a mut crate::W<REG> {
        self.variant(TO_PER_A::TO128)
    }
    #[doc = "Timeout of 255 to 256 prescale clocks."]
    #[inline(always)]
    pub fn to256(self) -> &'a mut crate::W<REG> {
        self.variant(TO_PER_A::TO256)
    }
    #[doc = "Timeout of 511 to 512 prescale clocks."]
    #[inline(always)]
    pub fn to512(self) -> &'a mut crate::W<REG> {
        self.variant(TO_PER_A::TO512)
    }
}
#[doc = "Field `TO_CLKDIV` reader - Pre-Scale Select. Selects the Pre-Scale divider for timer clock input."]
pub type TO_CLKDIV_R = crate::FieldReader<TO_CLKDIV_A>;
#[doc = "Pre-Scale Select. Selects the Pre-Scale divider for timer clock input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TO_CLKDIV_A {
    #[doc = "0: Disable timer."]
    DIS = 0,
    #[doc = "1: hclk / 256."]
    DIV256 = 1,
    #[doc = "2: hclk / 64k."]
    DIV64K = 2,
    #[doc = "3: hclk / 16M."]
    DIV16M = 3,
}
impl From<TO_CLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: TO_CLKDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TO_CLKDIV_A {
    type Ux = u8;
}
impl TO_CLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TO_CLKDIV_A {
        match self.bits {
            0 => TO_CLKDIV_A::DIS,
            1 => TO_CLKDIV_A::DIV256,
            2 => TO_CLKDIV_A::DIV64K,
            3 => TO_CLKDIV_A::DIV16M,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable timer."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TO_CLKDIV_A::DIS
    }
    #[doc = "hclk / 256."]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == TO_CLKDIV_A::DIV256
    }
    #[doc = "hclk / 64k."]
    #[inline(always)]
    pub fn is_div64k(&self) -> bool {
        *self == TO_CLKDIV_A::DIV64K
    }
    #[doc = "hclk / 16M."]
    #[inline(always)]
    pub fn is_div16m(&self) -> bool {
        *self == TO_CLKDIV_A::DIV16M
    }
}
#[doc = "Field `TO_CLKDIV` writer - Pre-Scale Select. Selects the Pre-Scale divider for timer clock input."]
pub type TO_CLKDIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TO_CLKDIV_A>;
impl<'a, REG> TO_CLKDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable timer."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TO_CLKDIV_A::DIS)
    }
    #[doc = "hclk / 256."]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(TO_CLKDIV_A::DIV256)
    }
    #[doc = "hclk / 64k."]
    #[inline(always)]
    pub fn div64k(self) -> &'a mut crate::W<REG> {
        self.variant(TO_CLKDIV_A::DIV64K)
    }
    #[doc = "hclk / 16M."]
    #[inline(always)]
    pub fn div16m(self) -> &'a mut crate::W<REG> {
        self.variant(TO_CLKDIV_A::DIV16M)
    }
}
#[doc = "Field `SRCWD` reader - Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value."]
pub type SRCWD_R = crate::FieldReader<SRCWD_A>;
#[doc = "Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRCWD_A {
    #[doc = "0: Byte."]
    BYTE = 0,
    #[doc = "1: Halfword."]
    HALF_WORD = 1,
    #[doc = "2: Word."]
    WORD = 2,
}
impl From<SRCWD_A> for u8 {
    #[inline(always)]
    fn from(variant: SRCWD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SRCWD_A {
    type Ux = u8;
}
impl SRCWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SRCWD_A> {
        match self.bits {
            0 => Some(SRCWD_A::BYTE),
            1 => Some(SRCWD_A::HALF_WORD),
            2 => Some(SRCWD_A::WORD),
            _ => None,
        }
    }
    #[doc = "Byte."]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == SRCWD_A::BYTE
    }
    #[doc = "Halfword."]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == SRCWD_A::HALF_WORD
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == SRCWD_A::WORD
    }
}
#[doc = "Field `SRCWD` writer - Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value."]
pub type SRCWD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SRCWD_A>;
impl<'a, REG> SRCWD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Byte."]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(SRCWD_A::BYTE)
    }
    #[doc = "Halfword."]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut crate::W<REG> {
        self.variant(SRCWD_A::HALF_WORD)
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(SRCWD_A::WORD)
    }
}
#[doc = "Field `SRCINC` reader - Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals."]
pub type SRCINC_R = crate::BitReader<SRCINC_A>;
#[doc = "Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRCINC_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<SRCINC_A> for bool {
    #[inline(always)]
    fn from(variant: SRCINC_A) -> Self {
        variant as u8 != 0
    }
}
impl SRCINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRCINC_A {
        match self.bits {
            false => SRCINC_A::DIS,
            true => SRCINC_A::EN,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SRCINC_A::DIS
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SRCINC_A::EN
    }
}
#[doc = "Field `SRCINC` writer - Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals."]
pub type SRCINC_W<'a, REG> = crate::BitWriter<'a, REG, SRCINC_A>;
impl<'a, REG> SRCINC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(SRCINC_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(SRCINC_A::EN)
    }
}
#[doc = "Field `DSTWD` reader - Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width)."]
pub type DSTWD_R = crate::FieldReader<DSTWD_A>;
#[doc = "Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSTWD_A {
    #[doc = "0: Byte."]
    BYTE = 0,
    #[doc = "1: Halfword."]
    HALF_WORD = 1,
    #[doc = "2: Word."]
    WORD = 2,
}
impl From<DSTWD_A> for u8 {
    #[inline(always)]
    fn from(variant: DSTWD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DSTWD_A {
    type Ux = u8;
}
impl DSTWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DSTWD_A> {
        match self.bits {
            0 => Some(DSTWD_A::BYTE),
            1 => Some(DSTWD_A::HALF_WORD),
            2 => Some(DSTWD_A::WORD),
            _ => None,
        }
    }
    #[doc = "Byte."]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DSTWD_A::BYTE
    }
    #[doc = "Halfword."]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == DSTWD_A::HALF_WORD
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == DSTWD_A::WORD
    }
}
#[doc = "Field `DSTWD` writer - Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width)."]
pub type DSTWD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DSTWD_A>;
impl<'a, REG> DSTWD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Byte."]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(DSTWD_A::BYTE)
    }
    #[doc = "Halfword."]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut crate::W<REG> {
        self.variant(DSTWD_A::HALF_WORD)
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(DSTWD_A::WORD)
    }
}
#[doc = "Field `DSTINC` reader - Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals."]
pub type DSTINC_R = crate::BitReader<DSTINC_A>;
#[doc = "Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSTINC_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<DSTINC_A> for bool {
    #[inline(always)]
    fn from(variant: DSTINC_A) -> Self {
        variant as u8 != 0
    }
}
impl DSTINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DSTINC_A {
        match self.bits {
            false => DSTINC_A::DIS,
            true => DSTINC_A::EN,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DSTINC_A::DIS
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DSTINC_A::EN
    }
}
#[doc = "Field `DSTINC` writer - Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals."]
pub type DSTINC_W<'a, REG> = crate::BitWriter<'a, REG, DSTINC_A>;
impl<'a, REG> DSTINC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(DSTINC_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(DSTINC_A::EN)
    }
}
#[doc = "Field `BURST_SIZE` reader - Burst Size. The number of bytes to be transferred into and out of the DMA FIFO in a single burst. Burst size equals 1 + value stored in this field."]
pub type BURST_SIZE_R = crate::FieldReader;
#[doc = "Field `BURST_SIZE` writer - Burst Size. The number of bytes to be transferred into and out of the DMA FIFO in a single burst. Burst size equals 1 + value stored in this field."]
pub type BURST_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DIS_IE` reader - Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0."]
pub type DIS_IE_R = crate::BitReader<DIS_IE_A>;
#[doc = "Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIS_IE_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<DIS_IE_A> for bool {
    #[inline(always)]
    fn from(variant: DIS_IE_A) -> Self {
        variant as u8 != 0
    }
}
impl DIS_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIS_IE_A {
        match self.bits {
            false => DIS_IE_A::DIS,
            true => DIS_IE_A::EN,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DIS_IE_A::DIS
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DIS_IE_A::EN
    }
}
#[doc = "Field `DIS_IE` writer - Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0."]
pub type DIS_IE_W<'a, REG> = crate::BitWriter<'a, REG, DIS_IE_A>;
impl<'a, REG> DIS_IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(DIS_IE_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(DIS_IE_A::EN)
    }
}
#[doc = "Field `CTZ_IE` reader - Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs."]
pub type CTZ_IE_R = crate::BitReader<CTZ_IE_A>;
#[doc = "Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTZ_IE_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<CTZ_IE_A> for bool {
    #[inline(always)]
    fn from(variant: CTZ_IE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTZ_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTZ_IE_A {
        match self.bits {
            false => CTZ_IE_A::DIS,
            true => CTZ_IE_A::EN,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CTZ_IE_A::DIS
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CTZ_IE_A::EN
    }
}
#[doc = "Field `CTZ_IE` writer - Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs."]
pub type CTZ_IE_W<'a, REG> = crate::BitWriter<'a, REG, CTZ_IE_A>;
impl<'a, REG> CTZ_IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(CTZ_IE_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(CTZ_IE_A::EN)
    }
}
impl R {
    #[doc = "Bit 0 - Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed."]
    #[inline(always)]
    pub fn rlden(&self) -> RLDEN_R {
        RLDEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - DMA Priority."]
    #[inline(always)]
    pub fn pri(&self) -> PRI_R {
        PRI_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:9 - Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active."]
    #[inline(always)]
    pub fn request(&self) -> REQUEST_R {
        REQUEST_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bit 10 - Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive."]
    #[inline(always)]
    pub fn to_wait(&self) -> TO_WAIT_R {
        TO_WAIT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Timeout Period Select."]
    #[inline(always)]
    pub fn to_per(&self) -> TO_PER_R {
        TO_PER_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - Pre-Scale Select. Selects the Pre-Scale divider for timer clock input."]
    #[inline(always)]
    pub fn to_clkdiv(&self) -> TO_CLKDIV_R {
        TO_CLKDIV_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value."]
    #[inline(always)]
    pub fn srcwd(&self) -> SRCWD_R {
        SRCWD_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals."]
    #[inline(always)]
    pub fn srcinc(&self) -> SRCINC_R {
        SRCINC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width)."]
    #[inline(always)]
    pub fn dstwd(&self) -> DSTWD_R {
        DSTWD_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals."]
    #[inline(always)]
    pub fn dstinc(&self) -> DSTINC_R {
        DSTINC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Burst Size. The number of bytes to be transferred into and out of the DMA FIFO in a single burst. Burst size equals 1 + value stored in this field."]
    #[inline(always)]
    pub fn burst_size(&self) -> BURST_SIZE_R {
        BURST_SIZE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0."]
    #[inline(always)]
    pub fn dis_ie(&self) -> DIS_IE_R {
        DIS_IE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs."]
    #[inline(always)]
    pub fn ctz_ie(&self) -> CTZ_IE_R {
        CTZ_IE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CTRL_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed."]
    #[inline(always)]
    #[must_use]
    pub fn rlden(&mut self) -> RLDEN_W<CTRL_SPEC> {
        RLDEN_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - DMA Priority."]
    #[inline(always)]
    #[must_use]
    pub fn pri(&mut self) -> PRI_W<CTRL_SPEC> {
        PRI_W::new(self, 2)
    }
    #[doc = "Bits 4:9 - Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active."]
    #[inline(always)]
    #[must_use]
    pub fn request(&mut self) -> REQUEST_W<CTRL_SPEC> {
        REQUEST_W::new(self, 4)
    }
    #[doc = "Bit 10 - Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive."]
    #[inline(always)]
    #[must_use]
    pub fn to_wait(&mut self) -> TO_WAIT_W<CTRL_SPEC> {
        TO_WAIT_W::new(self, 10)
    }
    #[doc = "Bits 11:13 - Timeout Period Select."]
    #[inline(always)]
    #[must_use]
    pub fn to_per(&mut self) -> TO_PER_W<CTRL_SPEC> {
        TO_PER_W::new(self, 11)
    }
    #[doc = "Bits 14:15 - Pre-Scale Select. Selects the Pre-Scale divider for timer clock input."]
    #[inline(always)]
    #[must_use]
    pub fn to_clkdiv(&mut self) -> TO_CLKDIV_W<CTRL_SPEC> {
        TO_CLKDIV_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value."]
    #[inline(always)]
    #[must_use]
    pub fn srcwd(&mut self) -> SRCWD_W<CTRL_SPEC> {
        SRCWD_W::new(self, 16)
    }
    #[doc = "Bit 18 - Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals."]
    #[inline(always)]
    #[must_use]
    pub fn srcinc(&mut self) -> SRCINC_W<CTRL_SPEC> {
        SRCINC_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width)."]
    #[inline(always)]
    #[must_use]
    pub fn dstwd(&mut self) -> DSTWD_W<CTRL_SPEC> {
        DSTWD_W::new(self, 20)
    }
    #[doc = "Bit 22 - Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals."]
    #[inline(always)]
    #[must_use]
    pub fn dstinc(&mut self) -> DSTINC_W<CTRL_SPEC> {
        DSTINC_W::new(self, 22)
    }
    #[doc = "Bits 24:28 - Burst Size. The number of bytes to be transferred into and out of the DMA FIFO in a single burst. Burst size equals 1 + value stored in this field."]
    #[inline(always)]
    #[must_use]
    pub fn burst_size(&mut self) -> BURST_SIZE_W<CTRL_SPEC> {
        BURST_SIZE_W::new(self, 24)
    }
    #[doc = "Bit 30 - Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0."]
    #[inline(always)]
    #[must_use]
    pub fn dis_ie(&mut self) -> DIS_IE_W<CTRL_SPEC> {
        DIS_IE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ctz_ie(&mut self) -> CTZ_IE_W<CTRL_SPEC> {
        CTZ_IE_W::new(self, 31)
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
#[doc = "DMA Channel Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
