#[doc = "Register `PCLKDIS0` reader"]
pub type R = crate::R<PCLKDIS0_SPEC>;
#[doc = "Register `PCLKDIS0` writer"]
pub type W = crate::W<PCLKDIS0_SPEC>;
#[doc = "Field `GPIO0` reader - GPIO0 Clock Disable."]
pub type GPIO0_R = crate::BitReader<GPIO0_A>;
#[doc = "GPIO0 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO0_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<GPIO0_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIO0_A {
        match self.bits {
            false => GPIO0_A::EN,
            true => GPIO0_A::DIS,
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GPIO0_A::EN
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO0_A::DIS
    }
}
#[doc = "Field `GPIO0` writer - GPIO0 Clock Disable."]
pub type GPIO0_W<'a, REG> = crate::BitWriter<'a, REG, GPIO0_A>;
impl<'a, REG> GPIO0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO0_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO0_A::DIS)
    }
}
#[doc = "Field `GPIO1` reader - GPIO1 Clock Disable."]
pub use GPIO0_R as GPIO1_R;
#[doc = "Field `DMA` reader - DMA Clock Disable."]
pub use GPIO0_R as DMA_R;
#[doc = "Field `SPI1` reader - SPI 1 Clock Disable."]
pub use GPIO0_R as SPI1_R;
#[doc = "Field `UART0` reader - UART 0 Clock Disable."]
pub use GPIO0_R as UART0_R;
#[doc = "Field `UART1` reader - UART 1 Clock Disable."]
pub use GPIO0_R as UART1_R;
#[doc = "Field `I2C0` reader - I2C 0 Clock Disable."]
pub use GPIO0_R as I2C0_R;
#[doc = "Field `TMR0` reader - Timer 0 Clock Disable."]
pub use GPIO0_R as TMR0_R;
#[doc = "Field `TMR1` reader - Timer 1 Clock Disable."]
pub use GPIO0_R as TMR1_R;
#[doc = "Field `TMR2` reader - Timer 2 Clock Disable."]
pub use GPIO0_R as TMR2_R;
#[doc = "Field `TMR3` reader - Timer 3 Clock Disable."]
pub use GPIO0_R as TMR3_R;
#[doc = "Field `ADC` reader - ADC Clock Disable."]
pub use GPIO0_R as ADC_R;
#[doc = "Field `CNN` reader - CNN Clock Disable."]
pub use GPIO0_R as CNN_R;
#[doc = "Field `I2C1` reader - I2C 1 Clock Disable."]
pub use GPIO0_R as I2C1_R;
#[doc = "Field `PT` reader - Pluse Train Clock Disable."]
pub use GPIO0_R as PT_R;
#[doc = "Field `GPIO1` writer - GPIO1 Clock Disable."]
pub use GPIO0_W as GPIO1_W;
#[doc = "Field `DMA` writer - DMA Clock Disable."]
pub use GPIO0_W as DMA_W;
#[doc = "Field `SPI1` writer - SPI 1 Clock Disable."]
pub use GPIO0_W as SPI1_W;
#[doc = "Field `UART0` writer - UART 0 Clock Disable."]
pub use GPIO0_W as UART0_W;
#[doc = "Field `UART1` writer - UART 1 Clock Disable."]
pub use GPIO0_W as UART1_W;
#[doc = "Field `I2C0` writer - I2C 0 Clock Disable."]
pub use GPIO0_W as I2C0_W;
#[doc = "Field `TMR0` writer - Timer 0 Clock Disable."]
pub use GPIO0_W as TMR0_W;
#[doc = "Field `TMR1` writer - Timer 1 Clock Disable."]
pub use GPIO0_W as TMR1_W;
#[doc = "Field `TMR2` writer - Timer 2 Clock Disable."]
pub use GPIO0_W as TMR2_W;
#[doc = "Field `TMR3` writer - Timer 3 Clock Disable."]
pub use GPIO0_W as TMR3_W;
#[doc = "Field `ADC` writer - ADC Clock Disable."]
pub use GPIO0_W as ADC_W;
#[doc = "Field `CNN` writer - CNN Clock Disable."]
pub use GPIO0_W as CNN_W;
#[doc = "Field `I2C1` writer - I2C 1 Clock Disable."]
pub use GPIO0_W as I2C1_W;
#[doc = "Field `PT` writer - Pluse Train Clock Disable."]
pub use GPIO0_W as PT_W;
impl R {
    #[doc = "Bit 0 - GPIO0 Clock Disable."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO1 Clock Disable."]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO1_R {
        GPIO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Clock Disable."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI 1 Clock Disable."]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - UART 0 Clock Disable."]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART 1 Clock Disable."]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - I2C 0 Clock Disable."]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Timer 0 Clock Disable."]
    #[inline(always)]
    pub fn tmr0(&self) -> TMR0_R {
        TMR0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer 1 Clock Disable."]
    #[inline(always)]
    pub fn tmr1(&self) -> TMR1_R {
        TMR1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer 2 Clock Disable."]
    #[inline(always)]
    pub fn tmr2(&self) -> TMR2_R {
        TMR2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer 3 Clock Disable."]
    #[inline(always)]
    pub fn tmr3(&self) -> TMR3_R {
        TMR3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 23 - ADC Clock Disable."]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CNN Clock Disable."]
    #[inline(always)]
    pub fn cnn(&self) -> CNN_R {
        CNN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - I2C 1 Clock Disable."]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Pluse Train Clock Disable."]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO0 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn gpio0(&mut self) -> GPIO0_W<PCLKDIS0_SPEC> {
        GPIO0_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO1 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn gpio1(&mut self) -> GPIO1_W<PCLKDIS0_SPEC> {
        GPIO1_W::new(self, 1)
    }
    #[doc = "Bit 5 - DMA Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<PCLKDIS0_SPEC> {
        DMA_W::new(self, 5)
    }
    #[doc = "Bit 6 - SPI 1 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<PCLKDIS0_SPEC> {
        SPI1_W::new(self, 6)
    }
    #[doc = "Bit 9 - UART 0 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> UART0_W<PCLKDIS0_SPEC> {
        UART0_W::new(self, 9)
    }
    #[doc = "Bit 10 - UART 1 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<PCLKDIS0_SPEC> {
        UART1_W::new(self, 10)
    }
    #[doc = "Bit 13 - I2C 0 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2C0_W<PCLKDIS0_SPEC> {
        I2C0_W::new(self, 13)
    }
    #[doc = "Bit 15 - Timer 0 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn tmr0(&mut self) -> TMR0_W<PCLKDIS0_SPEC> {
        TMR0_W::new(self, 15)
    }
    #[doc = "Bit 16 - Timer 1 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn tmr1(&mut self) -> TMR1_W<PCLKDIS0_SPEC> {
        TMR1_W::new(self, 16)
    }
    #[doc = "Bit 17 - Timer 2 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn tmr2(&mut self) -> TMR2_W<PCLKDIS0_SPEC> {
        TMR2_W::new(self, 17)
    }
    #[doc = "Bit 18 - Timer 3 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn tmr3(&mut self) -> TMR3_W<PCLKDIS0_SPEC> {
        TMR3_W::new(self, 18)
    }
    #[doc = "Bit 23 - ADC Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn adc(&mut self) -> ADC_W<PCLKDIS0_SPEC> {
        ADC_W::new(self, 23)
    }
    #[doc = "Bit 25 - CNN Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn cnn(&mut self) -> CNN_W<PCLKDIS0_SPEC> {
        CNN_W::new(self, 25)
    }
    #[doc = "Bit 28 - I2C 1 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<PCLKDIS0_SPEC> {
        I2C1_W::new(self, 28)
    }
    #[doc = "Bit 29 - Pluse Train Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PT_W<PCLKDIS0_SPEC> {
        PT_W::new(self, 29)
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
#[doc = "Peripheral Clock Disable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pclkdis0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pclkdis0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCLKDIS0_SPEC;
impl crate::RegisterSpec for PCLKDIS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pclkdis0::R`](R) reader structure"]
impl crate::Readable for PCLKDIS0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pclkdis0::W`](W) writer structure"]
impl crate::Writable for PCLKDIS0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCLKDIS0 to value 0"]
impl crate::Resettable for PCLKDIS0_SPEC {
    const RESET_VALUE: u32 = 0;
}
