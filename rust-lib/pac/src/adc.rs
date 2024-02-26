#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    status: STATUS,
    data: DATA,
    intr: INTR,
    limit0: LIMIT0,
    limit1: LIMIT1,
    limit2: LIMIT2,
    limit3: LIMIT3,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC Control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - ADC Status"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x08 - ADC Output Data"]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
    #[doc = "0x0c - ADC Interrupt Control Register"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0x10 - ADC Limit"]
    #[inline(always)]
    pub const fn limit0(&self) -> &LIMIT0 {
        &self.limit0
    }
    #[doc = "0x14 - ADC Limit"]
    #[inline(always)]
    pub const fn limit1(&self) -> &LIMIT1 {
        &self.limit1
    }
    #[doc = "0x18 - ADC Limit"]
    #[inline(always)]
    pub const fn limit2(&self) -> &LIMIT2 {
        &self.limit2
    }
    #[doc = "0x1c - ADC Limit"]
    #[inline(always)]
    pub const fn limit3(&self) -> &LIMIT3 {
        &self.limit3
    }
}
#[doc = "CTRL (rw) register accessor: ADC Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "ADC Control"]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: ADC Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "ADC Status"]
pub mod status;
#[doc = "DATA (rw) register accessor: ADC Output Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "ADC Output Data"]
pub mod data;
#[doc = "INTR (rw) register accessor: ADC Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "ADC Interrupt Control Register"]
pub mod intr;
#[doc = "LIMIT0 (rw) register accessor: ADC Limit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`limit0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`limit0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@limit0`]
module"]
pub type LIMIT0 = crate::Reg<limit0::LIMIT0_SPEC>;
#[doc = "ADC Limit"]
pub mod limit0;
pub use limit0 as limit1;
pub use limit0 as limit2;
pub use limit0 as limit3;
pub use LIMIT0 as LIMIT1;
pub use LIMIT0 as LIMIT2;
pub use LIMIT0 as LIMIT3;
