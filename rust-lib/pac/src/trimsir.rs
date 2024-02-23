#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    rtc: RTC,
    _reserved1: [u8; 0x28],
    simo: SIMO,
    _reserved2: [u8; 0x04],
    ipolo: IPOLO,
    ctrl: CTRL,
    inro: INRO,
}
impl RegisterBlock {
    #[doc = "0x08 - RTC Trim System Initialization Register."]
    #[inline(always)]
    pub const fn rtc(&self) -> &RTC {
        &self.rtc
    }
    #[doc = "0x34 - SIMO Trim System Initialization Register."]
    #[inline(always)]
    pub const fn simo(&self) -> &SIMO {
        &self.simo
    }
    #[doc = "0x3c - IPO Low Trim System Initialization Register."]
    #[inline(always)]
    pub const fn ipolo(&self) -> &IPOLO {
        &self.ipolo
    }
    #[doc = "0x40 - Control Trim System Initialization Register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x44 - RTC Trim System Initialization Register."]
    #[inline(always)]
    pub const fn inro(&self) -> &INRO {
        &self.inro
    }
}
#[doc = "RTC (rw) register accessor: RTC Trim System Initialization Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc`]
module"]
pub type RTC = crate::Reg<rtc::RTC_SPEC>;
#[doc = "RTC Trim System Initialization Register."]
pub mod rtc;
#[doc = "SIMO (r) register accessor: SIMO Trim System Initialization Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`simo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simo`]
module"]
pub type SIMO = crate::Reg<simo::SIMO_SPEC>;
#[doc = "SIMO Trim System Initialization Register."]
pub mod simo;
#[doc = "IPOLO (r) register accessor: IPO Low Trim System Initialization Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipolo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipolo`]
module"]
pub type IPOLO = crate::Reg<ipolo::IPOLO_SPEC>;
#[doc = "IPO Low Trim System Initialization Register."]
pub mod ipolo;
#[doc = "CTRL (rw) register accessor: Control Trim System Initialization Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Trim System Initialization Register."]
pub mod ctrl;
#[doc = "INRO (rw) register accessor: RTC Trim System Initialization Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inro::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inro::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inro`]
module"]
pub type INRO = crate::Reg<inro::INRO_SPEC>;
#[doc = "RTC Trim System Initialization Register."]
pub mod inro;
