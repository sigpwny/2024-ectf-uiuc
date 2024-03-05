#[doc = "Register `ADJ_DWN` reader"]
pub type R = crate::R<ADJ_DWN_SPEC>;
#[doc = "Register `ADJ_DWN` writer"]
pub type W = crate::W<ADJ_DWN_SPEC>;
#[doc = "Field `DLY` reader - Number of prescaled clocks between updates of the adjustment delay counter"]
pub type DLY_R = crate::FieldReader<u16>;
#[doc = "Field `DLY` writer - Number of prescaled clocks between updates of the adjustment delay counter"]
pub type DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PRE` reader - Number of clocks before DVS_ADJ_DWN_DLY is decremented"]
pub type PRE_R = crate::FieldReader;
#[doc = "Field `PRE` writer - Number of clocks before DVS_ADJ_DWN_DLY is decremented"]
pub type PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Number of prescaled clocks between updates of the adjustment delay counter"]
    #[inline(always)]
    pub fn dly(&self) -> DLY_R {
        DLY_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Number of clocks before DVS_ADJ_DWN_DLY is decremented"]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of prescaled clocks between updates of the adjustment delay counter"]
    #[inline(always)]
    #[must_use]
    pub fn dly(&mut self) -> DLY_W<ADJ_DWN_SPEC> {
        DLY_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Number of clocks before DVS_ADJ_DWN_DLY is decremented"]
    #[inline(always)]
    #[must_use]
    pub fn pre(&mut self) -> PRE_W<ADJ_DWN_SPEC> {
        PRE_W::new(self, 16)
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
#[doc = "Down Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adj_dwn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adj_dwn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADJ_DWN_SPEC;
impl crate::RegisterSpec for ADJ_DWN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adj_dwn::R`](R) reader structure"]
impl crate::Readable for ADJ_DWN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adj_dwn::W`](W) writer structure"]
impl crate::Writable for ADJ_DWN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADJ_DWN to value 0"]
impl crate::Resettable for ADJ_DWN_SPEC {
    const RESET_VALUE: u32 = 0;
}
