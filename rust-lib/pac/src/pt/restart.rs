#[doc = "Register `RESTART` reader"]
pub type R = crate::R<RESTART_SPEC>;
#[doc = "Register `RESTART` writer"]
pub type W = crate::W<RESTART_SPEC>;
#[doc = "Field `pt_x_select` reader - Auto-Restart PT X Select"]
pub type PT_X_SELECT_R = crate::FieldReader;
#[doc = "Field `pt_x_select` writer - Auto-Restart PT X Select"]
pub type PT_X_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `on_pt_x_loop_exit` reader - Enable Auto-Restart on PT X Loop Exit"]
pub type ON_PT_X_LOOP_EXIT_R = crate::BitReader;
#[doc = "Field `on_pt_x_loop_exit` writer - Enable Auto-Restart on PT X Loop Exit"]
pub type ON_PT_X_LOOP_EXIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pt_y_select` reader - Auto-Restart PT Y Select"]
pub type PT_Y_SELECT_R = crate::FieldReader;
#[doc = "Field `pt_y_select` writer - Auto-Restart PT Y Select"]
pub type PT_Y_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `on_pt_y_loop_exit` reader - Enable Auto-Restart on PT Y Loop Exit"]
pub type ON_PT_Y_LOOP_EXIT_R = crate::BitReader;
#[doc = "Field `on_pt_y_loop_exit` writer - Enable Auto-Restart on PT Y Loop Exit"]
pub type ON_PT_Y_LOOP_EXIT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Auto-Restart PT X Select"]
    #[inline(always)]
    pub fn pt_x_select(&self) -> PT_X_SELECT_R {
        PT_X_SELECT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Enable Auto-Restart on PT X Loop Exit"]
    #[inline(always)]
    pub fn on_pt_x_loop_exit(&self) -> ON_PT_X_LOOP_EXIT_R {
        ON_PT_X_LOOP_EXIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Auto-Restart PT Y Select"]
    #[inline(always)]
    pub fn pt_y_select(&self) -> PT_Y_SELECT_R {
        PT_Y_SELECT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Enable Auto-Restart on PT Y Loop Exit"]
    #[inline(always)]
    pub fn on_pt_y_loop_exit(&self) -> ON_PT_Y_LOOP_EXIT_R {
        ON_PT_Y_LOOP_EXIT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Auto-Restart PT X Select"]
    #[inline(always)]
    #[must_use]
    pub fn pt_x_select(&mut self) -> PT_X_SELECT_W<RESTART_SPEC> {
        PT_X_SELECT_W::new(self, 0)
    }
    #[doc = "Bit 7 - Enable Auto-Restart on PT X Loop Exit"]
    #[inline(always)]
    #[must_use]
    pub fn on_pt_x_loop_exit(&mut self) -> ON_PT_X_LOOP_EXIT_W<RESTART_SPEC> {
        ON_PT_X_LOOP_EXIT_W::new(self, 7)
    }
    #[doc = "Bits 8:12 - Auto-Restart PT Y Select"]
    #[inline(always)]
    #[must_use]
    pub fn pt_y_select(&mut self) -> PT_Y_SELECT_W<RESTART_SPEC> {
        PT_Y_SELECT_W::new(self, 8)
    }
    #[doc = "Bit 15 - Enable Auto-Restart on PT Y Loop Exit"]
    #[inline(always)]
    #[must_use]
    pub fn on_pt_y_loop_exit(&mut self) -> ON_PT_Y_LOOP_EXIT_W<RESTART_SPEC> {
        ON_PT_Y_LOOP_EXIT_W::new(self, 15)
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
#[doc = "Pulse Train Auto-Restart Configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`restart::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`restart::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESTART_SPEC;
impl crate::RegisterSpec for RESTART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`restart::R`](R) reader structure"]
impl crate::Readable for RESTART_SPEC {}
#[doc = "`write(|w| ..)` method takes [`restart::W`](W) writer structure"]
impl crate::Writable for RESTART_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESTART to value 0"]
impl crate::Resettable for RESTART_SPEC {
    const RESET_VALUE: u32 = 0;
}
