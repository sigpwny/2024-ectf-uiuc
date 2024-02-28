#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    status: STATUS,
    int_en: INT_EN,
    int_fl: INT_FL,
    clkdiv: CLKDIV,
    osr: OSR,
    txpeek: TXPEEK,
    pnr: PNR,
    fifo: FIFO,
    _reserved9: [u8; 0x0c],
    dma: DMA,
    wken: WKEN,
    wkfl: WKFL,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Status register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x08 - Interrupt Enable control register"]
    #[inline(always)]
    pub const fn int_en(&self) -> &INT_EN {
        &self.int_en
    }
    #[doc = "0x0c - Interrupt status flags Control register"]
    #[inline(always)]
    pub const fn int_fl(&self) -> &INT_FL {
        &self.int_fl
    }
    #[doc = "0x10 - Clock Divider register"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &CLKDIV {
        &self.clkdiv
    }
    #[doc = "0x14 - Over Sampling Rate register"]
    #[inline(always)]
    pub const fn osr(&self) -> &OSR {
        &self.osr
    }
    #[doc = "0x18 - TX FIFO Output Peek register"]
    #[inline(always)]
    pub const fn txpeek(&self) -> &TXPEEK {
        &self.txpeek
    }
    #[doc = "0x1c - Pin register"]
    #[inline(always)]
    pub const fn pnr(&self) -> &PNR {
        &self.pnr
    }
    #[doc = "0x20 - FIFO Read/Write register"]
    #[inline(always)]
    pub const fn fifo(&self) -> &FIFO {
        &self.fifo
    }
    #[doc = "0x30 - DMA Configuration register"]
    #[inline(always)]
    pub const fn dma(&self) -> &DMA {
        &self.dma
    }
    #[doc = "0x34 - Wake up enable Control register"]
    #[inline(always)]
    pub const fn wken(&self) -> &WKEN {
        &self.wken
    }
    #[doc = "0x38 - Wake up Flags register"]
    #[inline(always)]
    pub const fn wkfl(&self) -> &WKFL {
        &self.wkfl
    }
}
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register"]
pub mod status;
#[doc = "INT_EN (rw) register accessor: Interrupt Enable control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_en`]
module"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "Interrupt Enable control register"]
pub mod int_en;
#[doc = "INT_FL (rw) register accessor: Interrupt status flags Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_fl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_fl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_fl`]
module"]
pub type INT_FL = crate::Reg<int_fl::INT_FL_SPEC>;
#[doc = "Interrupt status flags Control register"]
pub mod int_fl;
#[doc = "CLKDIV (rw) register accessor: Clock Divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`]
module"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock Divider register"]
pub mod clkdiv;
#[doc = "OSR (rw) register accessor: Over Sampling Rate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osr`]
module"]
pub type OSR = crate::Reg<osr::OSR_SPEC>;
#[doc = "Over Sampling Rate register"]
pub mod osr;
#[doc = "TXPEEK (rw) register accessor: TX FIFO Output Peek register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txpeek::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txpeek::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txpeek`]
module"]
pub type TXPEEK = crate::Reg<txpeek::TXPEEK_SPEC>;
#[doc = "TX FIFO Output Peek register"]
pub mod txpeek;
#[doc = "PNR (rw) register accessor: Pin register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pnr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pnr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pnr`]
module"]
pub type PNR = crate::Reg<pnr::PNR_SPEC>;
#[doc = "Pin register"]
pub mod pnr;
#[doc = "FIFO (rw) register accessor: FIFO Read/Write register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`]
module"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "FIFO Read/Write register"]
pub mod fifo;
#[doc = "DMA (rw) register accessor: DMA Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma`]
module"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "DMA Configuration register"]
pub mod dma;
#[doc = "WKEN (rw) register accessor: Wake up enable Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wken::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wken::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wken`]
module"]
pub type WKEN = crate::Reg<wken::WKEN_SPEC>;
#[doc = "Wake up enable Control register"]
pub mod wken;
#[doc = "WKFL (rw) register accessor: Wake up Flags register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wkfl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wkfl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkfl`]
module"]
pub type WKFL = crate::Reg<wkfl::WKFL_SPEC>;
#[doc = "Wake up Flags register"]
pub mod wkfl;
