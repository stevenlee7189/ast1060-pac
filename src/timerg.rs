#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x30],
    timerg030: Timerg030,
    timerg034: Timerg034,
    _reserved2: [u8; 0x04],
    timerg03c: Timerg03c,
}
impl RegisterBlock {
    #[doc = "0x30 - Control Register"]
    #[inline(always)]
    pub const fn timerg030(&self) -> &Timerg030 {
        &self.timerg030
    }
    #[doc = "0x34 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn timerg034(&self) -> &Timerg034 {
        &self.timerg034
    }
    #[doc = "0x3c - TMC30 Clear Register"]
    #[inline(always)]
    pub const fn timerg03c(&self) -> &Timerg03c {
        &self.timerg03c
    }
}
#[doc = "TIMERG030 (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timerg030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timerg030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timerg030`] module"]
#[doc(alias = "TIMERG030")]
pub type Timerg030 = crate::Reg<timerg030::Timerg030Spec>;
#[doc = "Control Register"]
pub mod timerg030;
#[doc = "TIMERG034 (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timerg034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timerg034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timerg034`] module"]
#[doc(alias = "TIMERG034")]
pub type Timerg034 = crate::Reg<timerg034::Timerg034Spec>;
#[doc = "Interrupt Status Register"]
pub mod timerg034;
#[doc = "TIMERG03C (rw) register accessor: TMC30 Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timerg03c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timerg03c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timerg03c`] module"]
#[doc(alias = "TIMERG03C")]
pub type Timerg03c = crate::Reg<timerg03c::Timerg03cSpec>;
#[doc = "TMC30 Clear Register"]
pub mod timerg03c;
