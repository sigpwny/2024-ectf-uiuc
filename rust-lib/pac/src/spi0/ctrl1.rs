#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `TX_NUM_CHAR` reader - Nubmer of Characters to transmit."]
pub type TX_NUM_CHAR_R = crate::FieldReader<u16>;
#[doc = "Field `TX_NUM_CHAR` writer - Nubmer of Characters to transmit."]
pub type TX_NUM_CHAR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RX_NUM_CHAR` reader - Nubmer of Characters to receive."]
pub type RX_NUM_CHAR_R = crate::FieldReader<u16>;
#[doc = "Field `RX_NUM_CHAR` writer - Nubmer of Characters to receive."]
pub type RX_NUM_CHAR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Nubmer of Characters to transmit."]
    #[inline(always)]
    pub fn tx_num_char(&self) -> TX_NUM_CHAR_R {
        TX_NUM_CHAR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Nubmer of Characters to receive."]
    #[inline(always)]
    pub fn rx_num_char(&self) -> RX_NUM_CHAR_R {
        RX_NUM_CHAR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Nubmer of Characters to transmit."]
    #[inline(always)]
    #[must_use]
    pub fn tx_num_char(&mut self) -> TX_NUM_CHAR_W<CTRL1_SPEC> {
        TX_NUM_CHAR_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Nubmer of Characters to receive."]
    #[inline(always)]
    #[must_use]
    pub fn rx_num_char(&mut self) -> RX_NUM_CHAR_W<CTRL1_SPEC> {
        RX_NUM_CHAR_W::new(self, 16)
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
#[doc = "Register for controlling SPI peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
