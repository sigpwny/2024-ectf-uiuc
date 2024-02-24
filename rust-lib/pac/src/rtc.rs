#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    sec: SEC,
    ssec: SSEC,
    toda: TODA,
    sseca: SSECA,
    ctrl: CTRL,
    trim: TRIM,
    oscctrl: OSCCTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - RTC Second Counter. This register contains the 32-bit second counter."]
    #[inline(always)]
    pub const fn sec(&self) -> &SEC {
        &self.sec
    }
    #[doc = "0x04 - RTC Sub-second Counter. This counter increments at 256Hz. RTC_SEC is incremented when this register rolls over from 0xFF to 0x00."]
    #[inline(always)]
    pub const fn ssec(&self) -> &SSEC {
        &self.ssec
    }
    #[doc = "0x08 - Time-of-day Alarm."]
    #[inline(always)]
    pub const fn toda(&self) -> &TODA {
        &self.toda
    }
    #[doc = "0x0c - RTC sub-second alarm. This register contains the reload value for the sub-second alarm."]
    #[inline(always)]
    pub const fn sseca(&self) -> &SSECA {
        &self.sseca
    }
    #[doc = "0x10 - RTC Control Register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x14 - RTC Trim Register."]
    #[inline(always)]
    pub const fn trim(&self) -> &TRIM {
        &self.trim
    }
    #[doc = "0x18 - RTC Oscillator Control Register."]
    #[inline(always)]
    pub const fn oscctrl(&self) -> &OSCCTRL {
        &self.oscctrl
    }
}
#[doc = "SEC (rw) register accessor: RTC Second Counter. This register contains the 32-bit second counter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec`]
module"]
pub type SEC = crate::Reg<sec::SEC_SPEC>;
#[doc = "RTC Second Counter. This register contains the 32-bit second counter."]
pub mod sec;
#[doc = "SSEC (rw) register accessor: RTC Sub-second Counter. This counter increments at 256Hz. RTC_SEC is incremented when this register rolls over from 0xFF to 0x00.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssec`]
module"]
pub type SSEC = crate::Reg<ssec::SSEC_SPEC>;
#[doc = "RTC Sub-second Counter. This counter increments at 256Hz. RTC_SEC is incremented when this register rolls over from 0xFF to 0x00."]
pub mod ssec;
#[doc = "TODA (rw) register accessor: Time-of-day Alarm.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`toda::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`toda::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@toda`]
module"]
pub type TODA = crate::Reg<toda::TODA_SPEC>;
#[doc = "Time-of-day Alarm."]
pub mod toda;
#[doc = "SSECA (rw) register accessor: RTC sub-second alarm. This register contains the reload value for the sub-second alarm.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sseca::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sseca::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sseca`]
module"]
pub type SSECA = crate::Reg<sseca::SSECA_SPEC>;
#[doc = "RTC sub-second alarm. This register contains the reload value for the sub-second alarm."]
pub mod sseca;
#[doc = "CTRL (rw) register accessor: RTC Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "RTC Control Register."]
pub mod ctrl;
#[doc = "TRIM (rw) register accessor: RTC Trim Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trim`]
module"]
pub type TRIM = crate::Reg<trim::TRIM_SPEC>;
#[doc = "RTC Trim Register."]
pub mod trim;
#[doc = "OSCCTRL (rw) register accessor: RTC Oscillator Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oscctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscctrl`]
module"]
pub type OSCCTRL = crate::Reg<oscctrl::OSCCTRL_SPEC>;
#[doc = "RTC Oscillator Control Register."]
pub mod oscctrl;
