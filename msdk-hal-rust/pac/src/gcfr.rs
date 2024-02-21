#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    reg0: REG0,
    reg1: REG1,
    reg2: REG2,
    reg3: REG3,
}
impl RegisterBlock {
    #[doc = "0x00 - Register 0."]
    #[inline(always)]
    pub const fn reg0(&self) -> &REG0 {
        &self.reg0
    }
    #[doc = "0x04 - Register 1."]
    #[inline(always)]
    pub const fn reg1(&self) -> &REG1 {
        &self.reg1
    }
    #[doc = "0x08 - Register 2."]
    #[inline(always)]
    pub const fn reg2(&self) -> &REG2 {
        &self.reg2
    }
    #[doc = "0x0c - Register 3."]
    #[inline(always)]
    pub const fn reg3(&self) -> &REG3 {
        &self.reg3
    }
}
#[doc = "REG0 (rw) register accessor: Register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg0`]
module"]
pub type REG0 = crate::Reg<reg0::REG0_SPEC>;
#[doc = "Register 0."]
pub mod reg0;
#[doc = "REG1 (rw) register accessor: Register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg1`]
module"]
pub type REG1 = crate::Reg<reg1::REG1_SPEC>;
#[doc = "Register 1."]
pub mod reg1;
#[doc = "REG2 (rw) register accessor: Register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg2`]
module"]
pub type REG2 = crate::Reg<reg2::REG2_SPEC>;
#[doc = "Register 2."]
pub mod reg2;
#[doc = "REG3 (rw) register accessor: Register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg3`]
module"]
pub type REG3 = crate::Reg<reg3::REG3_SPEC>;
#[doc = "Register 3."]
pub mod reg3;
