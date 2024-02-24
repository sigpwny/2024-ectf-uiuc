#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    rate_length: RATE_LENGTH,
    train: TRAIN,
    loop_: LOOP,
    restart: RESTART,
}
impl RegisterBlock {
    #[doc = "0x00 - Pulse Train Configuration"]
    #[inline(always)]
    pub const fn rate_length(&self) -> &RATE_LENGTH {
        &self.rate_length
    }
    #[doc = "0x04 - Write the repeating bit pattern that is shifted out, LSB first, when configured in Pulse Train mode. See PT_RATE_LENGTH.mode for setting the length."]
    #[inline(always)]
    pub const fn train(&self) -> &TRAIN {
        &self.train
    }
    #[doc = "0x08 - Pulse Train Loop Count"]
    #[inline(always)]
    pub const fn loop_(&self) -> &LOOP {
        &self.loop_
    }
    #[doc = "0x0c - Pulse Train Auto-Restart Configuration."]
    #[inline(always)]
    pub const fn restart(&self) -> &RESTART {
        &self.restart
    }
}
#[doc = "RATE_LENGTH (rw) register accessor: Pulse Train Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rate_length::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rate_length::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rate_length`]
module"]
pub type RATE_LENGTH = crate::Reg<rate_length::RATE_LENGTH_SPEC>;
#[doc = "Pulse Train Configuration"]
pub mod rate_length;
#[doc = "TRAIN (rw) register accessor: Write the repeating bit pattern that is shifted out, LSB first, when configured in Pulse Train mode. See PT_RATE_LENGTH.mode for setting the length.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`train::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`train::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@train`]
module"]
pub type TRAIN = crate::Reg<train::TRAIN_SPEC>;
#[doc = "Write the repeating bit pattern that is shifted out, LSB first, when configured in Pulse Train mode. See PT_RATE_LENGTH.mode for setting the length."]
pub mod train;
#[doc = "LOOP (rw) register accessor: Pulse Train Loop Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`loop_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`loop_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@loop_`]
module"]
pub type LOOP = crate::Reg<loop_::LOOP_SPEC>;
#[doc = "Pulse Train Loop Count"]
pub mod loop_;
#[doc = "RESTART (rw) register accessor: Pulse Train Auto-Restart Configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`restart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`restart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@restart`]
module"]
pub type RESTART = crate::Reg<restart::RESTART_SPEC>;
#[doc = "Pulse Train Auto-Restart Configuration."]
pub mod restart;
