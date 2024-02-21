#[doc = "Register `PCLKDIS1` reader"]
pub type R = crate::R<PCLKDIS1_SPEC>;
#[doc = "Register `PCLKDIS1` writer"]
pub type W = crate::W<PCLKDIS1_SPEC>;
#[doc = "Field `UART2` reader - UART2 Clock Disable."]
pub type UART2_R = crate::BitReader<UART2_A>;
#[doc = "UART2 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART2_A {
    #[doc = "0: Enable."]
    EN = 0,
    #[doc = "1: Disable."]
    DIS = 1,
}
impl From<UART2_A> for bool {
    #[inline(always)]
    fn from(variant: UART2_A) -> Self {
        variant as u8 != 0
    }
}
impl UART2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UART2_A {
        match self.bits {
            false => UART2_A::EN,
            true => UART2_A::DIS,
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == UART2_A::EN
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == UART2_A::DIS
    }
}
#[doc = "Field `UART2` writer - UART2 Clock Disable."]
pub type UART2_W<'a, REG> = crate::BitWriter<'a, REG, UART2_A>;
impl<'a, REG> UART2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(UART2_A::EN)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(UART2_A::DIS)
    }
}
#[doc = "Field `TRNG` reader - TRNG Clock Disable."]
pub use UART2_R as TRNG_R;
#[doc = "Field `SMPHR` reader - SMPHR Clock Disable."]
pub use UART2_R as SMPHR_R;
#[doc = "Field `OWM` reader - One-Wire Clock Disable."]
pub use UART2_R as OWM_R;
#[doc = "Field `CRC` reader - CRC Clock Disable."]
pub use UART2_R as CRC_R;
#[doc = "Field `AES` reader - AES Clock Disable."]
pub use UART2_R as AES_R;
#[doc = "Field `SPI0` reader - SPI 0 Clock Disable."]
pub use UART2_R as SPI0_R;
#[doc = "Field `PCIF` reader - Parallel Camera Interface Clock Disable."]
pub use UART2_R as PCIF_R;
#[doc = "Field `I2S` reader - I2S Clock Disable."]
pub use UART2_R as I2S_R;
#[doc = "Field `I2C2` reader - I2C2 Clock Disable."]
pub use UART2_R as I2C2_R;
#[doc = "Field `WDT0` reader - Watch Dog Timer 0 Clock Disable."]
pub use UART2_R as WDT0_R;
#[doc = "Field `CPU1` reader - CPU1 Clock Disable."]
pub use UART2_R as CPU1_R;
#[doc = "Field `TRNG` writer - TRNG Clock Disable."]
pub use UART2_W as TRNG_W;
#[doc = "Field `SMPHR` writer - SMPHR Clock Disable."]
pub use UART2_W as SMPHR_W;
#[doc = "Field `OWM` writer - One-Wire Clock Disable."]
pub use UART2_W as OWM_W;
#[doc = "Field `CRC` writer - CRC Clock Disable."]
pub use UART2_W as CRC_W;
#[doc = "Field `AES` writer - AES Clock Disable."]
pub use UART2_W as AES_W;
#[doc = "Field `SPI0` writer - SPI 0 Clock Disable."]
pub use UART2_W as SPI0_W;
#[doc = "Field `PCIF` writer - Parallel Camera Interface Clock Disable."]
pub use UART2_W as PCIF_W;
#[doc = "Field `I2S` writer - I2S Clock Disable."]
pub use UART2_W as I2S_W;
#[doc = "Field `I2C2` writer - I2C2 Clock Disable."]
pub use UART2_W as I2C2_W;
#[doc = "Field `WDT0` writer - Watch Dog Timer 0 Clock Disable."]
pub use UART2_W as WDT0_W;
#[doc = "Field `CPU1` writer - CPU1 Clock Disable."]
pub use UART2_W as CPU1_W;
impl R {
    #[doc = "Bit 1 - UART2 Clock Disable."]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TRNG Clock Disable."]
    #[inline(always)]
    pub fn trng(&self) -> TRNG_R {
        TRNG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - SMPHR Clock Disable."]
    #[inline(always)]
    pub fn smphr(&self) -> SMPHR_R {
        SMPHR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - One-Wire Clock Disable."]
    #[inline(always)]
    pub fn owm(&self) -> OWM_R {
        OWM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CRC Clock Disable."]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AES Clock Disable."]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SPI 0 Clock Disable."]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Parallel Camera Interface Clock Disable."]
    #[inline(always)]
    pub fn pcif(&self) -> PCIF_R {
        PCIF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 23 - I2S Clock Disable."]
    #[inline(always)]
    pub fn i2s(&self) -> I2S_R {
        I2S_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - I2C2 Clock Disable."]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - Watch Dog Timer 0 Clock Disable."]
    #[inline(always)]
    pub fn wdt0(&self) -> WDT0_R {
        WDT0_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU1 Clock Disable."]
    #[inline(always)]
    pub fn cpu1(&self) -> CPU1_R {
        CPU1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - UART2 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn uart2(&mut self) -> UART2_W<PCLKDIS1_SPEC> {
        UART2_W::new(self, 1)
    }
    #[doc = "Bit 2 - TRNG Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn trng(&mut self) -> TRNG_W<PCLKDIS1_SPEC> {
        TRNG_W::new(self, 2)
    }
    #[doc = "Bit 9 - SMPHR Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn smphr(&mut self) -> SMPHR_W<PCLKDIS1_SPEC> {
        SMPHR_W::new(self, 9)
    }
    #[doc = "Bit 13 - One-Wire Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn owm(&mut self) -> OWM_W<PCLKDIS1_SPEC> {
        OWM_W::new(self, 13)
    }
    #[doc = "Bit 14 - CRC Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<PCLKDIS1_SPEC> {
        CRC_W::new(self, 14)
    }
    #[doc = "Bit 15 - AES Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn aes(&mut self) -> AES_W<PCLKDIS1_SPEC> {
        AES_W::new(self, 15)
    }
    #[doc = "Bit 16 - SPI 0 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<PCLKDIS1_SPEC> {
        SPI0_W::new(self, 16)
    }
    #[doc = "Bit 18 - Parallel Camera Interface Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn pcif(&mut self) -> PCIF_W<PCLKDIS1_SPEC> {
        PCIF_W::new(self, 18)
    }
    #[doc = "Bit 23 - I2S Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn i2s(&mut self) -> I2S_W<PCLKDIS1_SPEC> {
        I2S_W::new(self, 23)
    }
    #[doc = "Bit 24 - I2C2 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2(&mut self) -> I2C2_W<PCLKDIS1_SPEC> {
        I2C2_W::new(self, 24)
    }
    #[doc = "Bit 27 - Watch Dog Timer 0 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn wdt0(&mut self) -> WDT0_W<PCLKDIS1_SPEC> {
        WDT0_W::new(self, 27)
    }
    #[doc = "Bit 31 - CPU1 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn cpu1(&mut self) -> CPU1_W<PCLKDIS1_SPEC> {
        CPU1_W::new(self, 31)
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
#[doc = "Peripheral Clock Disable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pclkdis1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pclkdis1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCLKDIS1_SPEC;
impl crate::RegisterSpec for PCLKDIS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pclkdis1::R`](R) reader structure"]
impl crate::Readable for PCLKDIS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pclkdis1::W`](W) writer structure"]
impl crate::Writable for PCLKDIS1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCLKDIS1 to value 0"]
impl crate::Resettable for PCLKDIS1_SPEC {
    const RESET_VALUE: u32 = 0;
}
