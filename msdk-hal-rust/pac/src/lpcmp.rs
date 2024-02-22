#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: [CTRL; 3],
}
impl RegisterBlock {
    #[doc = "0x00..0x0c - Comparator Control Register."]
    #[inline(always)]
    pub const fn ctrl(&self, n: usize) -> &CTRL {
        &self.ctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x0c - Comparator Control Register."]
    #[inline(always)]
    pub fn ctrl_iter(&self) -> impl Iterator<Item = &CTRL> {
        self.ctrl.iter()
    }
}
#[doc = "CTRL (rw) register accessor: Comparator Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Comparator Control Register."]
pub mod ctrl;
