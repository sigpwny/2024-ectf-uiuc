#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    fctrl0: FCTRL0,
    autocal0: AUTOCAL0,
    autocal1: AUTOCAL1,
    autocal2: AUTOCAL2,
    urvbootaddr: URVBOOTADDR,
    urvctrl: URVCTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - Function Control 0."]
    #[inline(always)]
    pub const fn fctrl0(&self) -> &FCTRL0 {
        &self.fctrl0
    }
    #[doc = "0x04 - Automatic Calibration 0."]
    #[inline(always)]
    pub const fn autocal0(&self) -> &AUTOCAL0 {
        &self.autocal0
    }
    #[doc = "0x08 - Automatic Calibration 1."]
    #[inline(always)]
    pub const fn autocal1(&self) -> &AUTOCAL1 {
        &self.autocal1
    }
    #[doc = "0x0c - Automatic Calibration 2"]
    #[inline(always)]
    pub const fn autocal2(&self) -> &AUTOCAL2 {
        &self.autocal2
    }
    #[doc = "0x10 - RISC-V Boot Address."]
    #[inline(always)]
    pub const fn urvbootaddr(&self) -> &URVBOOTADDR {
        &self.urvbootaddr
    }
    #[doc = "0x14 - RISC-V Control Register."]
    #[inline(always)]
    pub const fn urvctrl(&self) -> &URVCTRL {
        &self.urvctrl
    }
}
#[doc = "FCTRL0 (rw) register accessor: Function Control 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrl0`]
module"]
pub type FCTRL0 = crate::Reg<fctrl0::FCTRL0_SPEC>;
#[doc = "Function Control 0."]
pub mod fctrl0;
#[doc = "AUTOCAL0 (rw) register accessor: Automatic Calibration 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`autocal0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`autocal0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autocal0`]
module"]
pub type AUTOCAL0 = crate::Reg<autocal0::AUTOCAL0_SPEC>;
#[doc = "Automatic Calibration 0."]
pub mod autocal0;
#[doc = "AUTOCAL1 (rw) register accessor: Automatic Calibration 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`autocal1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`autocal1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autocal1`]
module"]
pub type AUTOCAL1 = crate::Reg<autocal1::AUTOCAL1_SPEC>;
#[doc = "Automatic Calibration 1."]
pub mod autocal1;
#[doc = "AUTOCAL2 (rw) register accessor: Automatic Calibration 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`autocal2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`autocal2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autocal2`]
module"]
pub type AUTOCAL2 = crate::Reg<autocal2::AUTOCAL2_SPEC>;
#[doc = "Automatic Calibration 2"]
pub mod autocal2;
#[doc = "URVBOOTADDR (rw) register accessor: RISC-V Boot Address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`urvbootaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`urvbootaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@urvbootaddr`]
module"]
pub type URVBOOTADDR = crate::Reg<urvbootaddr::URVBOOTADDR_SPEC>;
#[doc = "RISC-V Boot Address."]
pub mod urvbootaddr;
#[doc = "URVCTRL (rw) register accessor: RISC-V Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`urvctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`urvctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@urvctrl`]
module"]
pub type URVCTRL = crate::Reg<urvctrl::URVCTRL_SPEC>;
#[doc = "RISC-V Control Register."]
pub mod urvctrl;
