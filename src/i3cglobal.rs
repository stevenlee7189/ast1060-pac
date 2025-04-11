#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    i3c010: I3c010,
    i3c014: I3c014,
    mi3c18: Mi3c18,
    mi3c1c: Mi3c1c,
    i3c020: I3c020,
    i3c024: I3c024,
    mi3c28: Mi3c28,
    mi3c2c: Mi3c2c,
    i3c030: I3c030,
    i3c034: I3c034,
    mi3c38: Mi3c38,
    mi3c3c: Mi3c3c,
    i3c040: I3c040,
    i3c044: I3c044,
    mi3c48: Mi3c48,
    mi3c4c: Mi3c4c,
    i3c050: I3c050,
    i3c054: I3c054,
    mi3c58: Mi3c58,
    mi3c5c: Mi3c5c,
    i3c060: I3c060,
    i3c064: I3c064,
    mi3c68: Mi3c68,
    mi3c6c: Mi3c6c,
}
impl RegisterBlock {
    #[doc = "0x10 - I3C1Reg0"]
    #[inline(always)]
    pub const fn i3c010(&self) -> &I3c010 {
        &self.i3c010
    }
    #[doc = "0x14 - I3C1Reg1"]
    #[inline(always)]
    pub const fn i3c014(&self) -> &I3c014 {
        &self.i3c014
    }
    #[doc = "0x18 - I3C1Dbg1 (Master)"]
    #[inline(always)]
    pub const fn mi3c18(&self) -> &Mi3c18 {
        &self.mi3c18
    }
    #[doc = "0x1c - I3C1Dbg2 (Master)"]
    #[inline(always)]
    pub const fn mi3c1c(&self) -> &Mi3c1c {
        &self.mi3c1c
    }
    #[doc = "0x20 - I3C2Reg0"]
    #[inline(always)]
    pub const fn i3c020(&self) -> &I3c020 {
        &self.i3c020
    }
    #[doc = "0x24 - I3C2Reg1"]
    #[inline(always)]
    pub const fn i3c024(&self) -> &I3c024 {
        &self.i3c024
    }
    #[doc = "0x28 - I3C2Dbg1 (Master)"]
    #[inline(always)]
    pub const fn mi3c28(&self) -> &Mi3c28 {
        &self.mi3c28
    }
    #[doc = "0x2c - I3C2Dbg2 (Master)"]
    #[inline(always)]
    pub const fn mi3c2c(&self) -> &Mi3c2c {
        &self.mi3c2c
    }
    #[doc = "0x30 - I3C3Reg0"]
    #[inline(always)]
    pub const fn i3c030(&self) -> &I3c030 {
        &self.i3c030
    }
    #[doc = "0x34 - I3C3Reg1"]
    #[inline(always)]
    pub const fn i3c034(&self) -> &I3c034 {
        &self.i3c034
    }
    #[doc = "0x38 - I3C3Dbg1 (Master)"]
    #[inline(always)]
    pub const fn mi3c38(&self) -> &Mi3c38 {
        &self.mi3c38
    }
    #[doc = "0x3c - I3C3Dbg2 (Master)"]
    #[inline(always)]
    pub const fn mi3c3c(&self) -> &Mi3c3c {
        &self.mi3c3c
    }
    #[doc = "0x40 - I3C4Reg0"]
    #[inline(always)]
    pub const fn i3c040(&self) -> &I3c040 {
        &self.i3c040
    }
    #[doc = "0x44 - I3C4Reg1"]
    #[inline(always)]
    pub const fn i3c044(&self) -> &I3c044 {
        &self.i3c044
    }
    #[doc = "0x48 - I3C4Dbg1 (Master)"]
    #[inline(always)]
    pub const fn mi3c48(&self) -> &Mi3c48 {
        &self.mi3c48
    }
    #[doc = "0x4c - I3C4Dbg2 (Master)"]
    #[inline(always)]
    pub const fn mi3c4c(&self) -> &Mi3c4c {
        &self.mi3c4c
    }
    #[doc = "0x50 - I3C5Reg0"]
    #[inline(always)]
    pub const fn i3c050(&self) -> &I3c050 {
        &self.i3c050
    }
    #[doc = "0x54 - I3C5Reg1"]
    #[inline(always)]
    pub const fn i3c054(&self) -> &I3c054 {
        &self.i3c054
    }
    #[doc = "0x58 - I3C5Dbg1 (Master)"]
    #[inline(always)]
    pub const fn mi3c58(&self) -> &Mi3c58 {
        &self.mi3c58
    }
    #[doc = "0x5c - I3C5Dbg2 (Master)"]
    #[inline(always)]
    pub const fn mi3c5c(&self) -> &Mi3c5c {
        &self.mi3c5c
    }
    #[doc = "0x60 - I3C6Reg0"]
    #[inline(always)]
    pub const fn i3c060(&self) -> &I3c060 {
        &self.i3c060
    }
    #[doc = "0x64 - I3C6Reg1"]
    #[inline(always)]
    pub const fn i3c064(&self) -> &I3c064 {
        &self.i3c064
    }
    #[doc = "0x68 - I3C6Dbg1 (Master)"]
    #[inline(always)]
    pub const fn mi3c68(&self) -> &Mi3c68 {
        &self.mi3c68
    }
    #[doc = "0x6c - I3C6Dbg2 (Master)"]
    #[inline(always)]
    pub const fn mi3c6c(&self) -> &Mi3c6c {
        &self.mi3c6c
    }
}
#[doc = "I3C010 (rw) register accessor: I3C1Reg0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3c010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3c010`] module"]
#[doc(alias = "I3C010")]
pub type I3c010 = crate::Reg<i3c010::I3c010Spec>;
#[doc = "I3C1Reg0"]
pub mod i3c010;
#[doc = "I3C014 (rw) register accessor: I3C1Reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`i3c014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3c014`] module"]
#[doc(alias = "I3C014")]
pub type I3c014 = crate::Reg<i3c014::I3c014Spec>;
#[doc = "I3C1Reg1"]
pub mod i3c014;
#[doc = "MI3C18 (rw) register accessor: I3C1Dbg1 (Master)\n\nYou can [`read`](crate::Reg::read) this register and get [`mi3c18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mi3c18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi3c18`] module"]
#[doc(alias = "MI3C18")]
pub type Mi3c18 = crate::Reg<mi3c18::Mi3c18Spec>;
#[doc = "I3C1Dbg1 (Master)"]
pub mod mi3c18;
#[doc = "MI3C1C (rw) register accessor: I3C1Dbg2 (Master)\n\nYou can [`read`](crate::Reg::read) this register and get [`mi3c1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mi3c1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi3c1c`] module"]
#[doc(alias = "MI3C1C")]
pub type Mi3c1c = crate::Reg<mi3c1c::Mi3c1cSpec>;
#[doc = "I3C1Dbg2 (Master)"]
pub mod mi3c1c;
#[doc = "I3C020 (rw) register accessor: I3C2Reg0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3c020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3c020`] module"]
#[doc(alias = "I3C020")]
pub type I3c020 = crate::Reg<i3c020::I3c020Spec>;
#[doc = "I3C2Reg0"]
pub mod i3c020;
#[doc = "I3C024 (rw) register accessor: I3C2Reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`i3c024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3c024`] module"]
#[doc(alias = "I3C024")]
pub type I3c024 = crate::Reg<i3c024::I3c024Spec>;
#[doc = "I3C2Reg1"]
pub mod i3c024;
#[doc = "MI3C28 (rw) register accessor: I3C2Dbg1 (Master)\n\nYou can [`read`](crate::Reg::read) this register and get [`mi3c28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mi3c28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi3c28`] module"]
#[doc(alias = "MI3C28")]
pub type Mi3c28 = crate::Reg<mi3c28::Mi3c28Spec>;
#[doc = "I3C2Dbg1 (Master)"]
pub mod mi3c28;
#[doc = "MI3C2C (rw) register accessor: I3C2Dbg2 (Master)\n\nYou can [`read`](crate::Reg::read) this register and get [`mi3c2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mi3c2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi3c2c`] module"]
#[doc(alias = "MI3C2C")]
pub type Mi3c2c = crate::Reg<mi3c2c::Mi3c2cSpec>;
#[doc = "I3C2Dbg2 (Master)"]
pub mod mi3c2c;
#[doc = "I3C030 (rw) register accessor: I3C3Reg0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3c030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3c030`] module"]
#[doc(alias = "I3C030")]
pub type I3c030 = crate::Reg<i3c030::I3c030Spec>;
#[doc = "I3C3Reg0"]
pub mod i3c030;
#[doc = "I3C034 (rw) register accessor: I3C3Reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`i3c034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3c034`] module"]
#[doc(alias = "I3C034")]
pub type I3c034 = crate::Reg<i3c034::I3c034Spec>;
#[doc = "I3C3Reg1"]
pub mod i3c034;
#[doc = "MI3C38 (rw) register accessor: I3C3Dbg1 (Master)\n\nYou can [`read`](crate::Reg::read) this register and get [`mi3c38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mi3c38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi3c38`] module"]
#[doc(alias = "MI3C38")]
pub type Mi3c38 = crate::Reg<mi3c38::Mi3c38Spec>;
#[doc = "I3C3Dbg1 (Master)"]
pub mod mi3c38;
#[doc = "MI3C3C (rw) register accessor: I3C3Dbg2 (Master)\n\nYou can [`read`](crate::Reg::read) this register and get [`mi3c3c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mi3c3c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi3c3c`] module"]
#[doc(alias = "MI3C3C")]
pub type Mi3c3c = crate::Reg<mi3c3c::Mi3c3cSpec>;
#[doc = "I3C3Dbg2 (Master)"]
pub mod mi3c3c;
#[doc = "I3C040 (rw) register accessor: I3C4Reg0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3c040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3c040`] module"]
#[doc(alias = "I3C040")]
pub type I3c040 = crate::Reg<i3c040::I3c040Spec>;
#[doc = "I3C4Reg0"]
pub mod i3c040;
#[doc = "I3C044 (rw) register accessor: I3C4Reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`i3c044::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c044::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3c044`] module"]
#[doc(alias = "I3C044")]
pub type I3c044 = crate::Reg<i3c044::I3c044Spec>;
#[doc = "I3C4Reg1"]
pub mod i3c044;
#[doc = "MI3C48 (rw) register accessor: I3C4Dbg1 (Master)\n\nYou can [`read`](crate::Reg::read) this register and get [`mi3c48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mi3c48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi3c48`] module"]
#[doc(alias = "MI3C48")]
pub type Mi3c48 = crate::Reg<mi3c48::Mi3c48Spec>;
#[doc = "I3C4Dbg1 (Master)"]
pub mod mi3c48;
#[doc = "MI3C4C (rw) register accessor: I3C4Dbg2 (Master)\n\nYou can [`read`](crate::Reg::read) this register and get [`mi3c4c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mi3c4c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi3c4c`] module"]
#[doc(alias = "MI3C4C")]
pub type Mi3c4c = crate::Reg<mi3c4c::Mi3c4cSpec>;
#[doc = "I3C4Dbg2 (Master)"]
pub mod mi3c4c;
#[doc = "I3C050 (rw) register accessor: I3C5Reg0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3c050::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c050::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3c050`] module"]
#[doc(alias = "I3C050")]
pub type I3c050 = crate::Reg<i3c050::I3c050Spec>;
#[doc = "I3C5Reg0"]
pub mod i3c050;
#[doc = "I3C054 (rw) register accessor: I3C5Reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`i3c054::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c054::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3c054`] module"]
#[doc(alias = "I3C054")]
pub type I3c054 = crate::Reg<i3c054::I3c054Spec>;
#[doc = "I3C5Reg1"]
pub mod i3c054;
#[doc = "MI3C58 (rw) register accessor: I3C5Dbg1 (Master)\n\nYou can [`read`](crate::Reg::read) this register and get [`mi3c58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mi3c58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi3c58`] module"]
#[doc(alias = "MI3C58")]
pub type Mi3c58 = crate::Reg<mi3c58::Mi3c58Spec>;
#[doc = "I3C5Dbg1 (Master)"]
pub mod mi3c58;
#[doc = "MI3C5C (rw) register accessor: I3C5Dbg2 (Master)\n\nYou can [`read`](crate::Reg::read) this register and get [`mi3c5c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mi3c5c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi3c5c`] module"]
#[doc(alias = "MI3C5C")]
pub type Mi3c5c = crate::Reg<mi3c5c::Mi3c5cSpec>;
#[doc = "I3C5Dbg2 (Master)"]
pub mod mi3c5c;
#[doc = "I3C060 (rw) register accessor: I3C6Reg0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3c060::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c060::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3c060`] module"]
#[doc(alias = "I3C060")]
pub type I3c060 = crate::Reg<i3c060::I3c060Spec>;
#[doc = "I3C6Reg0"]
pub mod i3c060;
#[doc = "I3C064 (rw) register accessor: I3C6Reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`i3c064::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c064::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3c064`] module"]
#[doc(alias = "I3C064")]
pub type I3c064 = crate::Reg<i3c064::I3c064Spec>;
#[doc = "I3C6Reg1"]
pub mod i3c064;
#[doc = "MI3C68 (rw) register accessor: I3C6Dbg1 (Master)\n\nYou can [`read`](crate::Reg::read) this register and get [`mi3c68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mi3c68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi3c68`] module"]
#[doc(alias = "MI3C68")]
pub type Mi3c68 = crate::Reg<mi3c68::Mi3c68Spec>;
#[doc = "I3C6Dbg1 (Master)"]
pub mod mi3c68;
#[doc = "MI3C6C (rw) register accessor: I3C6Dbg2 (Master)\n\nYou can [`read`](crate::Reg::read) this register and get [`mi3c6c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mi3c6c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mi3c6c`] module"]
#[doc(alias = "MI3C6C")]
pub type Mi3c6c = crate::Reg<mi3c6c::Mi3c6cSpec>;
#[doc = "I3C6Dbg2 (Master)"]
pub mod mi3c6c;
