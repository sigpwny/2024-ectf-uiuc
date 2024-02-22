#[doc = "Register `INTFL0` reader"]
pub type R = crate::R<INTFL0_SPEC>;
#[doc = "Register `INTFL0` writer"]
pub type W = crate::W<INTFL0_SPEC>;
#[doc = "Field `DONE` reader - Transfer Done Interrupt."]
pub type DONE_R = crate::BitReader<INT_FL0_DONE_A>;
#[doc = "Transfer Done Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_FL0_DONE_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<INT_FL0_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: INT_FL0_DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT_FL0_DONE_A {
        match self.bits {
            false => INT_FL0_DONE_A::INACTIVE,
            true => INT_FL0_DONE_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == INT_FL0_DONE_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == INT_FL0_DONE_A::PENDING
    }
}
#[doc = "Field `DONE` writer - Transfer Done Interrupt."]
pub type DONE_W<'a, REG> = crate::BitWriter<'a, REG, INT_FL0_DONE_A>;
impl<'a, REG> DONE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(INT_FL0_DONE_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(INT_FL0_DONE_A::PENDING)
    }
}
#[doc = "Field `IRXM` reader - Interactive Receive Interrupt."]
pub type IRXM_R = crate::BitReader<IRXM_A>;
#[doc = "Interactive Receive Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRXM_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<IRXM_A> for bool {
    #[inline(always)]
    fn from(variant: IRXM_A) -> Self {
        variant as u8 != 0
    }
}
impl IRXM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IRXM_A {
        match self.bits {
            false => IRXM_A::INACTIVE,
            true => IRXM_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == IRXM_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == IRXM_A::PENDING
    }
}
#[doc = "Field `IRXM` writer - Interactive Receive Interrupt."]
pub type IRXM_W<'a, REG> = crate::BitWriter<'a, REG, IRXM_A>;
impl<'a, REG> IRXM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(IRXM_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(IRXM_A::PENDING)
    }
}
#[doc = "Field `GC_ADDR_MATCH` reader - Slave General Call Address Match Interrupt."]
pub type GC_ADDR_MATCH_R = crate::BitReader<GC_ADDR_MATCH_A>;
#[doc = "Slave General Call Address Match Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GC_ADDR_MATCH_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<GC_ADDR_MATCH_A> for bool {
    #[inline(always)]
    fn from(variant: GC_ADDR_MATCH_A) -> Self {
        variant as u8 != 0
    }
}
impl GC_ADDR_MATCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GC_ADDR_MATCH_A {
        match self.bits {
            false => GC_ADDR_MATCH_A::INACTIVE,
            true => GC_ADDR_MATCH_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == GC_ADDR_MATCH_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == GC_ADDR_MATCH_A::PENDING
    }
}
#[doc = "Field `GC_ADDR_MATCH` writer - Slave General Call Address Match Interrupt."]
pub type GC_ADDR_MATCH_W<'a, REG> = crate::BitWriter<'a, REG, GC_ADDR_MATCH_A>;
impl<'a, REG> GC_ADDR_MATCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(GC_ADDR_MATCH_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(GC_ADDR_MATCH_A::PENDING)
    }
}
#[doc = "Field `ADDR_MATCH` reader - Slave Address Match Interrupt."]
pub type ADDR_MATCH_R = crate::BitReader<ADDR_MATCH_A>;
#[doc = "Slave Address Match Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR_MATCH_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<ADDR_MATCH_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_MATCH_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDR_MATCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDR_MATCH_A {
        match self.bits {
            false => ADDR_MATCH_A::INACTIVE,
            true => ADDR_MATCH_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ADDR_MATCH_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ADDR_MATCH_A::PENDING
    }
}
#[doc = "Field `ADDR_MATCH` writer - Slave Address Match Interrupt."]
pub type ADDR_MATCH_W<'a, REG> = crate::BitWriter<'a, REG, ADDR_MATCH_A>;
impl<'a, REG> ADDR_MATCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(ADDR_MATCH_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(ADDR_MATCH_A::PENDING)
    }
}
#[doc = "Field `RX_THD` reader - Receive Threshold Interrupt. This bit is automaticcaly cleared when RX_FIFO is below the threshold level."]
pub type RX_THD_R = crate::BitReader<RX_THD_A>;
#[doc = "Receive Threshold Interrupt. This bit is automaticcaly cleared when RX_FIFO is below the threshold level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_THD_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending. RX_FIFO equal or more bytes than the threshold."]
    PENDING = 1,
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
    pub const fn variant(&self) -> RX_THD_A {
        match self.bits {
            false => RX_THD_A::INACTIVE,
            true => RX_THD_A::PENDING,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == RX_THD_A::INACTIVE
    }
    #[doc = "An interrupt is pending. RX_FIFO equal or more bytes than the threshold."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RX_THD_A::PENDING
    }
}
#[doc = "Field `RX_THD` writer - Receive Threshold Interrupt. This bit is automaticcaly cleared when RX_FIFO is below the threshold level."]
pub type RX_THD_W<'a, REG> = crate::BitWriter<'a, REG, RX_THD_A>;
impl<'a, REG> RX_THD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(RX_THD_A::INACTIVE)
    }
    #[doc = "An interrupt is pending. RX_FIFO equal or more bytes than the threshold."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(RX_THD_A::PENDING)
    }
}
#[doc = "Field `TX_THD` reader - Transmit Threshold Interrupt. This bit is automaticcaly cleared when TX_FIFO is above the threshold level."]
pub type TX_THD_R = crate::BitReader<TX_THD_A>;
#[doc = "Transmit Threshold Interrupt. This bit is automaticcaly cleared when TX_FIFO is above the threshold level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_THD_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending. TX_FIFO has equal or less bytes than the threshold."]
    PENDING = 1,
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
    pub const fn variant(&self) -> TX_THD_A {
        match self.bits {
            false => TX_THD_A::INACTIVE,
            true => TX_THD_A::PENDING,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == TX_THD_A::INACTIVE
    }
    #[doc = "An interrupt is pending. TX_FIFO has equal or less bytes than the threshold."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TX_THD_A::PENDING
    }
}
#[doc = "Field `TX_THD` writer - Transmit Threshold Interrupt. This bit is automaticcaly cleared when TX_FIFO is above the threshold level."]
pub type TX_THD_W<'a, REG> = crate::BitWriter<'a, REG, TX_THD_A>;
impl<'a, REG> TX_THD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(TX_THD_A::INACTIVE)
    }
    #[doc = "An interrupt is pending. TX_FIFO has equal or less bytes than the threshold."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(TX_THD_A::PENDING)
    }
}
#[doc = "Field `STOP` reader - STOP Interrupt."]
pub type STOP_R = crate::BitReader<STOP_A>;
#[doc = "STOP Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending. TX_FIFO has equal or less bytes than the threshold."]
    PENDING = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::INACTIVE,
            true => STOP_A::PENDING,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == STOP_A::INACTIVE
    }
    #[doc = "An interrupt is pending. TX_FIFO has equal or less bytes than the threshold."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == STOP_A::PENDING
    }
}
#[doc = "Field `STOP` writer - STOP Interrupt."]
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG, STOP_A>;
impl<'a, REG> STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::INACTIVE)
    }
    #[doc = "An interrupt is pending. TX_FIFO has equal or less bytes than the threshold."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::PENDING)
    }
}
#[doc = "Field `ADDR_ACK` reader - Address Acknowledge Interrupt."]
pub type ADDR_ACK_R = crate::BitReader<ADDR_ACK_A>;
#[doc = "Address Acknowledge Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR_ACK_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<ADDR_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDR_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDR_ACK_A {
        match self.bits {
            false => ADDR_ACK_A::INACTIVE,
            true => ADDR_ACK_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ADDR_ACK_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ADDR_ACK_A::PENDING
    }
}
#[doc = "Field `ADDR_ACK` writer - Address Acknowledge Interrupt."]
pub type ADDR_ACK_W<'a, REG> = crate::BitWriter<'a, REG, ADDR_ACK_A>;
impl<'a, REG> ADDR_ACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(ADDR_ACK_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(ADDR_ACK_A::PENDING)
    }
}
#[doc = "Field `ARB_ERR` reader - Arbritation error Interrupt."]
pub type ARB_ERR_R = crate::BitReader<ARB_ERR_A>;
#[doc = "Arbritation error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARB_ERR_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<ARB_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ARB_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ARB_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ARB_ERR_A {
        match self.bits {
            false => ARB_ERR_A::INACTIVE,
            true => ARB_ERR_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ARB_ERR_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ARB_ERR_A::PENDING
    }
}
#[doc = "Field `ARB_ERR` writer - Arbritation error Interrupt."]
pub type ARB_ERR_W<'a, REG> = crate::BitWriter<'a, REG, ARB_ERR_A>;
impl<'a, REG> ARB_ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(ARB_ERR_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(ARB_ERR_A::PENDING)
    }
}
#[doc = "Field `TO_ERR` reader - timeout Error Interrupt."]
pub type TO_ERR_R = crate::BitReader<TO_ERR_A>;
#[doc = "timeout Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TO_ERR_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<TO_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: TO_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl TO_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TO_ERR_A {
        match self.bits {
            false => TO_ERR_A::INACTIVE,
            true => TO_ERR_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == TO_ERR_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TO_ERR_A::PENDING
    }
}
#[doc = "Field `TO_ERR` writer - timeout Error Interrupt."]
pub type TO_ERR_W<'a, REG> = crate::BitWriter<'a, REG, TO_ERR_A>;
impl<'a, REG> TO_ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(TO_ERR_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(TO_ERR_A::PENDING)
    }
}
#[doc = "Field `ADDR_NACK_ERR` reader - Address NACK Error Interrupt."]
pub type ADDR_NACK_ERR_R = crate::BitReader<ADDR_NACK_ERR_A>;
#[doc = "Address NACK Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR_NACK_ERR_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<ADDR_NACK_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_NACK_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDR_NACK_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDR_NACK_ERR_A {
        match self.bits {
            false => ADDR_NACK_ERR_A::INACTIVE,
            true => ADDR_NACK_ERR_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ADDR_NACK_ERR_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ADDR_NACK_ERR_A::PENDING
    }
}
#[doc = "Field `ADDR_NACK_ERR` writer - Address NACK Error Interrupt."]
pub type ADDR_NACK_ERR_W<'a, REG> = crate::BitWriter<'a, REG, ADDR_NACK_ERR_A>;
impl<'a, REG> ADDR_NACK_ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(ADDR_NACK_ERR_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(ADDR_NACK_ERR_A::PENDING)
    }
}
#[doc = "Field `DATA_ERR` reader - Data NACK Error Interrupt."]
pub type DATA_ERR_R = crate::BitReader<DATA_ERR_A>;
#[doc = "Data NACK Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATA_ERR_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<DATA_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl DATA_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATA_ERR_A {
        match self.bits {
            false => DATA_ERR_A::INACTIVE,
            true => DATA_ERR_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DATA_ERR_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DATA_ERR_A::PENDING
    }
}
#[doc = "Field `DATA_ERR` writer - Data NACK Error Interrupt."]
pub type DATA_ERR_W<'a, REG> = crate::BitWriter<'a, REG, DATA_ERR_A>;
impl<'a, REG> DATA_ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_ERR_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_ERR_A::PENDING)
    }
}
#[doc = "Field `DNR_ERR` reader - Do Not Respond Error Interrupt."]
pub type DNR_ERR_R = crate::BitReader<DNR_ERR_A>;
#[doc = "Do Not Respond Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DNR_ERR_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<DNR_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: DNR_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl DNR_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DNR_ERR_A {
        match self.bits {
            false => DNR_ERR_A::INACTIVE,
            true => DNR_ERR_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DNR_ERR_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DNR_ERR_A::PENDING
    }
}
#[doc = "Field `DNR_ERR` writer - Do Not Respond Error Interrupt."]
pub type DNR_ERR_W<'a, REG> = crate::BitWriter<'a, REG, DNR_ERR_A>;
impl<'a, REG> DNR_ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(DNR_ERR_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(DNR_ERR_A::PENDING)
    }
}
#[doc = "Field `START_ERR` reader - Start Error Interrupt."]
pub type START_ERR_R = crate::BitReader<START_ERR_A>;
#[doc = "Start Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_ERR_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<START_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: START_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl START_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> START_ERR_A {
        match self.bits {
            false => START_ERR_A::INACTIVE,
            true => START_ERR_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == START_ERR_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == START_ERR_A::PENDING
    }
}
#[doc = "Field `START_ERR` writer - Start Error Interrupt."]
pub type START_ERR_W<'a, REG> = crate::BitWriter<'a, REG, START_ERR_A>;
impl<'a, REG> START_ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(START_ERR_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(START_ERR_A::PENDING)
    }
}
#[doc = "Field `STOP_ERR` reader - Stop Error Interrupt."]
pub type STOP_ERR_R = crate::BitReader<STOP_ERR_A>;
#[doc = "Stop Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_ERR_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<STOP_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOP_ERR_A {
        match self.bits {
            false => STOP_ERR_A::INACTIVE,
            true => STOP_ERR_A::PENDING,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == STOP_ERR_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == STOP_ERR_A::PENDING
    }
}
#[doc = "Field `STOP_ERR` writer - Stop Error Interrupt."]
pub type STOP_ERR_W<'a, REG> = crate::BitWriter<'a, REG, STOP_ERR_A>;
impl<'a, REG> STOP_ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_ERR_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_ERR_A::PENDING)
    }
}
#[doc = "Field `TX_LOCKOUT` reader - Transmit Lock Out Interrupt."]
pub type TX_LOCKOUT_R = crate::BitReader;
#[doc = "Field `TX_LOCKOUT` writer - Transmit Lock Out Interrupt."]
pub type TX_LOCKOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAMI` reader - Multiple Address Match Interrupt"]
pub type MAMI_R = crate::FieldReader;
#[doc = "Field `MAMI` writer - Multiple Address Match Interrupt"]
pub type MAMI_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RD_ADDR_MATCH` reader - Slave Read Address Match Interrupt"]
pub type RD_ADDR_MATCH_R = crate::BitReader;
#[doc = "Field `RD_ADDR_MATCH` writer - Slave Read Address Match Interrupt"]
pub type RD_ADDR_MATCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_ADDR_MATCH` reader - Slave Write Address Match Interrupt"]
pub type WR_ADDR_MATCH_R = crate::BitReader;
#[doc = "Field `WR_ADDR_MATCH` writer - Slave Write Address Match Interrupt"]
pub type WR_ADDR_MATCH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Done Interrupt."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interactive Receive Interrupt."]
    #[inline(always)]
    pub fn irxm(&self) -> IRXM_R {
        IRXM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave General Call Address Match Interrupt."]
    #[inline(always)]
    pub fn gc_addr_match(&self) -> GC_ADDR_MATCH_R {
        GC_ADDR_MATCH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Address Match Interrupt."]
    #[inline(always)]
    pub fn addr_match(&self) -> ADDR_MATCH_R {
        ADDR_MATCH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Threshold Interrupt. This bit is automaticcaly cleared when RX_FIFO is below the threshold level."]
    #[inline(always)]
    pub fn rx_thd(&self) -> RX_THD_R {
        RX_THD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Threshold Interrupt. This bit is automaticcaly cleared when TX_FIFO is above the threshold level."]
    #[inline(always)]
    pub fn tx_thd(&self) -> TX_THD_R {
        TX_THD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - STOP Interrupt."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Address Acknowledge Interrupt."]
    #[inline(always)]
    pub fn addr_ack(&self) -> ADDR_ACK_R {
        ADDR_ACK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Arbritation error Interrupt."]
    #[inline(always)]
    pub fn arb_err(&self) -> ARB_ERR_R {
        ARB_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - timeout Error Interrupt."]
    #[inline(always)]
    pub fn to_err(&self) -> TO_ERR_R {
        TO_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Address NACK Error Interrupt."]
    #[inline(always)]
    pub fn addr_nack_err(&self) -> ADDR_NACK_ERR_R {
        ADDR_NACK_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data NACK Error Interrupt."]
    #[inline(always)]
    pub fn data_err(&self) -> DATA_ERR_R {
        DATA_ERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Do Not Respond Error Interrupt."]
    #[inline(always)]
    pub fn dnr_err(&self) -> DNR_ERR_R {
        DNR_ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Start Error Interrupt."]
    #[inline(always)]
    pub fn start_err(&self) -> START_ERR_R {
        START_ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stop Error Interrupt."]
    #[inline(always)]
    pub fn stop_err(&self) -> STOP_ERR_R {
        STOP_ERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmit Lock Out Interrupt."]
    #[inline(always)]
    pub fn tx_lockout(&self) -> TX_LOCKOUT_R {
        TX_LOCKOUT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Multiple Address Match Interrupt"]
    #[inline(always)]
    pub fn mami(&self) -> MAMI_R {
        MAMI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Slave Read Address Match Interrupt"]
    #[inline(always)]
    pub fn rd_addr_match(&self) -> RD_ADDR_MATCH_R {
        RD_ADDR_MATCH_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Slave Write Address Match Interrupt"]
    #[inline(always)]
    pub fn wr_addr_match(&self) -> WR_ADDR_MATCH_R {
        WR_ADDR_MATCH_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Done Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<INTFL0_SPEC> {
        DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interactive Receive Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn irxm(&mut self) -> IRXM_W<INTFL0_SPEC> {
        IRXM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Slave General Call Address Match Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn gc_addr_match(&mut self) -> GC_ADDR_MATCH_W<INTFL0_SPEC> {
        GC_ADDR_MATCH_W::new(self, 2)
    }
    #[doc = "Bit 3 - Slave Address Match Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn addr_match(&mut self) -> ADDR_MATCH_W<INTFL0_SPEC> {
        ADDR_MATCH_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive Threshold Interrupt. This bit is automaticcaly cleared when RX_FIFO is below the threshold level."]
    #[inline(always)]
    #[must_use]
    pub fn rx_thd(&mut self) -> RX_THD_W<INTFL0_SPEC> {
        RX_THD_W::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit Threshold Interrupt. This bit is automaticcaly cleared when TX_FIFO is above the threshold level."]
    #[inline(always)]
    #[must_use]
    pub fn tx_thd(&mut self) -> TX_THD_W<INTFL0_SPEC> {
        TX_THD_W::new(self, 5)
    }
    #[doc = "Bit 6 - STOP Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<INTFL0_SPEC> {
        STOP_W::new(self, 6)
    }
    #[doc = "Bit 7 - Address Acknowledge Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn addr_ack(&mut self) -> ADDR_ACK_W<INTFL0_SPEC> {
        ADDR_ACK_W::new(self, 7)
    }
    #[doc = "Bit 8 - Arbritation error Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn arb_err(&mut self) -> ARB_ERR_W<INTFL0_SPEC> {
        ARB_ERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - timeout Error Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn to_err(&mut self) -> TO_ERR_W<INTFL0_SPEC> {
        TO_ERR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Address NACK Error Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn addr_nack_err(&mut self) -> ADDR_NACK_ERR_W<INTFL0_SPEC> {
        ADDR_NACK_ERR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Data NACK Error Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn data_err(&mut self) -> DATA_ERR_W<INTFL0_SPEC> {
        DATA_ERR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Do Not Respond Error Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dnr_err(&mut self) -> DNR_ERR_W<INTFL0_SPEC> {
        DNR_ERR_W::new(self, 12)
    }
    #[doc = "Bit 13 - Start Error Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn start_err(&mut self) -> START_ERR_W<INTFL0_SPEC> {
        START_ERR_W::new(self, 13)
    }
    #[doc = "Bit 14 - Stop Error Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn stop_err(&mut self) -> STOP_ERR_W<INTFL0_SPEC> {
        STOP_ERR_W::new(self, 14)
    }
    #[doc = "Bit 15 - Transmit Lock Out Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_lockout(&mut self) -> TX_LOCKOUT_W<INTFL0_SPEC> {
        TX_LOCKOUT_W::new(self, 15)
    }
    #[doc = "Bits 16:21 - Multiple Address Match Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mami(&mut self) -> MAMI_W<INTFL0_SPEC> {
        MAMI_W::new(self, 16)
    }
    #[doc = "Bit 22 - Slave Read Address Match Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rd_addr_match(&mut self) -> RD_ADDR_MATCH_W<INTFL0_SPEC> {
        RD_ADDR_MATCH_W::new(self, 22)
    }
    #[doc = "Bit 23 - Slave Write Address Match Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wr_addr_match(&mut self) -> WR_ADDR_MATCH_W<INTFL0_SPEC> {
        WR_ADDR_MATCH_W::new(self, 23)
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
#[doc = "Interrupt Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFL0_SPEC;
impl crate::RegisterSpec for INTFL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intfl0::R`](R) reader structure"]
impl crate::Readable for INTFL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intfl0::W`](W) writer structure"]
impl crate::Writable for INTFL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTFL0 to value 0"]
impl crate::Resettable for INTFL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
