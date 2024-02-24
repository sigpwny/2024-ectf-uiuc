#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    sysctrl: SYSCTRL,
    rst0: RST0,
    clkctrl: CLKCTRL,
    pm: PM,
    _reserved4: [u8; 0x08],
    pclkdiv: PCLKDIV,
    _reserved5: [u8; 0x08],
    pclkdis0: PCLKDIS0,
    memctrl: MEMCTRL,
    memz: MEMZ,
    _reserved8: [u8; 0x10],
    sysst: SYSST,
    rst1: RST1,
    pclkdis1: PCLKDIS1,
    eventen: EVENTEN,
    revision: REVISION,
    sysie: SYSIE,
    _reserved14: [u8; 0x0c],
    eccerr: ECCERR,
    eccced: ECCCED,
    eccie: ECCIE,
    eccaddr: ECCADDR,
    _reserved18: [u8; 0x0c],
    gpr: GPR,
}
impl RegisterBlock {
    #[doc = "0x00 - System Control."]
    #[inline(always)]
    pub const fn sysctrl(&self) -> &SYSCTRL {
        &self.sysctrl
    }
    #[doc = "0x04 - Reset."]
    #[inline(always)]
    pub const fn rst0(&self) -> &RST0 {
        &self.rst0
    }
    #[doc = "0x08 - Clock Control."]
    #[inline(always)]
    pub const fn clkctrl(&self) -> &CLKCTRL {
        &self.clkctrl
    }
    #[doc = "0x0c - Power Management."]
    #[inline(always)]
    pub const fn pm(&self) -> &PM {
        &self.pm
    }
    #[doc = "0x18 - Peripheral Clock Divider."]
    #[inline(always)]
    pub const fn pclkdiv(&self) -> &PCLKDIV {
        &self.pclkdiv
    }
    #[doc = "0x24 - Peripheral Clock Disable."]
    #[inline(always)]
    pub const fn pclkdis0(&self) -> &PCLKDIS0 {
        &self.pclkdis0
    }
    #[doc = "0x28 - Memory Clock Control Register."]
    #[inline(always)]
    pub const fn memctrl(&self) -> &MEMCTRL {
        &self.memctrl
    }
    #[doc = "0x2c - Memory Zeroize Control."]
    #[inline(always)]
    pub const fn memz(&self) -> &MEMZ {
        &self.memz
    }
    #[doc = "0x40 - System Status Register."]
    #[inline(always)]
    pub const fn sysst(&self) -> &SYSST {
        &self.sysst
    }
    #[doc = "0x44 - Reset 1."]
    #[inline(always)]
    pub const fn rst1(&self) -> &RST1 {
        &self.rst1
    }
    #[doc = "0x48 - Peripheral Clock Disable."]
    #[inline(always)]
    pub const fn pclkdis1(&self) -> &PCLKDIS1 {
        &self.pclkdis1
    }
    #[doc = "0x4c - Event Enable Register."]
    #[inline(always)]
    pub const fn eventen(&self) -> &EVENTEN {
        &self.eventen
    }
    #[doc = "0x50 - Revision Register."]
    #[inline(always)]
    pub const fn revision(&self) -> &REVISION {
        &self.revision
    }
    #[doc = "0x54 - System Status Interrupt Enable Register."]
    #[inline(always)]
    pub const fn sysie(&self) -> &SYSIE {
        &self.sysie
    }
    #[doc = "0x64 - ECC Error Register"]
    #[inline(always)]
    pub const fn eccerr(&self) -> &ECCERR {
        &self.eccerr
    }
    #[doc = "0x68 - ECC Not Double Error Detect Register"]
    #[inline(always)]
    pub const fn eccced(&self) -> &ECCCED {
        &self.eccced
    }
    #[doc = "0x6c - ECC IRQ Enable Register"]
    #[inline(always)]
    pub const fn eccie(&self) -> &ECCIE {
        &self.eccie
    }
    #[doc = "0x70 - ECC Error Address Register"]
    #[inline(always)]
    pub const fn eccaddr(&self) -> &ECCADDR {
        &self.eccaddr
    }
    #[doc = "0x80 - General Purpose Register."]
    #[inline(always)]
    pub const fn gpr(&self) -> &GPR {
        &self.gpr
    }
}
#[doc = "SYSCTRL (rw) register accessor: System Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctrl`]
module"]
pub type SYSCTRL = crate::Reg<sysctrl::SYSCTRL_SPEC>;
#[doc = "System Control."]
pub mod sysctrl;
#[doc = "RST0 (rw) register accessor: Reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst0`]
module"]
pub type RST0 = crate::Reg<rst0::RST0_SPEC>;
#[doc = "Reset."]
pub mod rst0;
#[doc = "CLKCTRL (rw) register accessor: Clock Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkctrl`]
module"]
pub type CLKCTRL = crate::Reg<clkctrl::CLKCTRL_SPEC>;
#[doc = "Clock Control."]
pub mod clkctrl;
#[doc = "PM (rw) register accessor: Power Management.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pm`]
module"]
pub type PM = crate::Reg<pm::PM_SPEC>;
#[doc = "Power Management."]
pub mod pm;
#[doc = "PCLKDIV (rw) register accessor: Peripheral Clock Divider.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pclkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pclkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pclkdiv`]
module"]
pub type PCLKDIV = crate::Reg<pclkdiv::PCLKDIV_SPEC>;
#[doc = "Peripheral Clock Divider."]
pub mod pclkdiv;
#[doc = "PCLKDIS0 (rw) register accessor: Peripheral Clock Disable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pclkdis0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pclkdis0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pclkdis0`]
module"]
pub type PCLKDIS0 = crate::Reg<pclkdis0::PCLKDIS0_SPEC>;
#[doc = "Peripheral Clock Disable."]
pub mod pclkdis0;
#[doc = "MEMCTRL (rw) register accessor: Memory Clock Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memctrl`]
module"]
pub type MEMCTRL = crate::Reg<memctrl::MEMCTRL_SPEC>;
#[doc = "Memory Clock Control Register."]
pub mod memctrl;
#[doc = "MEMZ (rw) register accessor: Memory Zeroize Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memz`]
module"]
pub type MEMZ = crate::Reg<memz::MEMZ_SPEC>;
#[doc = "Memory Zeroize Control."]
pub mod memz;
#[doc = "SYSST (rw) register accessor: System Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysst`]
module"]
pub type SYSST = crate::Reg<sysst::SYSST_SPEC>;
#[doc = "System Status Register."]
pub mod sysst;
#[doc = "RST1 (rw) register accessor: Reset 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst1`]
module"]
pub type RST1 = crate::Reg<rst1::RST1_SPEC>;
#[doc = "Reset 1."]
pub mod rst1;
#[doc = "PCLKDIS1 (rw) register accessor: Peripheral Clock Disable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pclkdis1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pclkdis1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pclkdis1`]
module"]
pub type PCLKDIS1 = crate::Reg<pclkdis1::PCLKDIS1_SPEC>;
#[doc = "Peripheral Clock Disable."]
pub mod pclkdis1;
#[doc = "EVENTEN (rw) register accessor: Event Enable Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eventen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eventen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eventen`]
module"]
pub type EVENTEN = crate::Reg<eventen::EVENTEN_SPEC>;
#[doc = "Event Enable Register."]
pub mod eventen;
#[doc = "REVISION (r) register accessor: Revision Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revision::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revision`]
module"]
pub type REVISION = crate::Reg<revision::REVISION_SPEC>;
#[doc = "Revision Register."]
pub mod revision;
#[doc = "SYSIE (rw) register accessor: System Status Interrupt Enable Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysie`]
module"]
pub type SYSIE = crate::Reg<sysie::SYSIE_SPEC>;
#[doc = "System Status Interrupt Enable Register."]
pub mod sysie;
#[doc = "ECCERR (rw) register accessor: ECC Error Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccerr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccerr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccerr`]
module"]
pub type ECCERR = crate::Reg<eccerr::ECCERR_SPEC>;
#[doc = "ECC Error Register"]
pub mod eccerr;
#[doc = "ECCCED (rw) register accessor: ECC Not Double Error Detect Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccced::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccced::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccced`]
module"]
pub type ECCCED = crate::Reg<eccced::ECCCED_SPEC>;
#[doc = "ECC Not Double Error Detect Register"]
pub mod eccced;
#[doc = "ECCIE (rw) register accessor: ECC IRQ Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccie`]
module"]
pub type ECCIE = crate::Reg<eccie::ECCIE_SPEC>;
#[doc = "ECC IRQ Enable Register"]
pub mod eccie;
#[doc = "ECCADDR (rw) register accessor: ECC Error Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccaddr`]
module"]
pub type ECCADDR = crate::Reg<eccaddr::ECCADDR_SPEC>;
#[doc = "ECC Error Address Register"]
pub mod eccaddr;
#[doc = "GPR (rw) register accessor: General Purpose Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpr`]
module"]
pub type GPR = crate::Reg<gpr::GPR_SPEC>;
#[doc = "General Purpose Register."]
pub mod gpr;
