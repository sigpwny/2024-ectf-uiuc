#[doc = "Register `GPIO3_CTRL` reader"]
pub type R = crate::R<GPIO3_CTRL_SPEC>;
#[doc = "Register `GPIO3_CTRL` writer"]
pub type W = crate::W<GPIO3_CTRL_SPEC>;
#[doc = "Field `P30_DO` reader - GPIO3 Pin 0 Data Output."]
pub type P30_DO_R = crate::BitReader;
#[doc = "Field `P30_DO` writer - GPIO3 Pin 0 Data Output."]
pub type P30_DO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30_OE` reader - GPIO3 Pin 0 Output Enable."]
pub type P30_OE_R = crate::BitReader;
#[doc = "Field `P30_OE` writer - GPIO3 Pin 0 Output Enable."]
pub type P30_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30_PE` reader - GPIO3 Pin 0 Pull-up Enable."]
pub type P30_PE_R = crate::BitReader;
#[doc = "Field `P30_PE` writer - GPIO3 Pin 0 Pull-up Enable."]
pub type P30_PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30_IN` reader - GPIO3 Pin 0 Input Status."]
pub type P30_IN_R = crate::BitReader;
#[doc = "Field `P30_IN` writer - GPIO3 Pin 0 Input Status."]
pub type P30_IN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31_DO` reader - GPIO3 Pin 1 Data Output."]
pub type P31_DO_R = crate::BitReader;
#[doc = "Field `P31_DO` writer - GPIO3 Pin 1 Data Output."]
pub type P31_DO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31_OE` reader - GPIO3 Pin 1 Output Enable."]
pub type P31_OE_R = crate::BitReader;
#[doc = "Field `P31_OE` writer - GPIO3 Pin 1 Output Enable."]
pub type P31_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31_PE` reader - GPIO3 Pin 1 Pull-up Enable."]
pub type P31_PE_R = crate::BitReader;
#[doc = "Field `P31_PE` writer - GPIO3 Pin 1 Pull-up Enable."]
pub type P31_PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31_IN` reader - GPIO3 Pin 1 Input Status."]
pub type P31_IN_R = crate::BitReader;
#[doc = "Field `P31_IN` writer - GPIO3 Pin 1 Input Status."]
pub type P31_IN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO3 Pin 0 Data Output."]
    #[inline(always)]
    pub fn p30_do(&self) -> P30_DO_R {
        P30_DO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO3 Pin 0 Output Enable."]
    #[inline(always)]
    pub fn p30_oe(&self) -> P30_OE_R {
        P30_OE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO3 Pin 0 Pull-up Enable."]
    #[inline(always)]
    pub fn p30_pe(&self) -> P30_PE_R {
        P30_PE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO3 Pin 0 Input Status."]
    #[inline(always)]
    pub fn p30_in(&self) -> P30_IN_R {
        P30_IN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO3 Pin 1 Data Output."]
    #[inline(always)]
    pub fn p31_do(&self) -> P31_DO_R {
        P31_DO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO3 Pin 1 Output Enable."]
    #[inline(always)]
    pub fn p31_oe(&self) -> P31_OE_R {
        P31_OE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO3 Pin 1 Pull-up Enable."]
    #[inline(always)]
    pub fn p31_pe(&self) -> P31_PE_R {
        P31_PE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO3 Pin 1 Input Status."]
    #[inline(always)]
    pub fn p31_in(&self) -> P31_IN_R {
        P31_IN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO3 Pin 0 Data Output."]
    #[inline(always)]
    #[must_use]
    pub fn p30_do(&mut self) -> P30_DO_W<GPIO3_CTRL_SPEC> {
        P30_DO_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO3 Pin 0 Output Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p30_oe(&mut self) -> P30_OE_W<GPIO3_CTRL_SPEC> {
        P30_OE_W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO3 Pin 0 Pull-up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p30_pe(&mut self) -> P30_PE_W<GPIO3_CTRL_SPEC> {
        P30_PE_W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO3 Pin 0 Input Status."]
    #[inline(always)]
    #[must_use]
    pub fn p30_in(&mut self) -> P30_IN_W<GPIO3_CTRL_SPEC> {
        P30_IN_W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO3 Pin 1 Data Output."]
    #[inline(always)]
    #[must_use]
    pub fn p31_do(&mut self) -> P31_DO_W<GPIO3_CTRL_SPEC> {
        P31_DO_W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO3 Pin 1 Output Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p31_oe(&mut self) -> P31_OE_W<GPIO3_CTRL_SPEC> {
        P31_OE_W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO3 Pin 1 Pull-up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p31_pe(&mut self) -> P31_PE_W<GPIO3_CTRL_SPEC> {
        P31_PE_W::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO3 Pin 1 Input Status."]
    #[inline(always)]
    #[must_use]
    pub fn p31_in(&mut self) -> P31_IN_W<GPIO3_CTRL_SPEC> {
        P31_IN_W::new(self, 7)
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
#[doc = "GPIO3 Pin Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO3_CTRL_SPEC;
impl crate::RegisterSpec for GPIO3_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio3_ctrl::R`](R) reader structure"]
impl crate::Readable for GPIO3_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio3_ctrl::W`](W) writer structure"]
impl crate::Writable for GPIO3_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO3_CTRL to value 0"]
impl crate::Resettable for GPIO3_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
