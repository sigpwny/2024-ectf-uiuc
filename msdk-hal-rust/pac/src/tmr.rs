#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cnt: CNT,
    cmp: CMP,
    pwm: PWM,
    intfl: INTFL,
    ctrl0: CTRL0,
    nolcmp: NOLCMP,
    ctrl1: CTRL1,
    wkfl: WKFL,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer Counter Register."]
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    #[doc = "0x04 - Timer Compare Register."]
    #[inline(always)]
    pub const fn cmp(&self) -> &CMP {
        &self.cmp
    }
    #[doc = "0x08 - Timer PWM Register."]
    #[inline(always)]
    pub const fn pwm(&self) -> &PWM {
        &self.pwm
    }
    #[doc = "0x0c - Timer Interrupt Status Register."]
    #[inline(always)]
    pub const fn intfl(&self) -> &INTFL {
        &self.intfl
    }
    #[doc = "0x10 - Timer Control Register."]
    #[inline(always)]
    pub const fn ctrl0(&self) -> &CTRL0 {
        &self.ctrl0
    }
    #[doc = "0x14 - Timer Non-Overlapping Compare Register."]
    #[inline(always)]
    pub const fn nolcmp(&self) -> &NOLCMP {
        &self.nolcmp
    }
    #[doc = "0x18 - Timer Configuration Register."]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
    #[doc = "0x1c - Timer Wakeup Status Register."]
    #[inline(always)]
    pub const fn wkfl(&self) -> &WKFL {
        &self.wkfl
    }
}
#[doc = "CNT (rw) register accessor: Timer Counter Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Timer Counter Register."]
pub mod cnt;
#[doc = "CMP (rw) register accessor: Timer Compare Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp`]
module"]
pub type CMP = crate::Reg<cmp::CMP_SPEC>;
#[doc = "Timer Compare Register."]
pub mod cmp;
#[doc = "PWM (rw) register accessor: Timer PWM Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm`]
module"]
pub type PWM = crate::Reg<pwm::PWM_SPEC>;
#[doc = "Timer PWM Register."]
pub mod pwm;
#[doc = "INTFL (rw) register accessor: Timer Interrupt Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intfl`]
module"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "Timer Interrupt Status Register."]
pub mod intfl;
#[doc = "CTRL0 (rw) register accessor: Timer Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl0`]
module"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "Timer Control Register."]
pub mod ctrl0;
#[doc = "NOLCMP (rw) register accessor: Timer Non-Overlapping Compare Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nolcmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nolcmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nolcmp`]
module"]
pub type NOLCMP = crate::Reg<nolcmp::NOLCMP_SPEC>;
#[doc = "Timer Non-Overlapping Compare Register."]
pub mod nolcmp;
#[doc = "CTRL1 (rw) register accessor: Timer Configuration Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`]
module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Timer Configuration Register."]
pub mod ctrl1;
#[doc = "WKFL (rw) register accessor: Timer Wakeup Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wkfl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wkfl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkfl`]
module"]
pub type WKFL = crate::Reg<wkfl::WKFL_SPEC>;
#[doc = "Timer Wakeup Status Register."]
pub mod wkfl;
