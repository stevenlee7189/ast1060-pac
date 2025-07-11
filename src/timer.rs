#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timer000: Timer000,
    timer004: Timer004,
    timer008: Timer008,
    timer00c: Timer00c,
}
impl RegisterBlock {
    #[doc = "0x00 - Counter \\#1 Status Register"]
    #[inline(always)]
    pub const fn timer000(&self) -> &Timer000 {
        &self.timer000
    }
    #[doc = "0x04 - Counter \\#1 Reload Value Register"]
    #[inline(always)]
    pub const fn timer004(&self) -> &Timer004 {
        &self.timer004
    }
    #[doc = "0x08 - Counter \\#1 First Matching Register"]
    #[inline(always)]
    pub const fn timer008(&self) -> &Timer008 {
        &self.timer008
    }
    #[doc = "0x0c - Counter \\#1 Second Matching Register"]
    #[inline(always)]
    pub const fn timer00c(&self) -> &Timer00c {
        &self.timer00c
    }
}
#[doc = "TIMER000 (rw) register accessor: Counter \\#1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer000`] module"]
#[doc(alias = "TIMER000")]
pub type Timer000 = crate::Reg<timer000::Timer000Spec>;
#[doc = "Counter \\#1 Status Register"]
pub mod timer000;
#[doc = "TIMER004 (rw) register accessor: Counter \\#1 Reload Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer004`] module"]
#[doc(alias = "TIMER004")]
pub type Timer004 = crate::Reg<timer004::Timer004Spec>;
#[doc = "Counter \\#1 Reload Value Register"]
pub mod timer004;
#[doc = "TIMER008 (rw) register accessor: Counter \\#1 First Matching Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer008`] module"]
#[doc(alias = "TIMER008")]
pub type Timer008 = crate::Reg<timer008::Timer008Spec>;
#[doc = "Counter \\#1 First Matching Register"]
pub mod timer008;
#[doc = "TIMER00C (rw) register accessor: Counter \\#1 Second Matching Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer00c`] module"]
#[doc(alias = "TIMER00C")]
pub type Timer00c = crate::Reg<timer00c::Timer00cSpec>;
#[doc = "Counter \\#1 Second Matching Register"]
pub mod timer00c;
