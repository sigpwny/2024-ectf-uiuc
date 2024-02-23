#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl0ch0: CTRL0CH0,
    _reserved1: [u8; 0x0c],
    ctrl1ch0: CTRL1CH0,
    _reserved2: [u8; 0x0c],
    filtch0: FILTCH0,
    _reserved3: [u8; 0x0c],
    dmach0: DMACH0,
    _reserved4: [u8; 0x0c],
    fifoch0: FIFOCH0,
    _reserved5: [u8; 0x0c],
    intfl: INTFL,
    inten: INTEN,
    extsetup: EXTSETUP,
    wken: WKEN,
    wkfl: WKFL,
}
impl RegisterBlock {
    #[doc = "0x00 - Global mode channel."]
    #[inline(always)]
    pub const fn ctrl0ch0(&self) -> &CTRL0CH0 {
        &self.ctrl0ch0
    }
    #[doc = "0x10 - Local channel Setup."]
    #[inline(always)]
    pub const fn ctrl1ch0(&self) -> &CTRL1CH0 {
        &self.ctrl1ch0
    }
    #[doc = "0x20 - Filter."]
    #[inline(always)]
    pub const fn filtch0(&self) -> &FILTCH0 {
        &self.filtch0
    }
    #[doc = "0x30 - DMA Control."]
    #[inline(always)]
    pub const fn dmach0(&self) -> &DMACH0 {
        &self.dmach0
    }
    #[doc = "0x40 - I2S Fifo."]
    #[inline(always)]
    pub const fn fifoch0(&self) -> &FIFOCH0 {
        &self.fifoch0
    }
    #[doc = "0x50 - ISR Status."]
    #[inline(always)]
    pub const fn intfl(&self) -> &INTFL {
        &self.intfl
    }
    #[doc = "0x54 - Interrupt Enable."]
    #[inline(always)]
    pub const fn inten(&self) -> &INTEN {
        &self.inten
    }
    #[doc = "0x58 - Ext Control."]
    #[inline(always)]
    pub const fn extsetup(&self) -> &EXTSETUP {
        &self.extsetup
    }
    #[doc = "0x5c - Wakeup Enable."]
    #[inline(always)]
    pub const fn wken(&self) -> &WKEN {
        &self.wken
    }
    #[doc = "0x60 - Wakeup Flags."]
    #[inline(always)]
    pub const fn wkfl(&self) -> &WKFL {
        &self.wkfl
    }
}
#[doc = "CTRL0CH0 (rw) register accessor: Global mode channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl0ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl0ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl0ch0`]
module"]
pub type CTRL0CH0 = crate::Reg<ctrl0ch0::CTRL0CH0_SPEC>;
#[doc = "Global mode channel."]
pub mod ctrl0ch0;
#[doc = "CTRL1CH0 (rw) register accessor: Local channel Setup.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1ch0`]
module"]
pub type CTRL1CH0 = crate::Reg<ctrl1ch0::CTRL1CH0_SPEC>;
#[doc = "Local channel Setup."]
pub mod ctrl1ch0;
#[doc = "FILTCH0 (rw) register accessor: Filter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filtch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filtch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filtch0`]
module"]
pub type FILTCH0 = crate::Reg<filtch0::FILTCH0_SPEC>;
#[doc = "Filter."]
pub mod filtch0;
#[doc = "DMACH0 (rw) register accessor: DMA Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach0`]
module"]
pub type DMACH0 = crate::Reg<dmach0::DMACH0_SPEC>;
#[doc = "DMA Control."]
pub mod dmach0;
#[doc = "FIFOCH0 (rw) register accessor: I2S Fifo.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifoch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifoch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifoch0`]
module"]
pub type FIFOCH0 = crate::Reg<fifoch0::FIFOCH0_SPEC>;
#[doc = "I2S Fifo."]
pub mod fifoch0;
#[doc = "INTFL (rw) register accessor: ISR Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intfl`]
module"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "ISR Status."]
pub mod intfl;
#[doc = "INTEN (rw) register accessor: Interrupt Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt Enable."]
pub mod inten;
#[doc = "EXTSETUP (rw) register accessor: Ext Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extsetup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extsetup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extsetup`]
module"]
pub type EXTSETUP = crate::Reg<extsetup::EXTSETUP_SPEC>;
#[doc = "Ext Control."]
pub mod extsetup;
#[doc = "WKEN (rw) register accessor: Wakeup Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wken::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wken::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wken`]
module"]
pub type WKEN = crate::Reg<wken::WKEN_SPEC>;
#[doc = "Wakeup Enable."]
pub mod wken;
#[doc = "WKFL (rw) register accessor: Wakeup Flags.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wkfl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wkfl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkfl`]
module"]
pub type WKFL = crate::Reg<wkfl::WKFL_SPEC>;
#[doc = "Wakeup Flags."]
pub mod wkfl;
