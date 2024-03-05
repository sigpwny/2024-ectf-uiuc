#[doc = "Register `DS1` reader"]
pub type R = crate::R<DS1_SPEC>;
#[doc = "Register `DS1` writer"]
pub type W = crate::W<DS1_SPEC>;
#[doc = "Field `GPIO_DS1` reader - Mask of all of the pins on the port."]
pub type GPIO_DS1_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_DS1` writer - Mask of all of the pins on the port."]
pub type GPIO_DS1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_ds1(&self) -> GPIO_DS1_R {
        GPIO_DS1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_ds1(&mut self) -> GPIO_DS1_W<DS1_SPEC> {
        GPIO_DS1_W::new(self, 0)
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
#[doc = "GPIO Drive Strength 1 Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ds1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ds1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DS1_SPEC;
impl crate::RegisterSpec for DS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ds1::R`](R) reader structure"]
impl crate::Readable for DS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ds1::W`](W) writer structure"]
impl crate::Writable for DS1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DS1 to value 0"]
impl crate::Resettable for DS1_SPEC {
    const RESET_VALUE: u32 = 0;
}
