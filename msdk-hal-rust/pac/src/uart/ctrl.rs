#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `RX_THD_VAL` reader - This field specifies the depth of receive FIFO for interrupt generation (value 0 and > 16 are ignored)"]
pub type RX_THD_VAL_R = crate::FieldReader;
#[doc = "Field `RX_THD_VAL` writer - This field specifies the depth of receive FIFO for interrupt generation (value 0 and > 16 are ignored)"]
pub type RX_THD_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PAR_EN` reader - Parity Enable"]
pub type PAR_EN_R = crate::BitReader;
#[doc = "Field `PAR_EN` writer - Parity Enable"]
pub type PAR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAR_EO` reader - when PAREN=1 selects odd or even parity odd is 1 even is 0"]
pub type PAR_EO_R = crate::BitReader;
#[doc = "Field `PAR_EO` writer - when PAREN=1 selects odd or even parity odd is 1 even is 0"]
pub type PAR_EO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAR_MD` reader - Selects parity based on 1s or 0s count (when PAREN=1)"]
pub type PAR_MD_R = crate::BitReader;
#[doc = "Field `PAR_MD` writer - Selects parity based on 1s or 0s count (when PAREN=1)"]
pub type PAR_MD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS_DIS` reader - CTS Sampling Disable"]
pub type CTS_DIS_R = crate::BitReader;
#[doc = "Field `CTS_DIS` writer - CTS Sampling Disable"]
pub type CTS_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FLUSH` reader - Flushes the TX FIFO buffer. This bit is automatically cleared by hardware when flush is completed."]
pub type TX_FLUSH_R = crate::BitReader;
#[doc = "Field `TX_FLUSH` writer - Flushes the TX FIFO buffer. This bit is automatically cleared by hardware when flush is completed."]
pub type TX_FLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FLUSH` reader - Flushes the RX FIFO buffer. This bit is automatically cleared by hardware when flush is completed."]
pub type RX_FLUSH_R = crate::BitReader;
#[doc = "Field `RX_FLUSH` writer - Flushes the RX FIFO buffer. This bit is automatically cleared by hardware when flush is completed."]
pub type RX_FLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAR_SIZE` reader - Selects UART character size"]
pub type CHAR_SIZE_R = crate::FieldReader<CHAR_SIZE_A>;
#[doc = "Selects UART character size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHAR_SIZE_A {
    #[doc = "0: 5 bits"]
    _5BITS = 0,
    #[doc = "1: 6 bits"]
    _6BITS = 1,
    #[doc = "2: 7 bits"]
    _7BITS = 2,
    #[doc = "3: 8 bits"]
    _8BITS = 3,
}
impl From<CHAR_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CHAR_SIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHAR_SIZE_A {
    type Ux = u8;
}
impl CHAR_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHAR_SIZE_A {
        match self.bits {
            0 => CHAR_SIZE_A::_5BITS,
            1 => CHAR_SIZE_A::_6BITS,
            2 => CHAR_SIZE_A::_7BITS,
            3 => CHAR_SIZE_A::_8BITS,
            _ => unreachable!(),
        }
    }
    #[doc = "5 bits"]
    #[inline(always)]
    pub fn is_5bits(&self) -> bool {
        *self == CHAR_SIZE_A::_5BITS
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn is_6bits(&self) -> bool {
        *self == CHAR_SIZE_A::_6BITS
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn is_7bits(&self) -> bool {
        *self == CHAR_SIZE_A::_7BITS
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_8bits(&self) -> bool {
        *self == CHAR_SIZE_A::_8BITS
    }
}
#[doc = "Field `CHAR_SIZE` writer - Selects UART character size"]
pub type CHAR_SIZE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CHAR_SIZE_A>;
impl<'a, REG> CHAR_SIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "5 bits"]
    #[inline(always)]
    pub fn _5bits(self) -> &'a mut crate::W<REG> {
        self.variant(CHAR_SIZE_A::_5BITS)
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn _6bits(self) -> &'a mut crate::W<REG> {
        self.variant(CHAR_SIZE_A::_6BITS)
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn _7bits(self) -> &'a mut crate::W<REG> {
        self.variant(CHAR_SIZE_A::_7BITS)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _8bits(self) -> &'a mut crate::W<REG> {
        self.variant(CHAR_SIZE_A::_8BITS)
    }
}
#[doc = "Field `STOPBITS` reader - Selects the number of stop bits that will be generated"]
pub type STOPBITS_R = crate::BitReader;
#[doc = "Field `STOPBITS` writer - Selects the number of stop bits that will be generated"]
pub type STOPBITS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFC_EN` reader - Enables/disables hardware flow control"]
pub type HFC_EN_R = crate::BitReader;
#[doc = "Field `HFC_EN` writer - Enables/disables hardware flow control"]
pub type HFC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSDC` reader - Hardware Flow Control RTS Mode"]
pub type RTSDC_R = crate::BitReader;
#[doc = "Field `RTSDC` writer - Hardware Flow Control RTS Mode"]
pub type RTSDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCLKEN` reader - Baud clock enable"]
pub type BCLKEN_R = crate::BitReader;
#[doc = "Field `BCLKEN` writer - Baud clock enable"]
pub type BCLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCLKSRC` reader - To select the UART clock source for the UART engine (except APB registers). Secondary clock (used for baud rate generator) can be asynchronous from APB clock."]
pub type BCLKSRC_R = crate::FieldReader<BCLKSRC_A>;
#[doc = "To select the UART clock source for the UART engine (except APB registers). Secondary clock (used for baud rate generator) can be asynchronous from APB clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BCLKSRC_A {
    #[doc = "0: apb clock"]
    PERIPHERAL_CLOCK = 0,
    #[doc = "1: Clock 1"]
    EXTERNAL_CLOCK = 1,
    #[doc = "2: Clock 2"]
    CLK2 = 2,
    #[doc = "3: Clock 3"]
    CLK3 = 3,
}
impl From<BCLKSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: BCLKSRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BCLKSRC_A {
    type Ux = u8;
}
impl BCLKSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BCLKSRC_A {
        match self.bits {
            0 => BCLKSRC_A::PERIPHERAL_CLOCK,
            1 => BCLKSRC_A::EXTERNAL_CLOCK,
            2 => BCLKSRC_A::CLK2,
            3 => BCLKSRC_A::CLK3,
            _ => unreachable!(),
        }
    }
    #[doc = "apb clock"]
    #[inline(always)]
    pub fn is_peripheral_clock(&self) -> bool {
        *self == BCLKSRC_A::PERIPHERAL_CLOCK
    }
    #[doc = "Clock 1"]
    #[inline(always)]
    pub fn is_external_clock(&self) -> bool {
        *self == BCLKSRC_A::EXTERNAL_CLOCK
    }
    #[doc = "Clock 2"]
    #[inline(always)]
    pub fn is_clk2(&self) -> bool {
        *self == BCLKSRC_A::CLK2
    }
    #[doc = "Clock 3"]
    #[inline(always)]
    pub fn is_clk3(&self) -> bool {
        *self == BCLKSRC_A::CLK3
    }
}
#[doc = "Field `BCLKSRC` writer - To select the UART clock source for the UART engine (except APB registers). Secondary clock (used for baud rate generator) can be asynchronous from APB clock."]
pub type BCLKSRC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, BCLKSRC_A>;
impl<'a, REG> BCLKSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "apb clock"]
    #[inline(always)]
    pub fn peripheral_clock(self) -> &'a mut crate::W<REG> {
        self.variant(BCLKSRC_A::PERIPHERAL_CLOCK)
    }
    #[doc = "Clock 1"]
    #[inline(always)]
    pub fn external_clock(self) -> &'a mut crate::W<REG> {
        self.variant(BCLKSRC_A::EXTERNAL_CLOCK)
    }
    #[doc = "Clock 2"]
    #[inline(always)]
    pub fn clk2(self) -> &'a mut crate::W<REG> {
        self.variant(BCLKSRC_A::CLK2)
    }
    #[doc = "Clock 3"]
    #[inline(always)]
    pub fn clk3(self) -> &'a mut crate::W<REG> {
        self.variant(BCLKSRC_A::CLK3)
    }
}
#[doc = "Field `DPFE_EN` reader - Data/Parity bit frame error detection enable"]
pub type DPFE_EN_R = crate::BitReader;
#[doc = "Field `DPFE_EN` writer - Data/Parity bit frame error detection enable"]
pub type DPFE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCLKRDY` reader - Baud clock Ready read only bit"]
pub type BCLKRDY_R = crate::BitReader;
#[doc = "Field `BCLKRDY` writer - Baud clock Ready read only bit"]
pub type BCLKRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCAGM` reader - UART Clock Auto Gating mode"]
pub type UCAGM_R = crate::BitReader;
#[doc = "Field `UCAGM` writer - UART Clock Auto Gating mode"]
pub type UCAGM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDM` reader - Fractional Division Mode"]
pub type FDM_R = crate::BitReader;
#[doc = "Field `FDM` writer - Fractional Division Mode"]
pub type FDM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESM` reader - RX Dual Edge Sampling Mode"]
pub type DESM_R = crate::BitReader;
#[doc = "Field `DESM` writer - RX Dual Edge Sampling Mode"]
pub type DESM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - This field specifies the depth of receive FIFO for interrupt generation (value 0 and > 16 are ignored)"]
    #[inline(always)]
    pub fn rx_thd_val(&self) -> RX_THD_VAL_R {
        RX_THD_VAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Parity Enable"]
    #[inline(always)]
    pub fn par_en(&self) -> PAR_EN_R {
        PAR_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - when PAREN=1 selects odd or even parity odd is 1 even is 0"]
    #[inline(always)]
    pub fn par_eo(&self) -> PAR_EO_R {
        PAR_EO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects parity based on 1s or 0s count (when PAREN=1)"]
    #[inline(always)]
    pub fn par_md(&self) -> PAR_MD_R {
        PAR_MD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CTS Sampling Disable"]
    #[inline(always)]
    pub fn cts_dis(&self) -> CTS_DIS_R {
        CTS_DIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Flushes the TX FIFO buffer. This bit is automatically cleared by hardware when flush is completed."]
    #[inline(always)]
    pub fn tx_flush(&self) -> TX_FLUSH_R {
        TX_FLUSH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Flushes the RX FIFO buffer. This bit is automatically cleared by hardware when flush is completed."]
    #[inline(always)]
    pub fn rx_flush(&self) -> RX_FLUSH_R {
        RX_FLUSH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Selects UART character size"]
    #[inline(always)]
    pub fn char_size(&self) -> CHAR_SIZE_R {
        CHAR_SIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Selects the number of stop bits that will be generated"]
    #[inline(always)]
    pub fn stopbits(&self) -> STOPBITS_R {
        STOPBITS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables/disables hardware flow control"]
    #[inline(always)]
    pub fn hfc_en(&self) -> HFC_EN_R {
        HFC_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Hardware Flow Control RTS Mode"]
    #[inline(always)]
    pub fn rtsdc(&self) -> RTSDC_R {
        RTSDC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Baud clock enable"]
    #[inline(always)]
    pub fn bclken(&self) -> BCLKEN_R {
        BCLKEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - To select the UART clock source for the UART engine (except APB registers). Secondary clock (used for baud rate generator) can be asynchronous from APB clock."]
    #[inline(always)]
    pub fn bclksrc(&self) -> BCLKSRC_R {
        BCLKSRC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Data/Parity bit frame error detection enable"]
    #[inline(always)]
    pub fn dpfe_en(&self) -> DPFE_EN_R {
        DPFE_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Baud clock Ready read only bit"]
    #[inline(always)]
    pub fn bclkrdy(&self) -> BCLKRDY_R {
        BCLKRDY_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART Clock Auto Gating mode"]
    #[inline(always)]
    pub fn ucagm(&self) -> UCAGM_R {
        UCAGM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Fractional Division Mode"]
    #[inline(always)]
    pub fn fdm(&self) -> FDM_R {
        FDM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - RX Dual Edge Sampling Mode"]
    #[inline(always)]
    pub fn desm(&self) -> DESM_R {
        DESM_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - This field specifies the depth of receive FIFO for interrupt generation (value 0 and > 16 are ignored)"]
    #[inline(always)]
    #[must_use]
    pub fn rx_thd_val(&mut self) -> RX_THD_VAL_W<CTRL_SPEC> {
        RX_THD_VAL_W::new(self, 0)
    }
    #[doc = "Bit 4 - Parity Enable"]
    #[inline(always)]
    #[must_use]
    pub fn par_en(&mut self) -> PAR_EN_W<CTRL_SPEC> {
        PAR_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - when PAREN=1 selects odd or even parity odd is 1 even is 0"]
    #[inline(always)]
    #[must_use]
    pub fn par_eo(&mut self) -> PAR_EO_W<CTRL_SPEC> {
        PAR_EO_W::new(self, 5)
    }
    #[doc = "Bit 6 - Selects parity based on 1s or 0s count (when PAREN=1)"]
    #[inline(always)]
    #[must_use]
    pub fn par_md(&mut self) -> PAR_MD_W<CTRL_SPEC> {
        PAR_MD_W::new(self, 6)
    }
    #[doc = "Bit 7 - CTS Sampling Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cts_dis(&mut self) -> CTS_DIS_W<CTRL_SPEC> {
        CTS_DIS_W::new(self, 7)
    }
    #[doc = "Bit 8 - Flushes the TX FIFO buffer. This bit is automatically cleared by hardware when flush is completed."]
    #[inline(always)]
    #[must_use]
    pub fn tx_flush(&mut self) -> TX_FLUSH_W<CTRL_SPEC> {
        TX_FLUSH_W::new(self, 8)
    }
    #[doc = "Bit 9 - Flushes the RX FIFO buffer. This bit is automatically cleared by hardware when flush is completed."]
    #[inline(always)]
    #[must_use]
    pub fn rx_flush(&mut self) -> RX_FLUSH_W<CTRL_SPEC> {
        RX_FLUSH_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - Selects UART character size"]
    #[inline(always)]
    #[must_use]
    pub fn char_size(&mut self) -> CHAR_SIZE_W<CTRL_SPEC> {
        CHAR_SIZE_W::new(self, 10)
    }
    #[doc = "Bit 12 - Selects the number of stop bits that will be generated"]
    #[inline(always)]
    #[must_use]
    pub fn stopbits(&mut self) -> STOPBITS_W<CTRL_SPEC> {
        STOPBITS_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enables/disables hardware flow control"]
    #[inline(always)]
    #[must_use]
    pub fn hfc_en(&mut self) -> HFC_EN_W<CTRL_SPEC> {
        HFC_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Hardware Flow Control RTS Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rtsdc(&mut self) -> RTSDC_W<CTRL_SPEC> {
        RTSDC_W::new(self, 14)
    }
    #[doc = "Bit 15 - Baud clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn bclken(&mut self) -> BCLKEN_W<CTRL_SPEC> {
        BCLKEN_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - To select the UART clock source for the UART engine (except APB registers). Secondary clock (used for baud rate generator) can be asynchronous from APB clock."]
    #[inline(always)]
    #[must_use]
    pub fn bclksrc(&mut self) -> BCLKSRC_W<CTRL_SPEC> {
        BCLKSRC_W::new(self, 16)
    }
    #[doc = "Bit 18 - Data/Parity bit frame error detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpfe_en(&mut self) -> DPFE_EN_W<CTRL_SPEC> {
        DPFE_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Baud clock Ready read only bit"]
    #[inline(always)]
    #[must_use]
    pub fn bclkrdy(&mut self) -> BCLKRDY_W<CTRL_SPEC> {
        BCLKRDY_W::new(self, 19)
    }
    #[doc = "Bit 20 - UART Clock Auto Gating mode"]
    #[inline(always)]
    #[must_use]
    pub fn ucagm(&mut self) -> UCAGM_W<CTRL_SPEC> {
        UCAGM_W::new(self, 20)
    }
    #[doc = "Bit 21 - Fractional Division Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fdm(&mut self) -> FDM_W<CTRL_SPEC> {
        FDM_W::new(self, 21)
    }
    #[doc = "Bit 22 - RX Dual Edge Sampling Mode"]
    #[inline(always)]
    #[must_use]
    pub fn desm(&mut self) -> DESM_W<CTRL_SPEC> {
        DESM_W::new(self, 22)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
