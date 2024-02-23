#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    eccen: ECCEN,
    ipo_mtrim: IPO_MTRIM,
    outen: OUTEN,
    cmp_ctrl: CMP_CTRL,
    ctrl: CTRL,
    _reserved5: [u8; 0x0c],
    gpio3_ctrl: GPIO3_CTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - ECC Enable Register"]
    #[inline(always)]
    pub const fn eccen(&self) -> &ECCEN {
        &self.eccen
    }
    #[doc = "0x04 - IPO Manual Register"]
    #[inline(always)]
    pub const fn ipo_mtrim(&self) -> &IPO_MTRIM {
        &self.ipo_mtrim
    }
    #[doc = "0x08 - Output Enable Register"]
    #[inline(always)]
    pub const fn outen(&self) -> &OUTEN {
        &self.outen
    }
    #[doc = "0x0c - Comparator Control Register."]
    #[inline(always)]
    pub const fn cmp_ctrl(&self) -> &CMP_CTRL {
        &self.cmp_ctrl
    }
    #[doc = "0x10 - Miscellaneous Control Register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x20 - GPIO3 Pin Control Register."]
    #[inline(always)]
    pub const fn gpio3_ctrl(&self) -> &GPIO3_CTRL {
        &self.gpio3_ctrl
    }
}
#[doc = "ECCEN (rw) register accessor: ECC Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccen`]
module"]
pub type ECCEN = crate::Reg<eccen::ECCEN_SPEC>;
#[doc = "ECC Enable Register"]
pub mod eccen;
#[doc = "IPO_MTRIM (rw) register accessor: IPO Manual Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipo_mtrim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipo_mtrim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipo_mtrim`]
module"]
pub type IPO_MTRIM = crate::Reg<ipo_mtrim::IPO_MTRIM_SPEC>;
#[doc = "IPO Manual Register"]
pub mod ipo_mtrim;
#[doc = "OUTEN (rw) register accessor: Output Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outen`]
module"]
pub type OUTEN = crate::Reg<outen::OUTEN_SPEC>;
#[doc = "Output Enable Register"]
pub mod outen;
#[doc = "CMP_CTRL (rw) register accessor: Comparator Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp_ctrl`]
module"]
pub type CMP_CTRL = crate::Reg<cmp_ctrl::CMP_CTRL_SPEC>;
#[doc = "Comparator Control Register."]
pub mod cmp_ctrl;
#[doc = "CTRL (rw) register accessor: Miscellaneous Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Miscellaneous Control Register."]
pub mod ctrl;
#[doc = "GPIO3_CTRL (rw) register accessor: GPIO3 Pin Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3_ctrl`]
module"]
pub type GPIO3_CTRL = crate::Reg<gpio3_ctrl::GPIO3_CTRL_SPEC>;
#[doc = "GPIO3 Pin Control Register."]
pub mod gpio3_ctrl;
