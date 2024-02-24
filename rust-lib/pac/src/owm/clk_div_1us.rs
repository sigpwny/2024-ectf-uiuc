#[doc = "Register `CLK_DIV_1US` reader"]
pub type R = crate::R<CLK_DIV_1US_SPEC>;
#[doc = "Register `CLK_DIV_1US` writer"]
pub type W = crate::W<CLK_DIV_1US_SPEC>;
#[doc = "Field `divisor` reader - Clock Divisor for 1Mhz."]
pub type DIVISOR_R = crate::FieldReader;
#[doc = "Field `divisor` writer - Clock Divisor for 1Mhz."]
pub type DIVISOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clock Divisor for 1Mhz."]
    #[inline(always)]
    pub fn divisor(&self) -> DIVISOR_R {
        DIVISOR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Divisor for 1Mhz."]
    #[inline(always)]
    #[must_use]
    pub fn divisor(&mut self) -> DIVISOR_W<CLK_DIV_1US_SPEC> {
        DIVISOR_W::new(self, 0)
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
#[doc = "1-Wire Master Clock Divisor.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_div_1us::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_div_1us::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_DIV_1US_SPEC;
impl crate::RegisterSpec for CLK_DIV_1US_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_div_1us::R`](R) reader structure"]
impl crate::Readable for CLK_DIV_1US_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_div_1us::W`](W) writer structure"]
impl crate::Writable for CLK_DIV_1US_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_DIV_1US to value 0"]
impl crate::Resettable for CLK_DIV_1US_SPEC {
    const RESET_VALUE: u32 = 0;
}
