#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `TX_BUSY` reader - Read-only flag indicating the UART transmit status"]
pub type TX_BUSY_R = crate::BitReader;
#[doc = "Field `RX_BUSY` reader - Read-only flag indicating the UART receiver status"]
pub type RX_BUSY_R = crate::BitReader;
#[doc = "Field `RX_EM` reader - Read-only flag indicating the RX FIFO state"]
pub type RX_EM_R = crate::BitReader;
#[doc = "Field `RX_FULL` reader - Read-only flag indicating the RX FIFO state"]
pub type RX_FULL_R = crate::BitReader;
#[doc = "Field `TX_EM` reader - Read-only flag indicating the TX FIFO state"]
pub type TX_EM_R = crate::BitReader;
#[doc = "Field `TX_FULL` reader - Read-only flag indicating the TX FIFO state"]
pub type TX_FULL_R = crate::BitReader;
#[doc = "Field `RX_LVL` reader - Indicates the number of bytes currently in the RX FIFO (0-RX FIFO_ELTS)"]
pub type RX_LVL_R = crate::FieldReader;
#[doc = "Field `TX_LVL` reader - Indicates the number of bytes currently in the TX FIFO (0-TX FIFO_ELTS)"]
pub type TX_LVL_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Read-only flag indicating the UART transmit status"]
    #[inline(always)]
    pub fn tx_busy(&self) -> TX_BUSY_R {
        TX_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read-only flag indicating the UART receiver status"]
    #[inline(always)]
    pub fn rx_busy(&self) -> RX_BUSY_R {
        RX_BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Read-only flag indicating the RX FIFO state"]
    #[inline(always)]
    pub fn rx_em(&self) -> RX_EM_R {
        RX_EM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read-only flag indicating the RX FIFO state"]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Read-only flag indicating the TX FIFO state"]
    #[inline(always)]
    pub fn tx_em(&self) -> TX_EM_R {
        TX_EM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Read-only flag indicating the TX FIFO state"]
    #[inline(always)]
    pub fn tx_full(&self) -> TX_FULL_R {
        TX_FULL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Indicates the number of bytes currently in the RX FIFO (0-RX FIFO_ELTS)"]
    #[inline(always)]
    pub fn rx_lvl(&self) -> RX_LVL_R {
        RX_LVL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Indicates the number of bytes currently in the TX FIFO (0-TX FIFO_ELTS)"]
    #[inline(always)]
    pub fn tx_lvl(&self) -> TX_LVL_R {
        TX_LVL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
