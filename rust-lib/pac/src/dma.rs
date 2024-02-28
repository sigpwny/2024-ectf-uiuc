#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    inten: INTEN,
    intfl: INTFL,
    _reserved2: [u8; 0xf8],
    ch: [CH; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - DMA Control Register."]
    #[inline(always)]
    pub const fn inten(&self) -> &INTEN {
        &self.inten
    }
    #[doc = "0x04 - DMA Interrupt Register."]
    #[inline(always)]
    pub const fn intfl(&self) -> &INTFL {
        &self.intfl
    }
    #[doc = "0x100..0x180 - DMA Channel registers."]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x180 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
}
#[doc = "INTEN (rw) register accessor: DMA Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "DMA Control Register."]
pub mod inten;
#[doc = "INTFL (r) register accessor: DMA Interrupt Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intfl`]
module"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "DMA Interrupt Register."]
pub mod intfl;
#[doc = "DMA Channel registers."]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "DMA Channel registers."]
pub mod ch;
