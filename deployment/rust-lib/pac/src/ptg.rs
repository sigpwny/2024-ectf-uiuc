#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    enable: ENABLE,
    resync: RESYNC,
    intfl: INTFL,
    inten: INTEN,
    safe_en: SAFE_EN,
    safe_dis: SAFE_DIS,
}
impl RegisterBlock {
    #[doc = "0x00 - Global Enable/Disable Controls for All Pulse Trains"]
    #[inline(always)]
    pub const fn enable(&self) -> &ENABLE {
        &self.enable
    }
    #[doc = "0x04 - Global Resync (All Pulse Trains) Control"]
    #[inline(always)]
    pub const fn resync(&self) -> &RESYNC {
        &self.resync
    }
    #[doc = "0x08 - Pulse Train Interrupt Flags"]
    #[inline(always)]
    pub const fn intfl(&self) -> &INTFL {
        &self.intfl
    }
    #[doc = "0x0c - Pulse Train Interrupt Enable/Disable"]
    #[inline(always)]
    pub const fn inten(&self) -> &INTEN {
        &self.inten
    }
    #[doc = "0x10 - Pulse Train Global Safe Enable."]
    #[inline(always)]
    pub const fn safe_en(&self) -> &SAFE_EN {
        &self.safe_en
    }
    #[doc = "0x14 - Pulse Train Global Safe Disable."]
    #[inline(always)]
    pub const fn safe_dis(&self) -> &SAFE_DIS {
        &self.safe_dis
    }
}
#[doc = "ENABLE (rw) register accessor: Global Enable/Disable Controls for All Pulse Trains\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Global Enable/Disable Controls for All Pulse Trains"]
pub mod enable;
#[doc = "RESYNC (rw) register accessor: Global Resync (All Pulse Trains) Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resync`]
module"]
pub type RESYNC = crate::Reg<resync::RESYNC_SPEC>;
#[doc = "Global Resync (All Pulse Trains) Control"]
pub mod resync;
#[doc = "INTFL (rw) register accessor: Pulse Train Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intfl`]
module"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "Pulse Train Interrupt Flags"]
pub mod intfl;
#[doc = "INTEN (rw) register accessor: Pulse Train Interrupt Enable/Disable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Pulse Train Interrupt Enable/Disable"]
pub mod inten;
#[doc = "SAFE_EN (w) register accessor: Pulse Train Global Safe Enable.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`safe_en::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@safe_en`]
module"]
pub type SAFE_EN = crate::Reg<safe_en::SAFE_EN_SPEC>;
#[doc = "Pulse Train Global Safe Enable."]
pub mod safe_en;
#[doc = "SAFE_DIS (w) register accessor: Pulse Train Global Safe Disable.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`safe_dis::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@safe_dis`]
module"]
pub type SAFE_DIS = crate::Reg<safe_dis::SAFE_DIS_SPEC>;
#[doc = "Pulse Train Global Safe Disable."]
pub mod safe_dis;
