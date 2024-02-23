#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    vrego_a: VREGO_A,
    vrego_b: VREGO_B,
    vrego_c: VREGO_C,
    vrego_d: VREGO_D,
    ipka: IPKA,
    ipkb: IPKB,
    maxton: MAXTON,
    iload_a: ILOAD_A,
    iload_b: ILOAD_B,
    iload_c: ILOAD_C,
    iload_d: ILOAD_D,
    buck_alert_thr_a: BUCK_ALERT_THR_A,
    buck_alert_thr_b: BUCK_ALERT_THR_B,
    buck_alert_thr_c: BUCK_ALERT_THR_C,
    buck_alert_thr_d: BUCK_ALERT_THR_D,
    buck_out_ready: BUCK_OUT_READY,
    zero_cross_cal_a: ZERO_CROSS_CAL_A,
    zero_cross_cal_b: ZERO_CROSS_CAL_B,
    zero_cross_cal_c: ZERO_CROSS_CAL_C,
    zero_cross_cal_d: ZERO_CROSS_CAL_D,
}
impl RegisterBlock {
    #[doc = "0x04 - Buck Voltage Regulator A Control Register"]
    #[inline(always)]
    pub const fn vrego_a(&self) -> &VREGO_A {
        &self.vrego_a
    }
    #[doc = "0x08 - Buck Voltage Regulator B Control Register"]
    #[inline(always)]
    pub const fn vrego_b(&self) -> &VREGO_B {
        &self.vrego_b
    }
    #[doc = "0x0c - Buck Voltage Regulator C Control Register"]
    #[inline(always)]
    pub const fn vrego_c(&self) -> &VREGO_C {
        &self.vrego_c
    }
    #[doc = "0x10 - Buck Voltage Regulator D Control Register"]
    #[inline(always)]
    pub const fn vrego_d(&self) -> &VREGO_D {
        &self.vrego_d
    }
    #[doc = "0x14 - High Side FET Peak Current VREGO_A/VREGO_B Register"]
    #[inline(always)]
    pub const fn ipka(&self) -> &IPKA {
        &self.ipka
    }
    #[doc = "0x18 - High Side FET Peak Current VREGO_C/VREGO_D Register"]
    #[inline(always)]
    pub const fn ipkb(&self) -> &IPKB {
        &self.ipkb
    }
    #[doc = "0x1c - Maximum High Side FET Time On Register"]
    #[inline(always)]
    pub const fn maxton(&self) -> &MAXTON {
        &self.maxton
    }
    #[doc = "0x20 - Buck Cycle Count VREGO_A Register"]
    #[inline(always)]
    pub const fn iload_a(&self) -> &ILOAD_A {
        &self.iload_a
    }
    #[doc = "0x24 - Buck Cycle Count VREGO_B Register"]
    #[inline(always)]
    pub const fn iload_b(&self) -> &ILOAD_B {
        &self.iload_b
    }
    #[doc = "0x28 - Buck Cycle Count VREGO_C Register"]
    #[inline(always)]
    pub const fn iload_c(&self) -> &ILOAD_C {
        &self.iload_c
    }
    #[doc = "0x2c - Buck Cycle Count VREGO_D Register"]
    #[inline(always)]
    pub const fn iload_d(&self) -> &ILOAD_D {
        &self.iload_d
    }
    #[doc = "0x30 - Buck Cycle Count Alert VERGO_A Register"]
    #[inline(always)]
    pub const fn buck_alert_thr_a(&self) -> &BUCK_ALERT_THR_A {
        &self.buck_alert_thr_a
    }
    #[doc = "0x34 - Buck Cycle Count Alert VERGO_B Register"]
    #[inline(always)]
    pub const fn buck_alert_thr_b(&self) -> &BUCK_ALERT_THR_B {
        &self.buck_alert_thr_b
    }
    #[doc = "0x38 - Buck Cycle Count Alert VERGO_C Register"]
    #[inline(always)]
    pub const fn buck_alert_thr_c(&self) -> &BUCK_ALERT_THR_C {
        &self.buck_alert_thr_c
    }
    #[doc = "0x3c - Buck Cycle Count Alert VERGO_D Register"]
    #[inline(always)]
    pub const fn buck_alert_thr_d(&self) -> &BUCK_ALERT_THR_D {
        &self.buck_alert_thr_d
    }
    #[doc = "0x40 - Buck Regulator Output Ready Register"]
    #[inline(always)]
    pub const fn buck_out_ready(&self) -> &BUCK_OUT_READY {
        &self.buck_out_ready
    }
    #[doc = "0x44 - Zero Cross Calibration VERGO_A Register"]
    #[inline(always)]
    pub const fn zero_cross_cal_a(&self) -> &ZERO_CROSS_CAL_A {
        &self.zero_cross_cal_a
    }
    #[doc = "0x48 - Zero Cross Calibration VERGO_B Register"]
    #[inline(always)]
    pub const fn zero_cross_cal_b(&self) -> &ZERO_CROSS_CAL_B {
        &self.zero_cross_cal_b
    }
    #[doc = "0x4c - Zero Cross Calibration VERGO_C Register"]
    #[inline(always)]
    pub const fn zero_cross_cal_c(&self) -> &ZERO_CROSS_CAL_C {
        &self.zero_cross_cal_c
    }
    #[doc = "0x50 - Zero Cross Calibration VERGO_D Register"]
    #[inline(always)]
    pub const fn zero_cross_cal_d(&self) -> &ZERO_CROSS_CAL_D {
        &self.zero_cross_cal_d
    }
}
#[doc = "VREGO_A (rw) register accessor: Buck Voltage Regulator A Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vrego_a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vrego_a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vrego_a`]
module"]
pub type VREGO_A = crate::Reg<vrego_a::VREGO_A_SPEC>;
#[doc = "Buck Voltage Regulator A Control Register"]
pub mod vrego_a;
#[doc = "VREGO_B (rw) register accessor: Buck Voltage Regulator B Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vrego_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vrego_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vrego_b`]
module"]
pub type VREGO_B = crate::Reg<vrego_b::VREGO_B_SPEC>;
#[doc = "Buck Voltage Regulator B Control Register"]
pub mod vrego_b;
#[doc = "VREGO_C (rw) register accessor: Buck Voltage Regulator C Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vrego_c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vrego_c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vrego_c`]
module"]
pub type VREGO_C = crate::Reg<vrego_c::VREGO_C_SPEC>;
#[doc = "Buck Voltage Regulator C Control Register"]
pub mod vrego_c;
#[doc = "VREGO_D (rw) register accessor: Buck Voltage Regulator D Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vrego_d::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vrego_d::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vrego_d`]
module"]
pub type VREGO_D = crate::Reg<vrego_d::VREGO_D_SPEC>;
#[doc = "Buck Voltage Regulator D Control Register"]
pub mod vrego_d;
#[doc = "IPKA (rw) register accessor: High Side FET Peak Current VREGO_A/VREGO_B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipka::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipka::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipka`]
module"]
pub type IPKA = crate::Reg<ipka::IPKA_SPEC>;
#[doc = "High Side FET Peak Current VREGO_A/VREGO_B Register"]
pub mod ipka;
#[doc = "IPKB (rw) register accessor: High Side FET Peak Current VREGO_C/VREGO_D Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipkb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipkb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipkb`]
module"]
pub type IPKB = crate::Reg<ipkb::IPKB_SPEC>;
#[doc = "High Side FET Peak Current VREGO_C/VREGO_D Register"]
pub mod ipkb;
#[doc = "MAXTON (rw) register accessor: Maximum High Side FET Time On Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxton::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maxton::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxton`]
module"]
pub type MAXTON = crate::Reg<maxton::MAXTON_SPEC>;
#[doc = "Maximum High Side FET Time On Register"]
pub mod maxton;
#[doc = "ILOAD_A (r) register accessor: Buck Cycle Count VREGO_A Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iload_a::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iload_a`]
module"]
pub type ILOAD_A = crate::Reg<iload_a::ILOAD_A_SPEC>;
#[doc = "Buck Cycle Count VREGO_A Register"]
pub mod iload_a;
#[doc = "ILOAD_B (r) register accessor: Buck Cycle Count VREGO_B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iload_b::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iload_b`]
module"]
pub type ILOAD_B = crate::Reg<iload_b::ILOAD_B_SPEC>;
#[doc = "Buck Cycle Count VREGO_B Register"]
pub mod iload_b;
#[doc = "ILOAD_C (r) register accessor: Buck Cycle Count VREGO_C Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iload_c::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iload_c`]
module"]
pub type ILOAD_C = crate::Reg<iload_c::ILOAD_C_SPEC>;
#[doc = "Buck Cycle Count VREGO_C Register"]
pub mod iload_c;
#[doc = "ILOAD_D (r) register accessor: Buck Cycle Count VREGO_D Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iload_d::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iload_d`]
module"]
pub type ILOAD_D = crate::Reg<iload_d::ILOAD_D_SPEC>;
#[doc = "Buck Cycle Count VREGO_D Register"]
pub mod iload_d;
#[doc = "BUCK_ALERT_THR_A (rw) register accessor: Buck Cycle Count Alert VERGO_A Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buck_alert_thr_a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buck_alert_thr_a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buck_alert_thr_a`]
module"]
pub type BUCK_ALERT_THR_A = crate::Reg<buck_alert_thr_a::BUCK_ALERT_THR_A_SPEC>;
#[doc = "Buck Cycle Count Alert VERGO_A Register"]
pub mod buck_alert_thr_a;
#[doc = "BUCK_ALERT_THR_B (rw) register accessor: Buck Cycle Count Alert VERGO_B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buck_alert_thr_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buck_alert_thr_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buck_alert_thr_b`]
module"]
pub type BUCK_ALERT_THR_B = crate::Reg<buck_alert_thr_b::BUCK_ALERT_THR_B_SPEC>;
#[doc = "Buck Cycle Count Alert VERGO_B Register"]
pub mod buck_alert_thr_b;
#[doc = "BUCK_ALERT_THR_C (rw) register accessor: Buck Cycle Count Alert VERGO_C Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buck_alert_thr_c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buck_alert_thr_c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buck_alert_thr_c`]
module"]
pub type BUCK_ALERT_THR_C = crate::Reg<buck_alert_thr_c::BUCK_ALERT_THR_C_SPEC>;
#[doc = "Buck Cycle Count Alert VERGO_C Register"]
pub mod buck_alert_thr_c;
#[doc = "BUCK_ALERT_THR_D (rw) register accessor: Buck Cycle Count Alert VERGO_D Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buck_alert_thr_d::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buck_alert_thr_d::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buck_alert_thr_d`]
module"]
pub type BUCK_ALERT_THR_D = crate::Reg<buck_alert_thr_d::BUCK_ALERT_THR_D_SPEC>;
#[doc = "Buck Cycle Count Alert VERGO_D Register"]
pub mod buck_alert_thr_d;
#[doc = "BUCK_OUT_READY (r) register accessor: Buck Regulator Output Ready Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buck_out_ready::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buck_out_ready`]
module"]
pub type BUCK_OUT_READY = crate::Reg<buck_out_ready::BUCK_OUT_READY_SPEC>;
#[doc = "Buck Regulator Output Ready Register"]
pub mod buck_out_ready;
#[doc = "ZERO_CROSS_CAL_A (r) register accessor: Zero Cross Calibration VERGO_A Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`zero_cross_cal_a::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zero_cross_cal_a`]
module"]
pub type ZERO_CROSS_CAL_A = crate::Reg<zero_cross_cal_a::ZERO_CROSS_CAL_A_SPEC>;
#[doc = "Zero Cross Calibration VERGO_A Register"]
pub mod zero_cross_cal_a;
#[doc = "ZERO_CROSS_CAL_B (r) register accessor: Zero Cross Calibration VERGO_B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`zero_cross_cal_b::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zero_cross_cal_b`]
module"]
pub type ZERO_CROSS_CAL_B = crate::Reg<zero_cross_cal_b::ZERO_CROSS_CAL_B_SPEC>;
#[doc = "Zero Cross Calibration VERGO_B Register"]
pub mod zero_cross_cal_b;
#[doc = "ZERO_CROSS_CAL_C (r) register accessor: Zero Cross Calibration VERGO_C Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`zero_cross_cal_c::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zero_cross_cal_c`]
module"]
pub type ZERO_CROSS_CAL_C = crate::Reg<zero_cross_cal_c::ZERO_CROSS_CAL_C_SPEC>;
#[doc = "Zero Cross Calibration VERGO_C Register"]
pub mod zero_cross_cal_c;
#[doc = "ZERO_CROSS_CAL_D (r) register accessor: Zero Cross Calibration VERGO_D Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`zero_cross_cal_d::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zero_cross_cal_d`]
module"]
pub type ZERO_CROSS_CAL_D = crate::Reg<zero_cross_cal_d::ZERO_CROSS_CAL_D_SPEC>;
#[doc = "Zero Cross Calibration VERGO_D Register"]
pub mod zero_cross_cal_d;
