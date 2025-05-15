#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    spipf000: Spipf000,
    spipf004: Spipf004,
    spipf008: Spipf008,
    spipf00c: Spipf00c,
    spipf010: Spipf010,
    spipf014: Spipf014,
    spipf018: Spipf018,
    _reserved7: [u8; 0x60],
    spipf07c: Spipf07c,
    spipfwt: [Spipfwt; 32],
    spipfwa: [Spipfwa; 512],
}
impl RegisterBlock {
    #[doc = "0x00 - Engine Control Register"]
    #[inline(always)]
    pub const fn spipf000(&self) -> &Spipf000 {
        &self.spipf000
    }
    #[doc = "0x04 - Interrupt Enable and Status Register"]
    #[inline(always)]
    pub const fn spipf004(&self) -> &Spipf004 {
        &self.spipf004
    }
    #[doc = "0x08 - EAR and Over Speed Register"]
    #[inline(always)]
    pub const fn spipf008(&self) -> &Spipf008 {
        &self.spipf008
    }
    #[doc = "0x0c - Block FIFO Data Register"]
    #[inline(always)]
    pub const fn spipf00c(&self) -> &Spipf00c {
        &self.spipf00c
    }
    #[doc = "0x10 - Block Log DMA Base Address"]
    #[inline(always)]
    pub const fn spipf010(&self) -> &Spipf010 {
        &self.spipf010
    }
    #[doc = "0x14 - Block Log DMA Size"]
    #[inline(always)]
    pub const fn spipf014(&self) -> &Spipf014 {
        &self.spipf014
    }
    #[doc = "0x18 - Block Log DMA Write Pointer"]
    #[inline(always)]
    pub const fn spipf018(&self) -> &Spipf018 {
        &self.spipf018
    }
    #[doc = "0x7c - Write Disable Register"]
    #[inline(always)]
    pub const fn spipf07c(&self) -> &Spipf07c {
        &self.spipf07c
    }
    #[doc = "0x80..0x100 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipfwt(&self, n: usize) -> &Spipfwt {
        &self.spipfwt[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x100 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub fn spipfwt_iter(&self) -> impl Iterator<Item = &Spipfwt> {
        self.spipfwt.iter()
    }
    #[doc = "0x100..0x900 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub const fn spipfwa(&self, n: usize) -> &Spipfwa {
        &self.spipfwa[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x900 - Write Address Table (\\hlink{SPIPF000"]
    #[inline(always)]
    pub fn spipfwa_iter(&self) -> impl Iterator<Item = &Spipfwa> {
        self.spipfwa.iter()
    }
}
#[doc = "SPIPF000 (rw) register accessor: Engine Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf000`] module"]
#[doc(alias = "SPIPF000")]
pub type Spipf000 = crate::Reg<spipf000::Spipf000Spec>;
#[doc = "Engine Control Register"]
pub mod spipf000;
#[doc = "SPIPF004 (rw) register accessor: Interrupt Enable and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf004`] module"]
#[doc(alias = "SPIPF004")]
pub type Spipf004 = crate::Reg<spipf004::Spipf004Spec>;
#[doc = "Interrupt Enable and Status Register"]
pub mod spipf004;
#[doc = "SPIPF008 (rw) register accessor: EAR and Over Speed Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf008`] module"]
#[doc(alias = "SPIPF008")]
pub type Spipf008 = crate::Reg<spipf008::Spipf008Spec>;
#[doc = "EAR and Over Speed Register"]
pub mod spipf008;
#[doc = "SPIPF00C (rw) register accessor: Block FIFO Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf00c`] module"]
#[doc(alias = "SPIPF00C")]
pub type Spipf00c = crate::Reg<spipf00c::Spipf00cSpec>;
#[doc = "Block FIFO Data Register"]
pub mod spipf00c;
#[doc = "SPIPF010 (rw) register accessor: Block Log DMA Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf010`] module"]
#[doc(alias = "SPIPF010")]
pub type Spipf010 = crate::Reg<spipf010::Spipf010Spec>;
#[doc = "Block Log DMA Base Address"]
pub mod spipf010;
#[doc = "SPIPF014 (rw) register accessor: Block Log DMA Size\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf014`] module"]
#[doc(alias = "SPIPF014")]
pub type Spipf014 = crate::Reg<spipf014::Spipf014Spec>;
#[doc = "Block Log DMA Size"]
pub mod spipf014;
#[doc = "SPIPF018 (rw) register accessor: Block Log DMA Write Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf018`] module"]
#[doc(alias = "SPIPF018")]
pub type Spipf018 = crate::Reg<spipf018::Spipf018Spec>;
#[doc = "Block Log DMA Write Pointer"]
pub mod spipf018;
#[doc = "SPIPF07C (rw) register accessor: Write Disable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf07c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf07c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf07c`] module"]
#[doc(alias = "SPIPF07C")]
pub type Spipf07c = crate::Reg<spipf07c::Spipf07cSpec>;
#[doc = "Write Disable Register"]
pub mod spipf07c;
#[doc = "SPIPFWT (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipfwt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipfwt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipfwt`] module"]
#[doc(alias = "SPIPFWT")]
pub type Spipfwt = crate::Reg<spipfwt::SpipfwtSpec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipfwt;
#[doc = "SPIPFWA (rw) register accessor: Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipfwa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipfwa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipfwa`] module"]
#[doc(alias = "SPIPFWA")]
pub type Spipfwa = crate::Reg<spipfwa::SpipfwaSpec>;
#[doc = "Write Address Table (\\hlink{SPIPF000"]
pub mod spipfwa;
