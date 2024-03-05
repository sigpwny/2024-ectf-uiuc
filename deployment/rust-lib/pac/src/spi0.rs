#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_fifo8: [u8; 0x04],
    ctrl0: CTRL0,
    ctrl1: CTRL1,
    ctrl2: CTRL2,
    sstime: SSTIME,
    clkctrl: CLKCTRL,
    _reserved6: [u8; 0x04],
    dma: DMA,
    intfl: INTFL,
    inten: INTEN,
    wkfl: WKFL,
    wken: WKEN,
    stat: STAT,
}
impl RegisterBlock {
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub const fn fifo8(&self, n: usize) -> &FIFO8 {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(0).add(1 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub fn fifo8_iter(&self) -> impl Iterator<Item = &FIFO8> {
        (0..4)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(0).add(1 * n).cast() })
    }
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub const fn fifo16(&self, n: usize) -> &FIFO16 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(0).add(2 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub fn fifo16_iter(&self) -> impl Iterator<Item = &FIFO16> {
        (0..2)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(0).add(2 * n).cast() })
    }
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub const fn fifo32(&self) -> &FIFO32 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x04 - Register for controlling SPI peripheral."]
    #[inline(always)]
    pub const fn ctrl0(&self) -> &CTRL0 {
        &self.ctrl0
    }
    #[doc = "0x08 - Register for controlling SPI peripheral."]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
    #[doc = "0x0c - Register for controlling SPI peripheral."]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    #[doc = "0x10 - Register for controlling SPI peripheral/Slave Select Timing."]
    #[inline(always)]
    pub const fn sstime(&self) -> &SSTIME {
        &self.sstime
    }
    #[doc = "0x14 - Register for controlling SPI clock rate."]
    #[inline(always)]
    pub const fn clkctrl(&self) -> &CLKCTRL {
        &self.clkctrl
    }
    #[doc = "0x1c - Register for controlling DMA."]
    #[inline(always)]
    pub const fn dma(&self) -> &DMA {
        &self.dma
    }
    #[doc = "0x20 - Register for reading and clearing interrupt flags. All bits are write 1 to clear."]
    #[inline(always)]
    pub const fn intfl(&self) -> &INTFL {
        &self.intfl
    }
    #[doc = "0x24 - Register for enabling interrupts."]
    #[inline(always)]
    pub const fn inten(&self) -> &INTEN {
        &self.inten
    }
    #[doc = "0x28 - Register for wake up flags. All bits in this register are write 1 to clear."]
    #[inline(always)]
    pub const fn wkfl(&self) -> &WKFL {
        &self.wkfl
    }
    #[doc = "0x2c - Register for wake up enable."]
    #[inline(always)]
    pub const fn wken(&self) -> &WKEN {
        &self.wken
    }
    #[doc = "0x30 - SPI Status register."]
    #[inline(always)]
    pub const fn stat(&self) -> &STAT {
        &self.stat
    }
}
#[doc = "FIFO32 (rw) register accessor: Register for reading and writing the FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo32`]
module"]
pub type FIFO32 = crate::Reg<fifo32::FIFO32_SPEC>;
#[doc = "Register for reading and writing the FIFO."]
pub mod fifo32;
#[doc = "FIFO16 (rw) register accessor: Register for reading and writing the FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo16`]
module"]
pub type FIFO16 = crate::Reg<fifo16::FIFO16_SPEC>;
#[doc = "Register for reading and writing the FIFO."]
pub mod fifo16;
#[doc = "FIFO8 (rw) register accessor: Register for reading and writing the FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo8`]
module"]
pub type FIFO8 = crate::Reg<fifo8::FIFO8_SPEC>;
#[doc = "Register for reading and writing the FIFO."]
pub mod fifo8;
#[doc = "CTRL0 (rw) register accessor: Register for controlling SPI peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl0`]
module"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "Register for controlling SPI peripheral."]
pub mod ctrl0;
#[doc = "CTRL1 (rw) register accessor: Register for controlling SPI peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`]
module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Register for controlling SPI peripheral."]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: Register for controlling SPI peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`]
module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Register for controlling SPI peripheral."]
pub mod ctrl2;
#[doc = "SSTIME (rw) register accessor: Register for controlling SPI peripheral/Slave Select Timing.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sstime::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sstime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstime`]
module"]
pub type SSTIME = crate::Reg<sstime::SSTIME_SPEC>;
#[doc = "Register for controlling SPI peripheral/Slave Select Timing."]
pub mod sstime;
#[doc = "CLKCTRL (rw) register accessor: Register for controlling SPI clock rate.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkctrl`]
module"]
pub type CLKCTRL = crate::Reg<clkctrl::CLKCTRL_SPEC>;
#[doc = "Register for controlling SPI clock rate."]
pub mod clkctrl;
#[doc = "DMA (rw) register accessor: Register for controlling DMA.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma`]
module"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "Register for controlling DMA."]
pub mod dma;
#[doc = "INTFL (rw) register accessor: Register for reading and clearing interrupt flags. All bits are write 1 to clear.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intfl`]
module"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "Register for reading and clearing interrupt flags. All bits are write 1 to clear."]
pub mod intfl;
#[doc = "INTEN (rw) register accessor: Register for enabling interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Register for enabling interrupts."]
pub mod inten;
#[doc = "WKFL (rw) register accessor: Register for wake up flags. All bits in this register are write 1 to clear.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wkfl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wkfl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkfl`]
module"]
pub type WKFL = crate::Reg<wkfl::WKFL_SPEC>;
#[doc = "Register for wake up flags. All bits in this register are write 1 to clear."]
pub mod wkfl;
#[doc = "WKEN (rw) register accessor: Register for wake up enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wken::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wken::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wken`]
module"]
pub type WKEN = crate::Reg<wken::WKEN_SPEC>;
#[doc = "Register for wake up enable."]
pub mod wken;
#[doc = "STAT (r) register accessor: SPI Status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "SPI Status register."]
pub mod stat;
