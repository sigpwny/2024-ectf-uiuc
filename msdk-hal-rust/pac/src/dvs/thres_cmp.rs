#[doc = "Register `THRES_CMP` reader"]
pub type R = crate::R<THRES_CMP_SPEC>;
#[doc = "Register `THRES_CMP` writer"]
pub type W = crate::W<THRES_CMP_SPEC>;
#[doc = "Field `VCNTR_THRES_CNT` reader - Value used to determine 'low voltage' range"]
pub type VCNTR_THRES_CNT_R = crate::FieldReader;
#[doc = "Field `VCNTR_THRES_CNT` writer - Value used to determine 'low voltage' range"]
pub type VCNTR_THRES_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `VCNTR_THRES_MASK` reader - Mask applied to threshold and vcount to determine if the device is in a low voltage range"]
pub type VCNTR_THRES_MASK_R = crate::FieldReader;
#[doc = "Field `VCNTR_THRES_MASK` writer - Mask applied to threshold and vcount to determine if the device is in a low voltage range"]
pub type VCNTR_THRES_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Value used to determine 'low voltage' range"]
    #[inline(always)]
    pub fn vcntr_thres_cnt(&self) -> VCNTR_THRES_CNT_R {
        VCNTR_THRES_CNT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Mask applied to threshold and vcount to determine if the device is in a low voltage range"]
    #[inline(always)]
    pub fn vcntr_thres_mask(&self) -> VCNTR_THRES_MASK_R {
        VCNTR_THRES_MASK_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Value used to determine 'low voltage' range"]
    #[inline(always)]
    #[must_use]
    pub fn vcntr_thres_cnt(&mut self) -> VCNTR_THRES_CNT_W<THRES_CMP_SPEC> {
        VCNTR_THRES_CNT_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Mask applied to threshold and vcount to determine if the device is in a low voltage range"]
    #[inline(always)]
    #[must_use]
    pub fn vcntr_thres_mask(&mut self) -> VCNTR_THRES_MASK_W<THRES_CMP_SPEC> {
        VCNTR_THRES_MASK_W::new(self, 8)
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
#[doc = "Up Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`thres_cmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thres_cmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THRES_CMP_SPEC;
impl crate::RegisterSpec for THRES_CMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thres_cmp::R`](R) reader structure"]
impl crate::Readable for THRES_CMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`thres_cmp::W`](W) writer structure"]
impl crate::Writable for THRES_CMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets THRES_CMP to value 0"]
impl crate::Resettable for THRES_CMP_SPEC {
    const RESET_VALUE: u32 = 0;
}
