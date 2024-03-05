#[doc = "Register `RST1` reader"]
pub type R = crate::R<RST1_SPEC>;
#[doc = "Register `RST1` writer"]
pub type W = crate::W<RST1_SPEC>;
#[doc = "Field `I2C1` reader - I2C1 Reset."]
pub type I2C1_R = crate::BitReader<RESET_READ_A>;
#[doc = "I2C1 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_READ_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Starts reset or indicates reset in progress."]
    BUSY = 1,
}
impl From<RESET_READ_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESET_READ_A {
        match self.bits {
            false => RESET_READ_A::RESET_DONE,
            true => RESET_READ_A::BUSY,
        }
    }
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == RESET_READ_A::RESET_DONE
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RESET_READ_A::BUSY
    }
}
#[doc = "Field `I2C1` writer - I2C1 Reset."]
pub type I2C1_W<'a, REG> = crate::BitWriter<'a, REG, RESET_READ_A>;
impl<'a, REG> I2C1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn reset_done(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::RESET_DONE)
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::BUSY)
    }
}
#[doc = "Field `PT` reader - PT Reset."]
pub use I2C1_R as PT_R;
#[doc = "Field `OWM` reader - OWM Reset."]
pub use I2C1_R as OWM_R;
#[doc = "Field `CRC` reader - CRC Reset."]
pub use I2C1_R as CRC_R;
#[doc = "Field `AES` reader - AES Reset."]
pub use I2C1_R as AES_R;
#[doc = "Field `SPI0` reader - SPI 0 Reset."]
pub use I2C1_R as SPI0_R;
#[doc = "Field `SMPHR` reader - SMPHR Reset."]
pub use I2C1_R as SMPHR_R;
#[doc = "Field `I2S` reader - I2S Reset."]
pub use I2C1_R as I2S_R;
#[doc = "Field `I2C2` reader - I2C2 Reset."]
pub use I2C1_R as I2C2_R;
#[doc = "Field `DVS` reader - DVS Reset."]
pub use I2C1_R as DVS_R;
#[doc = "Field `SIMO` reader - SIMO Reset."]
pub use I2C1_R as SIMO_R;
#[doc = "Field `CPU1` reader - CPU1 Reset."]
pub use I2C1_R as CPU1_R;
#[doc = "Field `PT` writer - PT Reset."]
pub use I2C1_W as PT_W;
#[doc = "Field `OWM` writer - OWM Reset."]
pub use I2C1_W as OWM_W;
#[doc = "Field `CRC` writer - CRC Reset."]
pub use I2C1_W as CRC_W;
#[doc = "Field `AES` writer - AES Reset."]
pub use I2C1_W as AES_W;
#[doc = "Field `SPI0` writer - SPI 0 Reset."]
pub use I2C1_W as SPI0_W;
#[doc = "Field `SMPHR` writer - SMPHR Reset."]
pub use I2C1_W as SMPHR_W;
#[doc = "Field `I2S` writer - I2S Reset."]
pub use I2C1_W as I2S_W;
#[doc = "Field `I2C2` writer - I2C2 Reset."]
pub use I2C1_W as I2C2_W;
#[doc = "Field `DVS` writer - DVS Reset."]
pub use I2C1_W as DVS_W;
#[doc = "Field `SIMO` writer - SIMO Reset."]
pub use I2C1_W as SIMO_W;
#[doc = "Field `CPU1` writer - CPU1 Reset."]
pub use I2C1_W as CPU1_W;
impl R {
    #[doc = "Bit 0 - I2C1 Reset."]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PT Reset."]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - OWM Reset."]
    #[inline(always)]
    pub fn owm(&self) -> OWM_R {
        OWM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - CRC Reset."]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AES Reset."]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SPI 0 Reset."]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - SMPHR Reset."]
    #[inline(always)]
    pub fn smphr(&self) -> SMPHR_R {
        SMPHR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - I2S Reset."]
    #[inline(always)]
    pub fn i2s(&self) -> I2S_R {
        I2S_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I2C2 Reset."]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - DVS Reset."]
    #[inline(always)]
    pub fn dvs(&self) -> DVS_R {
        DVS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SIMO Reset."]
    #[inline(always)]
    pub fn simo(&self) -> SIMO_R {
        SIMO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU1 Reset."]
    #[inline(always)]
    pub fn cpu1(&self) -> CPU1_R {
        CPU1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C1 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<RST1_SPEC> {
        I2C1_W::new(self, 0)
    }
    #[doc = "Bit 1 - PT Reset."]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PT_W<RST1_SPEC> {
        PT_W::new(self, 1)
    }
    #[doc = "Bit 7 - OWM Reset."]
    #[inline(always)]
    #[must_use]
    pub fn owm(&mut self) -> OWM_W<RST1_SPEC> {
        OWM_W::new(self, 7)
    }
    #[doc = "Bit 9 - CRC Reset."]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<RST1_SPEC> {
        CRC_W::new(self, 9)
    }
    #[doc = "Bit 10 - AES Reset."]
    #[inline(always)]
    #[must_use]
    pub fn aes(&mut self) -> AES_W<RST1_SPEC> {
        AES_W::new(self, 10)
    }
    #[doc = "Bit 11 - SPI 0 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<RST1_SPEC> {
        SPI0_W::new(self, 11)
    }
    #[doc = "Bit 16 - SMPHR Reset."]
    #[inline(always)]
    #[must_use]
    pub fn smphr(&mut self) -> SMPHR_W<RST1_SPEC> {
        SMPHR_W::new(self, 16)
    }
    #[doc = "Bit 19 - I2S Reset."]
    #[inline(always)]
    #[must_use]
    pub fn i2s(&mut self) -> I2S_W<RST1_SPEC> {
        I2S_W::new(self, 19)
    }
    #[doc = "Bit 20 - I2C2 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2(&mut self) -> I2C2_W<RST1_SPEC> {
        I2C2_W::new(self, 20)
    }
    #[doc = "Bit 24 - DVS Reset."]
    #[inline(always)]
    #[must_use]
    pub fn dvs(&mut self) -> DVS_W<RST1_SPEC> {
        DVS_W::new(self, 24)
    }
    #[doc = "Bit 25 - SIMO Reset."]
    #[inline(always)]
    #[must_use]
    pub fn simo(&mut self) -> SIMO_W<RST1_SPEC> {
        SIMO_W::new(self, 25)
    }
    #[doc = "Bit 31 - CPU1 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn cpu1(&mut self) -> CPU1_W<RST1_SPEC> {
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
#[doc = "Reset 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RST1_SPEC;
impl crate::RegisterSpec for RST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst1::R`](R) reader structure"]
impl crate::Readable for RST1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rst1::W`](W) writer structure"]
impl crate::Writable for RST1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RST1 to value 0"]
impl crate::Resettable for RST1_SPEC {
    const RESET_VALUE: u32 = 0;
}
