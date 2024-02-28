#[doc = "Register `INTFL` reader"]
pub type R = crate::R<INTFL_SPEC>;
#[doc = "Register `INTFL` writer"]
pub type W = crate::W<INTFL_SPEC>;
#[doc = "Field `TX_THD` reader - TX FIFO Threshold Crossed."]
pub type TX_THD_R = crate::BitReader<TX_THD_A>;
#[doc = "TX FIFO Threshold Crossed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_THD_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<TX_THD_A> for bool {
    #[inline(always)]
    fn from(variant: TX_THD_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_THD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TX_THD_A> {
        match self.bits {
            true => Some(TX_THD_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TX_THD_A::CLEAR
    }
}
#[doc = "Field `TX_THD` writer - TX FIFO Threshold Crossed."]
pub type TX_THD_W<'a, REG> = crate::BitWriter<'a, REG, TX_THD_A>;
impl<'a, REG> TX_THD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TX_THD_A::CLEAR)
    }
}
#[doc = "Field `TX_EM` reader - TX FIFO Empty."]
pub type TX_EM_R = crate::BitReader<TX_EM_A>;
#[doc = "TX FIFO Empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_EM_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<TX_EM_A> for bool {
    #[inline(always)]
    fn from(variant: TX_EM_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_EM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TX_EM_A> {
        match self.bits {
            true => Some(TX_EM_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TX_EM_A::CLEAR
    }
}
#[doc = "Field `TX_EM` writer - TX FIFO Empty."]
pub type TX_EM_W<'a, REG> = crate::BitWriter<'a, REG, TX_EM_A>;
impl<'a, REG> TX_EM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TX_EM_A::CLEAR)
    }
}
#[doc = "Field `RX_THD` reader - RX FIFO Threshold Crossed."]
pub type RX_THD_R = crate::BitReader<RX_THD_A>;
#[doc = "RX FIFO Threshold Crossed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_THD_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<RX_THD_A> for bool {
    #[inline(always)]
    fn from(variant: RX_THD_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_THD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RX_THD_A> {
        match self.bits {
            true => Some(RX_THD_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RX_THD_A::CLEAR
    }
}
#[doc = "Field `RX_THD` writer - RX FIFO Threshold Crossed."]
pub type RX_THD_W<'a, REG> = crate::BitWriter<'a, REG, RX_THD_A>;
impl<'a, REG> RX_THD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RX_THD_A::CLEAR)
    }
}
#[doc = "Field `RX_FULL` reader - RX FIFO FULL."]
pub type RX_FULL_R = crate::BitReader<RX_FULL_A>;
#[doc = "RX FIFO FULL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FULL_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<RX_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FULL_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_FULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RX_FULL_A> {
        match self.bits {
            true => Some(RX_FULL_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RX_FULL_A::CLEAR
    }
}
#[doc = "Field `RX_FULL` writer - RX FIFO FULL."]
pub type RX_FULL_W<'a, REG> = crate::BitWriter<'a, REG, RX_FULL_A>;
impl<'a, REG> RX_FULL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FULL_A::CLEAR)
    }
}
#[doc = "Field `SSA` reader - Slave Select Asserted."]
pub type SSA_R = crate::BitReader<SSA_A>;
#[doc = "Slave Select Asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSA_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<SSA_A> for bool {
    #[inline(always)]
    fn from(variant: SSA_A) -> Self {
        variant as u8 != 0
    }
}
impl SSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SSA_A> {
        match self.bits {
            true => Some(SSA_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == SSA_A::CLEAR
    }
}
#[doc = "Field `SSA` writer - Slave Select Asserted."]
pub type SSA_W<'a, REG> = crate::BitWriter<'a, REG, SSA_A>;
impl<'a, REG> SSA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SSA_A::CLEAR)
    }
}
#[doc = "Field `SSD` reader - Slave Select Deasserted."]
pub type SSD_R = crate::BitReader<SSD_A>;
#[doc = "Slave Select Deasserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSD_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<SSD_A> for bool {
    #[inline(always)]
    fn from(variant: SSD_A) -> Self {
        variant as u8 != 0
    }
}
impl SSD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SSD_A> {
        match self.bits {
            true => Some(SSD_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == SSD_A::CLEAR
    }
}
#[doc = "Field `SSD` writer - Slave Select Deasserted."]
pub type SSD_W<'a, REG> = crate::BitWriter<'a, REG, SSD_A>;
impl<'a, REG> SSD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SSD_A::CLEAR)
    }
}
#[doc = "Field `FAULT` reader - Multi-Master Mode Fault."]
pub type FAULT_R = crate::BitReader<FAULT_A>;
#[doc = "Multi-Master Mode Fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAULT_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<FAULT_A> for bool {
    #[inline(always)]
    fn from(variant: FAULT_A) -> Self {
        variant as u8 != 0
    }
}
impl FAULT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FAULT_A> {
        match self.bits {
            true => Some(FAULT_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == FAULT_A::CLEAR
    }
}
#[doc = "Field `FAULT` writer - Multi-Master Mode Fault."]
pub type FAULT_W<'a, REG> = crate::BitWriter<'a, REG, FAULT_A>;
impl<'a, REG> FAULT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FAULT_A::CLEAR)
    }
}
#[doc = "Field `ABORT` reader - Slave Abort Detected."]
pub type ABORT_R = crate::BitReader<ABORT_A>;
#[doc = "Slave Abort Detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<ABORT_A> for bool {
    #[inline(always)]
    fn from(variant: ABORT_A) -> Self {
        variant as u8 != 0
    }
}
impl ABORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ABORT_A> {
        match self.bits {
            true => Some(ABORT_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ABORT_A::CLEAR
    }
}
#[doc = "Field `ABORT` writer - Slave Abort Detected."]
pub type ABORT_W<'a, REG> = crate::BitWriter<'a, REG, ABORT_A>;
impl<'a, REG> ABORT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ABORT_A::CLEAR)
    }
}
#[doc = "Field `MST_DONE` reader - Master Done, set when SPI Master has completed any transactions."]
pub type MST_DONE_R = crate::BitReader<MST_DONE_A>;
#[doc = "Master Done, set when SPI Master has completed any transactions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MST_DONE_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<MST_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: MST_DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl MST_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MST_DONE_A> {
        match self.bits {
            true => Some(MST_DONE_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == MST_DONE_A::CLEAR
    }
}
#[doc = "Field `MST_DONE` writer - Master Done, set when SPI Master has completed any transactions."]
pub type MST_DONE_W<'a, REG> = crate::BitWriter<'a, REG, MST_DONE_A>;
impl<'a, REG> MST_DONE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(MST_DONE_A::CLEAR)
    }
}
#[doc = "Field `TX_OV` reader - Transmit FIFO Overrun, set when the AMBA side attempts to write data to a full transmit FIFO."]
pub type TX_OV_R = crate::BitReader<TX_OV_A>;
#[doc = "Transmit FIFO Overrun, set when the AMBA side attempts to write data to a full transmit FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_OV_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<TX_OV_A> for bool {
    #[inline(always)]
    fn from(variant: TX_OV_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_OV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TX_OV_A> {
        match self.bits {
            true => Some(TX_OV_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TX_OV_A::CLEAR
    }
}
#[doc = "Field `TX_OV` writer - Transmit FIFO Overrun, set when the AMBA side attempts to write data to a full transmit FIFO."]
pub type TX_OV_W<'a, REG> = crate::BitWriter<'a, REG, TX_OV_A>;
impl<'a, REG> TX_OV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TX_OV_A::CLEAR)
    }
}
#[doc = "Field `TX_UN` reader - Transmit FIFO Underrun, set when the SPI side attempts to read data from an empty transmit FIFO."]
pub type TX_UN_R = crate::BitReader<TX_UN_A>;
#[doc = "Transmit FIFO Underrun, set when the SPI side attempts to read data from an empty transmit FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_UN_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<TX_UN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_UN_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_UN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TX_UN_A> {
        match self.bits {
            true => Some(TX_UN_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TX_UN_A::CLEAR
    }
}
#[doc = "Field `TX_UN` writer - Transmit FIFO Underrun, set when the SPI side attempts to read data from an empty transmit FIFO."]
pub type TX_UN_W<'a, REG> = crate::BitWriter<'a, REG, TX_UN_A>;
impl<'a, REG> TX_UN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TX_UN_A::CLEAR)
    }
}
#[doc = "Field `RX_OV` reader - Receive FIFO Overrun, set when the SPI side attempts to write to a full receive FIFO."]
pub type RX_OV_R = crate::BitReader<RX_OV_A>;
#[doc = "Receive FIFO Overrun, set when the SPI side attempts to write to a full receive FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_OV_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<RX_OV_A> for bool {
    #[inline(always)]
    fn from(variant: RX_OV_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_OV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RX_OV_A> {
        match self.bits {
            true => Some(RX_OV_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RX_OV_A::CLEAR
    }
}
#[doc = "Field `RX_OV` writer - Receive FIFO Overrun, set when the SPI side attempts to write to a full receive FIFO."]
pub type RX_OV_W<'a, REG> = crate::BitWriter<'a, REG, RX_OV_A>;
impl<'a, REG> RX_OV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RX_OV_A::CLEAR)
    }
}
#[doc = "Field `RX_UN` reader - Receive FIFO Underrun, set when the AMBA side attempts to read data from an empty receive FIFO."]
pub type RX_UN_R = crate::BitReader<RX_UN_A>;
#[doc = "Receive FIFO Underrun, set when the AMBA side attempts to read data from an empty receive FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_UN_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<RX_UN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_UN_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_UN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RX_UN_A> {
        match self.bits {
            true => Some(RX_UN_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RX_UN_A::CLEAR
    }
}
#[doc = "Field `RX_UN` writer - Receive FIFO Underrun, set when the AMBA side attempts to read data from an empty receive FIFO."]
pub type RX_UN_W<'a, REG> = crate::BitWriter<'a, REG, RX_UN_A>;
impl<'a, REG> RX_UN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RX_UN_A::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - TX FIFO Threshold Crossed."]
    #[inline(always)]
    pub fn tx_thd(&self) -> TX_THD_R {
        TX_THD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Empty."]
    #[inline(always)]
    pub fn tx_em(&self) -> TX_EM_R {
        TX_EM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO Threshold Crossed."]
    #[inline(always)]
    pub fn rx_thd(&self) -> RX_THD_R {
        RX_THD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO FULL."]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Select Asserted."]
    #[inline(always)]
    pub fn ssa(&self) -> SSA_R {
        SSA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slave Select Deasserted."]
    #[inline(always)]
    pub fn ssd(&self) -> SSD_R {
        SSD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Multi-Master Mode Fault."]
    #[inline(always)]
    pub fn fault(&self) -> FAULT_R {
        FAULT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Slave Abort Detected."]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Master Done, set when SPI Master has completed any transactions."]
    #[inline(always)]
    pub fn mst_done(&self) -> MST_DONE_R {
        MST_DONE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit FIFO Overrun, set when the AMBA side attempts to write data to a full transmit FIFO."]
    #[inline(always)]
    pub fn tx_ov(&self) -> TX_OV_R {
        TX_OV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit FIFO Underrun, set when the SPI side attempts to read data from an empty transmit FIFO."]
    #[inline(always)]
    pub fn tx_un(&self) -> TX_UN_R {
        TX_UN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive FIFO Overrun, set when the SPI side attempts to write to a full receive FIFO."]
    #[inline(always)]
    pub fn rx_ov(&self) -> RX_OV_R {
        RX_OV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO Underrun, set when the AMBA side attempts to read data from an empty receive FIFO."]
    #[inline(always)]
    pub fn rx_un(&self) -> RX_UN_R {
        RX_UN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX FIFO Threshold Crossed."]
    #[inline(always)]
    #[must_use]
    pub fn tx_thd(&mut self) -> TX_THD_W<INTFL_SPEC> {
        TX_THD_W::new(self, 0)
    }
    #[doc = "Bit 1 - TX FIFO Empty."]
    #[inline(always)]
    #[must_use]
    pub fn tx_em(&mut self) -> TX_EM_W<INTFL_SPEC> {
        TX_EM_W::new(self, 1)
    }
    #[doc = "Bit 2 - RX FIFO Threshold Crossed."]
    #[inline(always)]
    #[must_use]
    pub fn rx_thd(&mut self) -> RX_THD_W<INTFL_SPEC> {
        RX_THD_W::new(self, 2)
    }
    #[doc = "Bit 3 - RX FIFO FULL."]
    #[inline(always)]
    #[must_use]
    pub fn rx_full(&mut self) -> RX_FULL_W<INTFL_SPEC> {
        RX_FULL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Slave Select Asserted."]
    #[inline(always)]
    #[must_use]
    pub fn ssa(&mut self) -> SSA_W<INTFL_SPEC> {
        SSA_W::new(self, 4)
    }
    #[doc = "Bit 5 - Slave Select Deasserted."]
    #[inline(always)]
    #[must_use]
    pub fn ssd(&mut self) -> SSD_W<INTFL_SPEC> {
        SSD_W::new(self, 5)
    }
    #[doc = "Bit 8 - Multi-Master Mode Fault."]
    #[inline(always)]
    #[must_use]
    pub fn fault(&mut self) -> FAULT_W<INTFL_SPEC> {
        FAULT_W::new(self, 8)
    }
    #[doc = "Bit 9 - Slave Abort Detected."]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<INTFL_SPEC> {
        ABORT_W::new(self, 9)
    }
    #[doc = "Bit 11 - Master Done, set when SPI Master has completed any transactions."]
    #[inline(always)]
    #[must_use]
    pub fn mst_done(&mut self) -> MST_DONE_W<INTFL_SPEC> {
        MST_DONE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit FIFO Overrun, set when the AMBA side attempts to write data to a full transmit FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ov(&mut self) -> TX_OV_W<INTFL_SPEC> {
        TX_OV_W::new(self, 12)
    }
    #[doc = "Bit 13 - Transmit FIFO Underrun, set when the SPI side attempts to read data from an empty transmit FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn tx_un(&mut self) -> TX_UN_W<INTFL_SPEC> {
        TX_UN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Receive FIFO Overrun, set when the SPI side attempts to write to a full receive FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ov(&mut self) -> RX_OV_W<INTFL_SPEC> {
        RX_OV_W::new(self, 14)
    }
    #[doc = "Bit 15 - Receive FIFO Underrun, set when the AMBA side attempts to read data from an empty receive FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn rx_un(&mut self) -> RX_UN_W<INTFL_SPEC> {
        RX_UN_W::new(self, 15)
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
#[doc = "Register for reading and clearing interrupt flags. All bits are write 1 to clear.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFL_SPEC;
impl crate::RegisterSpec for INTFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intfl::R`](R) reader structure"]
impl crate::Readable for INTFL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intfl::W`](W) writer structure"]
impl crate::Writable for INTFL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTFL to value 0"]
impl crate::Resettable for INTFL_SPEC {
    const RESET_VALUE: u32 = 0;
}
