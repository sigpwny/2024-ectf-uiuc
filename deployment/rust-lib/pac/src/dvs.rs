#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctl: CTL,
    stat: STAT,
    direct: DIRECT,
    mon: MON,
    adj_up: ADJ_UP,
    adj_dwn: ADJ_DWN,
    thres_cmp: THRES_CMP,
    tap_sel: [TAP_SEL; 5],
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &CTL {
        &self.ctl
    }
    #[doc = "0x04 - Status Fields"]
    #[inline(always)]
    pub const fn stat(&self) -> &STAT {
        &self.stat
    }
    #[doc = "0x08 - Direct control of target voltage"]
    #[inline(always)]
    pub const fn direct(&self) -> &DIRECT {
        &self.direct
    }
    #[doc = "0x0c - Monitor Delay"]
    #[inline(always)]
    pub const fn mon(&self) -> &MON {
        &self.mon
    }
    #[doc = "0x10 - Up Delay Register"]
    #[inline(always)]
    pub const fn adj_up(&self) -> &ADJ_UP {
        &self.adj_up
    }
    #[doc = "0x14 - Down Delay Register"]
    #[inline(always)]
    pub const fn adj_dwn(&self) -> &ADJ_DWN {
        &self.adj_dwn
    }
    #[doc = "0x18 - Up Delay Register"]
    #[inline(always)]
    pub const fn thres_cmp(&self) -> &THRES_CMP {
        &self.thres_cmp
    }
    #[doc = "0x1c..0x30 - DVS Tap Select Register"]
    #[inline(always)]
    pub const fn tap_sel(&self, n: usize) -> &TAP_SEL {
        &self.tap_sel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c..0x30 - DVS Tap Select Register"]
    #[inline(always)]
    pub fn tap_sel_iter(&self) -> impl Iterator<Item = &TAP_SEL> {
        self.tap_sel.iter()
    }
}
#[doc = "CTL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control Register"]
pub mod ctl;
#[doc = "STAT (rw) register accessor: Status Fields\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status Fields"]
pub mod stat;
#[doc = "DIRECT (rw) register accessor: Direct control of target voltage\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`direct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`direct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@direct`]
module"]
pub type DIRECT = crate::Reg<direct::DIRECT_SPEC>;
#[doc = "Direct control of target voltage"]
pub mod direct;
#[doc = "MON (rw) register accessor: Monitor Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mon`]
module"]
pub type MON = crate::Reg<mon::MON_SPEC>;
#[doc = "Monitor Delay"]
pub mod mon;
#[doc = "ADJ_UP (rw) register accessor: Up Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adj_up::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adj_up::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adj_up`]
module"]
pub type ADJ_UP = crate::Reg<adj_up::ADJ_UP_SPEC>;
#[doc = "Up Delay Register"]
pub mod adj_up;
#[doc = "ADJ_DWN (rw) register accessor: Down Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adj_dwn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adj_dwn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adj_dwn`]
module"]
pub type ADJ_DWN = crate::Reg<adj_dwn::ADJ_DWN_SPEC>;
#[doc = "Down Delay Register"]
pub mod adj_dwn;
#[doc = "THRES_CMP (rw) register accessor: Up Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`thres_cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thres_cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thres_cmp`]
module"]
pub type THRES_CMP = crate::Reg<thres_cmp::THRES_CMP_SPEC>;
#[doc = "Up Delay Register"]
pub mod thres_cmp;
#[doc = "TAP_SEL (rw) register accessor: DVS Tap Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tap_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tap_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tap_sel`]
module"]
pub type TAP_SEL = crate::Reg<tap_sel::TAP_SEL_SPEC>;
#[doc = "DVS Tap Select Register"]
pub mod tap_sel;
